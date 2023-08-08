use std::collections::HashMap;
use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_raycast::RaycastMesh;
use petgraph::algo::connected_components;
use petgraph::graph::NodeIndex;
use petgraph::graph::NodeIndices;
use petgraph::graph::UnGraph;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::game::constants::*;
use crate::game::input::MouseoverRaycastSet;
use crate::game::resources::*;

type XCoordinate = u32;
type YCoordinate = u32;
type DoorPosition = Vec2;
type InternalGraph = UnGraph<Room, DoorPosition>;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NucleotideState::GeneratingMap), generate_map_system)
            .add_systems(
                OnEnter(NucleotideState::SelectingRoom),
                initialize_map_system,
            )
            .add_systems(
                Update,
                update_map_system.run_if(in_state(NucleotideState::SelectingRoom)),
            )
            .add_systems(OnExit(NucleotideState::SelectingRoom), despawn_map_system);
    }
}

// Systems
fn generate_map_system(mut commands: Commands) {
    let config = MapGenerationConfig::new(
        N_ROOMS_PER_FLOOR,
        MIN_ROOM_SIZE,
        MapGenerationConfig::load_room_type_weights(),
    );
    commands.insert_resource(MapState::generate_map(config));
    commands.insert_resource(NextState(Some(NucleotideState::SelectingRoom)));
}

fn initialize_map_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_state: Res<MapState>,
) {
    let node_indices = map_state.0.get_node_indices();
    let mut map_sprites = Vec::new();
    let mut to_add_adjacencies = Vec::new();
    let to_center_x = -MAP_FLOOR_SIZE.0 / 2.;
    let to_center_y = -MAP_FLOOR_SIZE.1 / 2.;
    for node_index in node_indices {
        let room = map_state
            .0
            .get_room(node_index)
            .expect("Node indices are guaranteed to exist.");
        let rect = room.rect;
        let room_type = room.room_type;

        let to_center_adjustment = Vec2::new(to_center_x, to_center_y);
        let (front_rect, back_rect) = get_front_and_back_room_sprites(
            &mut commands,
            &mut meshes,
            &mut materials,
            room,
            to_center_adjustment,
            node_index,
        );

        map_sprites.push(front_rect);
        map_sprites.push(back_rect);

        to_add_adjacencies.push((node_index, back_rect));

        let room_type_rect = Rect::from_center_size(rect.center(), Vec2::ONE * ROOM_TYPE_RECT_SIZE);
        let room_type_sprite = get_rect_sprite(
            &mut commands,
            &mut meshes,
            &mut materials,
            room_type_rect,
            2.0,
            to_center_adjustment,
            room_type.to_color(),
        );

        map_sprites.push(room_type_sprite);

        let player_rect = Rect::from_center_size(
            map_state
                .0
                .get_player_location()
                .expect("The player should have been instantiated already."),
            Vec2::ONE * PLAYER_RECT_ON_MAP_SIZE,
        );
        let player_sprite = get_rect_sprite(
            &mut commands,
            &mut meshes,
            &mut materials,
            player_rect,
            1.5,
            to_center_adjustment,
            Color::BLACK,
        );

        map_sprites.push(player_sprite);
    }

    to_add_adjacencies.iter().for_each(|(node_index, sprite)| {
        let adjacent_nodes = map_state.0.get_adjacent_node_indices(*node_index);
        let adjacent_sprites = adjacent_nodes
            .into_iter()
            .map(|idx| {
                to_add_adjacencies
                    .iter()
                    .filter(|(n, _)| *n == idx)
                    .map(|(_, s)| *s)
                    .collect::<HashSet<_>>()
            })
            .flatten()
            .collect();
        commands
            .entity(*sprite)
            .insert(AdjacentRoomSprites(adjacent_sprites));
    });

    let door_rects = map_state.0.get_door_rects();
    for rect in door_rects {
        let door_sprite = get_rect_sprite(
            &mut commands,
            &mut meshes,
            &mut materials,
            rect,
            2.0,
            Vec2::new(to_center_x, to_center_y),
            Color::WHITE,
        );
        map_sprites.push(door_sprite);
    }

    commands.insert_resource(MapSprites(map_sprites));
}

fn update_map_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_sprite_query: Query<(Entity, &mut Transform), With<PlayerSpriteOnMap>>,
    hoverables_query: Query<
        (Entity, &RaycastMesh<MouseoverRaycastSet>, &Transform),
        Without<PlayerSpriteOnMap>,
    >,
    player_room_query: Query<&AdjacentRoomSprites, With<ContainsPlayerComponent>>,
) {
    let adjacent_rooms: HashSet<Entity> = player_room_query.single().0.clone();
    for entity in adjacent_rooms.iter() {
        commands
            .entity(*entity)
            .insert(materials.add(ColorMaterial::from(Color::GREEN)));
    }

    let maybe_hovered_room = hoverables_query
        .into_iter()
        .filter(|(entity, raycast, _)| {
            raycast.intersections().len() > 0 && adjacent_rooms.contains(entity)
        })
        .next();
    for (_, mut player_transform) in player_sprite_query.iter_mut() {
        match maybe_hovered_room {
            Some((_, _, transform)) => {
                info!("Moving player transform.");
                *player_transform = *transform;
            }
            None => {
                // Do nothing
            }
        }
    }
}

fn despawn_map_system(mut commands: Commands, map_sprites: Res<MapSprites>) {
    for sprite in map_sprites.0.iter() {
        commands.entity(*sprite).despawn_recursive();
    }
}

// End Systems

// Resources
#[derive(Debug, Clone, Resource)]
pub struct MapState(Map);

impl Default for MapState {
    fn default() -> Self {
        Self(Map::empty())
    }
}

impl MapState {
    fn generate_map(config: MapGenerationConfig) -> Self {
        Self(Map::generate(config))
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MapGenerationConfig {
    n_rooms: usize,
    min_room_size: f32,
    room_type_weights: HashMap<RoomType, f32>,
}

impl MapGenerationConfig {
    pub fn new(
        n_rooms: usize,
        min_room_size: f32,
        room_type_weights: HashMap<RoomType, f32>,
    ) -> Self {
        Self {
            n_rooms,
            min_room_size,
            room_type_weights,
        }
    }

    pub fn load_room_type_weights() -> HashMap<RoomType, f32> {
        vec![
            (RoomType::Empty, EMPTY_ROOM_WEIGHT),
            (RoomType::Combat, COMBAT_ROOM_WEIGHT),
        ]
        .into_iter()
        .collect()
    }
    pub fn split(&self) -> Option<(MapGenerationConfig, MapGenerationConfig)> {
        if self.n_rooms <= 1 {
            return None;
        }
        let left_n_rooms = self.n_rooms / 2;
        let right_n_rooms = self.n_rooms - self.n_rooms / 2;
        return Some((
            Self::new(
                left_n_rooms,
                self.min_room_size,
                self.room_type_weights.clone(),
            ),
            Self::new(
                right_n_rooms,
                self.min_room_size,
                self.room_type_weights.clone(),
            ),
        ));
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MapSprites(Vec<Entity>);

//End Resources

// Components

#[derive(Debug, Clone, Component)]
pub struct NodeIndexComponent(NodeIndex);

#[derive(Debug, Clone, Component)]
pub struct AdjacentRoomSprites(HashSet<Entity>);

#[derive(Debug, Clone, Component)]
pub struct PlayerSpriteOnMap;

#[derive(Debug, Clone, Component)]
pub struct ContainsPlayerComponent;

// End Components

// Helper Structs
#[derive(Debug, Clone)]
pub struct Map {
    adjacency_graph: AdjacencyGraph,
}

impl Map {
    pub fn empty() -> Self {
        Self::new(AdjacencyGraph::empty())
    }

    pub fn new(adjacency_graph: AdjacencyGraph) -> Self {
        Self { adjacency_graph }
    }

    pub fn generate(config: MapGenerationConfig) -> Self {
        let mut rng = rand::thread_rng();

        let tree = Self::generate_room_tree(config.clone(), &mut rng);
        let graph = tree.generate_adjacency_graph(config, &mut rng);
        Map::new(graph)
    }

    pub fn get_adjacent_node_indices(&self, node_index: NodeIndex) -> HashSet<NodeIndex> {
        self.adjacency_graph.get_adjacent_node_indices(node_index)
    }

    pub fn get_node_indices(&self) -> NodeIndices<u32> {
        self.adjacency_graph.get_node_indices()
    }

    pub fn get_room(&self, node_index: NodeIndex) -> Option<Room> {
        self.adjacency_graph.get_room(node_index)
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        self.adjacency_graph.get_rooms()
    }

    pub fn get_room_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_room_rects()
    }

    pub fn get_door_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_door_rects()
    }

    pub fn get_player_location(&self) -> Option<Vec2> {
        self.adjacency_graph.get_player_location()
    }

    fn generate_room_tree(config: MapGenerationConfig, rng: &mut ThreadRng) -> RoomBinaryTreeNode {
        let room = Room::empty(Rect::from_corners(
            Vec2::ZERO,
            Vec2::new(MAP_FLOOR_SIZE.0, MAP_FLOOR_SIZE.1),
        ));

        let mut errors = Vec::new();
        for _i in 0..MAX_MAP_GENERATION_ITERATIONS {
            match RoomBinaryTreeNode::generate(rng, config.clone(), room) {
                Ok(m) => return m,
                Err(e) => {
                    errors.push(format!("{:?}", e));
                }
            }
        }
        panic!(
            "Exceeded max map generation iterations:\n{}",
            errors.join("\n")
        );
    }
}

#[derive(Debug, Clone)]
pub struct RoomBinaryTreeNode {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    room: Room,
}

impl From<Rect> for RoomBinaryTreeNode {
    fn from(value: Rect) -> Self {
        Self::from(Room::empty(value))
    }
}

impl From<Room> for RoomBinaryTreeNode {
    fn from(value: Room) -> Self {
        Self::leaf(value)
    }
}

impl RoomBinaryTreeNode {
    pub fn generate_adjacency_graph(
        &self,
        config: MapGenerationConfig,
        rng: &mut ThreadRng,
    ) -> AdjacencyGraph {
        let mut to_return = InternalGraph::new_undirected();
        let mut nodes_to_rooms: Vec<_> = self
            .get_leaf_rooms()
            .into_iter()
            .map(|mut room| {
                room.update_room_type_from_weights(config.room_type_weights.clone(), rng);
                return room;
            })
            .map(|room| (to_return.add_node(room), room))
            .collect();
        assert_eq!(nodes_to_rooms.len(), N_ROOMS_PER_FLOOR);

        let mut n_visited = 0;
        while let Some((l_node, l_room)) = nodes_to_rooms.pop() {
            for (r_node, r_room) in nodes_to_rooms.iter() {
                n_visited += 1;
                match Room::get_potential_door_position(l_room, r_room.clone()) {
                    None => {}
                    Some(position) => {
                        to_return.update_edge(l_node, *r_node, position);
                    }
                }
            }
        }

        assert_eq!(
            n_visited,
            ((N_ROOMS_PER_FLOOR) * (N_ROOMS_PER_FLOOR - 1)) / 2
        );

        assert_eq!(connected_components(&to_return), 1);

        println!("n_edges: {}", to_return.edge_count());
        let mut to_return = AdjacencyGraph::new(to_return);

        to_return.designate_entrance(rng);
        to_return.designate_exit(rng);

        return to_return;
    }

    pub fn generate(
        rng: &mut ThreadRng,
        config: MapGenerationConfig,
        room: Room,
    ) -> Result<Self, MapGenerationError> {
        match config.split() {
            None => {
                return Ok(Self::from(room));
            }
            Some((left_config, right_config)) => {
                let (left_room, right_room) = Self::split(room, rng, config)?;

                let left = Self::generate(rng, left_config, left_room)?;
                let right = Self::generate(rng, right_config, right_room)?;
                return Ok(Self::new(left, right, room));
            }
        };
    }

    pub fn split(
        room: Room,
        rng: &mut ThreadRng,
        config: MapGenerationConfig,
    ) -> Result<(Room, Room), MapGenerationError> {
        let is_vertical: bool = room.height() < room.width();
        let point =
            room.random_point_constrained(rng, (config.min_room_size, config.min_room_size))?;

        Ok(room.split(point, is_vertical))
    }

    pub fn new(left: Self, right: Self, room: Room) -> Self {
        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            room,
        }
    }

    pub fn leaf(room: Room) -> Self {
        Self {
            left: None,
            right: None,
            room,
        }
    }

    pub fn len(&self) -> usize {
        1 + self.n_children()
    }

    pub fn n_children(&self) -> usize {
        match (&self.left, &self.right) {
            (Some(_), Some(_)) => 2,
            (None, None) => 0,
            _ => 1,
        }
    }

    pub fn get_leaf_rooms(&self) -> Vec<Room> {
        match self.get_children() {
            Some((left_children, right_children)) => {
                let mut to_return = vec![];
                to_return.append(&mut left_children.get_leaf_rooms());
                to_return.append(&mut right_children.get_leaf_rooms());
                return to_return;
            }
            None => return vec![self.room],
        };
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        let mut to_return = vec![self.room];
        match self.get_children() {
            Some((left_children, right_children)) => {
                to_return.append(&mut left_children.get_rooms());
                to_return.append(&mut right_children.get_rooms());
            }
            None => {}
        }
        return to_return;
    }

    pub fn get_children(&self) -> Option<(Self, Self)> {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => Some((*left.clone(), *right.clone())),
            (None, None) => None,
            _ => panic!("It should be impossible to instantiate an unbalanced tree."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdjacencyGraph(InternalGraph);

impl AdjacencyGraph {
    pub fn empty() -> Self {
        let mut graph = InternalGraph::new_undirected();
        graph.add_node(Room::default());
        Self::new(graph)
    }
    pub fn new(graph: InternalGraph) -> Self {
        Self(graph)
    }

    pub fn get_adjacent_node_indices(&self, node_index: NodeIndex) -> HashSet<NodeIndex> {
        self.0.neighbors(node_index).collect()
    }

    pub fn get_node_indices(&self) -> NodeIndices<u32> {
        self.0.node_indices()
    }

    pub fn get_room(&self, node_index: NodeIndex) -> Option<Room> {
        self.0.node_weight(node_index).map(|r| *r)
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        self.0
            .clone()
            .into_nodes_edges()
            .0
            .into_iter()
            .map(|node| node.weight)
            .collect()
    }

    pub fn get_room_rects(&self) -> Vec<Rect> {
        self.get_rooms().into_iter().map(|room| room.rect).collect()
    }

    pub fn get_door_rects(&self) -> Vec<Rect> {
        let door_size = Vec2::new(DOOR_SIZE.0, DOOR_SIZE.1);
        self.0
            .clone()
            .into_nodes_edges()
            .1
            .into_iter()
            .map(|edge| {
                Rect::from_corners(edge.weight - door_size / 2., edge.weight + door_size / 2.)
            })
            .collect()
    }

    pub fn get_player_location(&self) -> Option<Vec2> {
        self.0
            .node_weights()
            .find(|n| n.explored_type == ExploredType::CurrentlyExploring)
            .map(|room| room.rect.center())
    }

    pub fn designate_entrance(&mut self, rng: &mut ThreadRng) {
        self.designate_random_room(
            rng,
            RoomType::Entrance,
            ExploredType::CurrentlyExploring,
            HashSet::new(),
        );
    }

    pub fn designate_exit(&mut self, rng: &mut ThreadRng) {
        let blocklist = vec![RoomType::Entrance].into_iter().collect();
        self.designate_random_room(rng, RoomType::Exit, ExploredType::Unexplored, blocklist);
    }

    fn designate_random_room(
        &mut self,
        rng: &mut ThreadRng,
        room_type: RoomType,
        explored_type: ExploredType,
        blocklisted_room_types: HashSet<RoomType>,
    ) {
        let mut nodes: Vec<&mut Room> = self
            .0
            .node_weights_mut()
            .filter(|w| !blocklisted_room_types.contains(&w.room_type))
            .collect();
        let index = (0..nodes.len())
            .choose(rng)
            .expect("There should be at least one node in the graph.");
        nodes[index].room_type = room_type;
        nodes[index].explored_type = explored_type;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Room {
    room_type: RoomType,
    rect: Rect,
    explored_type: ExploredType,
}

impl Default for Room {
    fn default() -> Self {
        Self::empty(Rect::from_corners(
            Vec2::ZERO,
            Vec2::new(MAP_FLOOR_SIZE.0, MAP_FLOOR_SIZE.1),
        ))
    }
}

impl Room {
    pub fn new(room_type: RoomType, rect: Rect, explored_type: ExploredType) -> Self {
        Self {
            room_type,
            rect,
            explored_type,
        }
    }

    pub fn empty(rect: Rect) -> Self {
        Self::new(RoomType::Empty, rect, ExploredType::Unexplored)
    }

    pub fn get_potential_door_position(l_room: Self, r_room: Self) -> Option<DoorPosition> {
        let delta = 3.0 * WALL_WIDTH;

        let l_rect = l_room.rect;
        let r_rect = r_room.rect;

        Self::get_potential_door_position_from_shift(l_rect, r_rect, delta * Vec2::X)
            .or(Self::get_potential_door_position_from_shift(
                l_rect,
                r_rect,
                delta * Vec2::NEG_X,
            ))
            .or(Self::get_potential_door_position_from_shift(
                l_rect,
                r_rect,
                delta * Vec2::Y,
            ))
            .or(Self::get_potential_door_position_from_shift(
                l_rect,
                r_rect,
                delta * Vec2::NEG_Y,
            ))
    }

    fn get_potential_door_position_from_shift(
        l_rect: Rect,
        r_rect: Rect,
        shift: Vec2,
    ) -> Option<DoorPosition> {
        let l_shifted = shift_rect(l_rect, shift);
        let r_shifted = shift_rect(r_rect, -shift);
        get_potential_rect_intersection_center(l_shifted, r_shifted)
    }

    pub fn random_point_constrained(
        &self,
        rng: &mut ThreadRng,
        min_room_size: (f32, f32),
    ) -> Result<Vec2, MapGenerationError> {
        let min_room_size_is_overconstrained =
            (2.0 * min_room_size.0 > self.width()) || (2.0 * min_room_size.1 > self.height());
        if min_room_size_is_overconstrained {
            Err(MapGenerationError::RandomPointOverconstrained)
        } else {
            let min_x = self.rect.min.x + min_room_size.0;
            let min_y = self.rect.min.y + min_room_size.1;

            let max_x = self.rect.max.x - min_room_size.0;
            let max_y = self.rect.max.y - min_room_size.1;

            let delta_x: f32 = rng.gen_range(0.0..=max_x - min_x);
            let delta_y: f32 = rng.gen_range(0.0..=max_y - min_y);

            let x = min_x + round_to_nearest(delta_x, min_room_size.0);
            let y = min_y + round_to_nearest(delta_y, min_room_size.1);

            Ok(Vec2::new(x, y))
        }
    }

    pub fn random_point(&self, rng: &mut ThreadRng) -> Vec2 {
        let x: f32 = rng.gen_range(self.rect.min.x..self.rect.max.x);
        let y: f32 = rng.gen_range(self.rect.min.y..self.rect.max.y);
        Vec2::new(x, y)
    }

    pub fn split(&self, point: Vec2, is_vertical: bool) -> (Room, Room) {
        assert_eq!(
            self.explored_type,
            ExploredType::Unexplored,
            "Can't split up rooms if there's already been a player instantiated."
        );
        match is_vertical {
            true => {
                let left = Rect::from_corners(self.min(), Vec2::new(point.x, self.max().y));
                let right = Rect::from_corners(Vec2::new(point.x, self.min().y), self.max());
                (
                    Room::new(self.room_type, left, ExploredType::Unexplored),
                    Room::new(self.room_type, right, ExploredType::Unexplored),
                )
            }
            false => {
                let bottom = Rect::from_corners(self.min(), Vec2::new(self.max().x, point.y));
                let top = Rect::from_corners(Vec2::new(self.min().x, point.y), self.max());
                (
                    Room::new(self.room_type, bottom, ExploredType::Unexplored),
                    Room::new(self.room_type, top, ExploredType::Unexplored),
                )
            }
        }
    }

    pub fn update_room_type_from_weights(
        &mut self,
        weights: HashMap<RoomType, f32>,
        rng: &mut ThreadRng,
    ) {
        let types = weights.keys().cloned().collect::<Vec<RoomType>>();
        let index = WeightedIndex::new(weights.iter().map(|w| *w.1).collect::<Vec<f32>>())
            .expect("The weights should be valid from the config.");
        let room_type = types[index.sample(rng)];
        self.room_type = room_type;
    }

    pub fn min(&self) -> Vec2 {
        self.rect.min
    }

    pub fn max(&self) -> Vec2 {
        self.rect.max
    }

    pub fn width(&self) -> f32 {
        self.rect.width()
    }

    pub fn height(&self) -> f32 {
        self.rect.height()
    }

    pub fn contains_player(&self) -> bool {
        self.explored_type == ExploredType::CurrentlyExploring
    }

    pub fn get_color(&self) -> Color {
        let blueprint_blue = Color::rgb(BLUEPRINT_BLUE.0, BLUEPRINT_BLUE.1, BLUEPRINT_BLUE.2);
        let blueprint_gray = color_lerp(blueprint_blue, get_grayscale(blueprint_blue), 0.75);
        match self.explored_type {
            ExploredType::Unexplored => blueprint_gray,
            ExploredType::PreviouslyExplored => blueprint_blue,
            ExploredType::CurrentlyExploring => blueprint_blue,
            ExploredType::Adjacent => blueprint_gray,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapGenerationError {
    RandomPointOverconstrained,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RoomType {
    #[default]
    Empty,
    Entrance,
    Exit,
    Combat,
}

impl RoomType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Empty => "Empty",
            Self::Entrance => "Entrance",
            Self::Exit => "Exit",
            Self::Combat => "Combat",
        }
        .to_string()
    }

    pub fn to_color(&self) -> Color {
        match self {
            Self::Empty => Color::WHITE,
            Self::Entrance => Color::YELLOW,
            Self::Exit => Color::GREEN,
            Self::Combat => Color::RED,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExploredType {
    #[default]
    Unexplored,
    PreviouslyExplored,
    CurrentlyExploring,
    Adjacent,
}

// End Helper Structs

//Helper Functions
fn shift_rect(rect: Rect, delta: Vec2) -> Rect {
    Rect::from_corners(rect.min + delta, rect.max + delta)
}

fn get_potential_rect_intersection_center(l_rect: Rect, r_rect: Rect) -> Option<Vec2> {
    let overlap = l_rect.intersect(r_rect);
    if overlap.is_empty() {
        return None;
    } else {
        Some(overlap.center())
    }
}

fn get_front_and_back_room_sprites(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    room: Room,
    global_translation: Vec2,
    node_index: NodeIndex,
) -> (Entity, Entity) {
    let rect = room.rect;
    let background_color = room.get_color();
    let back_sprite = get_rect_sprite(
        commands,
        meshes,
        materials,
        rect,
        0.,
        global_translation,
        Color::WHITE,
    );
    commands
        .entity(back_sprite)
        .insert(NodeIndexComponent(node_index));
    let front_rect = Rect::from_corners(
        rect.min + WALL_WIDTH * Vec2::ONE,
        rect.max - WALL_WIDTH * Vec2::ONE,
    );
    let front_sprite = get_rect_sprite(
        commands,
        meshes,
        materials,
        front_rect,
        1.,
        global_translation,
        background_color,
    );
    commands
        .entity(front_sprite)
        .insert(NodeIndexComponent(node_index));

    if room.contains_player() {
        commands.entity(back_sprite).insert(ContainsPlayerComponent);
        commands
            .entity(front_sprite)
            .insert(ContainsPlayerComponent);
    }

    return (front_sprite, back_sprite);
}

fn get_rect_sprite(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    rect: Rect,
    z_value: f32,
    global_translation: Vec2,
    color: Color,
) -> Entity {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(rect.size()))).into(),
            transform: Transform::from_translation(
                rect.center().extend(z_value) + global_translation.extend(0.),
            ),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(RaycastMesh::<MouseoverRaycastSet>::default())
        .id()
}

fn round_to_nearest(f: f32, nearest: f32) -> f32 {
    (f / nearest).round() * nearest
}

fn get_grayscale(color: Color) -> Color {
    // ChatGPT formula for grayness
    let luminosity = 0.21 * color.r() + 0.72 * color.g() + 0.07 * color.b();

    return Color::rgb(luminosity, luminosity, luminosity);
}

fn color_lerp(left: Color, right: Color, t: f32) -> Color {
    assert!(t >= 0.0);
    assert!(t <= 1.0);

    Color::rgb(
        left.r() * (1. - t) + right.r() * t,
        left.g() * (1. - t) + right.g() * t,
        left.b() * (1. - t) + right.b() * t,
    )
}
//End Helper Functions

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_room_split() {
        let room = Room::empty(Rect::from_corners(Vec2::new(1.0, 2.0), Vec2::new(3.0, 5.0)));
        let (left, right) = room.split(Vec2::new(1.5, 3.0), true);
        assert!(left.min().distance(room.min()) <= 0.001);
        assert!(right.max().distance(room.max()) <= 0.001);

        assert!(left.max().distance(Vec2::new(1.5, 5.0)) <= 0.001);
        assert!(right.min().distance(Vec2::new(1.5, 2.0)) <= 0.001);

        let (bottom, top) = room.split(Vec2::new(1.5, 3.0), false);
        assert!(bottom.min().distance(room.min()) <= 0.001);
        assert!(top.max().distance(room.max()) <= 0.001);

        assert!(bottom.max().distance(Vec2::new(3.0, 3.0)) <= 0.001);
        assert!(top.min().distance(Vec2::new(1.0, 3.0)) <= 0.001);
    }

    #[test]
    fn test_random_point() {
        let room = Room::empty(Rect::from_corners(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)));
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let point = room.random_point(&mut rng);
            assert!(point.x >= room.min().x);
            assert!(point.x <= room.max().x);
            assert!(point.y >= room.min().y);
            assert!(point.y <= room.max().y);
        }
    }
}
