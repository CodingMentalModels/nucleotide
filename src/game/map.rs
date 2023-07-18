use std::collections::HashMap;

use crate::game::constants::*;
use crate::game::resources::*;
use bevy::prelude::*;
use petgraph::algo::connected_components;
use petgraph::graph::UnGraph;
use rand::rngs::ThreadRng;
use rand::Rng;

type XCoordinate = u32;
type YCoordinate = u32;
type DoorPosition = Vec2;
type InternalGraph = UnGraph<Room, DoorPosition>;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            generate_map_system.in_schedule(OnEnter(NucleotideState::GeneratingMap)),
            update_map_system.in_schedule(OnEnter(NucleotideState::SelectingRoom)),
            despawn_map_system.in_schedule(OnExit(NucleotideState::SelectingRoom)),
        ));
    }
}

// Systems
fn generate_map_system(mut commands: Commands) {
    let config = MapGenerationConfig {
        n_rooms: N_ROOMS_PER_FLOOR,
        min_room_size: MIN_ROOM_SIZE,
    };
    commands.insert_resource(MapState::generate_map(config));
    commands.insert_resource(NextState(Some(NucleotideState::SelectingRoom)));
}

fn update_map_system(mut commands: Commands, map_state: Res<MapState>) {
    let room_rects = map_state.0.get_room_rects();
    let mut map_sprites = Vec::new();
    let to_center_x = -MAP_FLOOR_SIZE.0 / 2.;
    let to_center_y = -MAP_FLOOR_SIZE.1 / 2.;
    for rect in room_rects {
        let (front_rect, back_rect) = get_front_and_back_room_sprites(
            &mut commands,
            rect,
            Vec2::new(to_center_x, to_center_y),
        );
        map_sprites.push(front_rect);
        map_sprites.push(back_rect);
    }

    let door_rects = map_state.0.get_door_rects();
    for rect in door_rects {
        let door_sprite = get_rect_sprite(
            &mut commands,
            rect,
            1.0,
            Vec2::new(to_center_x, to_center_y),
            Color::WHITE,
        );
        map_sprites.push(door_sprite);
    }

    commands.insert_resource(MapSprites(map_sprites));
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

#[derive(Debug, Copy, Clone, Resource)]
pub struct MapGenerationConfig {
    n_rooms: usize,
    min_room_size: f32,
}

impl MapGenerationConfig {
    pub fn new(n_rooms: usize, min_room_size: f32) -> Self {
        Self {
            n_rooms,
            min_room_size,
        }
    }

    pub fn split(&self) -> Option<(MapGenerationConfig, MapGenerationConfig)> {
        if self.n_rooms <= 1 {
            return None;
        }
        let left_n_rooms = self.n_rooms / 2;
        let right_n_rooms = self.n_rooms - self.n_rooms / 2;
        return Some((
            Self::new(left_n_rooms, self.min_room_size),
            Self::new(right_n_rooms, self.min_room_size),
        ));
    }
}

#[derive(Debug, Clone, Resource)]
pub struct MapSprites(Vec<Entity>);

//End Resources

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
        let tree = Self::generate_room_tree(config);
        let graph = tree.generate_adjacency_graph();
        Map::new(graph)
    }

    pub fn get_room_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_room_rects()
    }

    pub fn get_door_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_door_rects()
    }

    fn generate_room_tree(config: MapGenerationConfig) -> RoomBinaryTreeNode {
        let mut rng = rand::thread_rng();

        let room = Room::new(Rect::from_corners(
            Vec2::ZERO,
            Vec2::new(MAP_FLOOR_SIZE.0, MAP_FLOOR_SIZE.1),
        ));

        let mut errors = Vec::new();
        for _i in 0..MAX_MAP_GENERATION_ITERATIONS {
            match RoomBinaryTreeNode::generate(&mut rng, config, room) {
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
        Self::from(Room::new(value))
    }
}

impl From<Room> for RoomBinaryTreeNode {
    fn from(value: Room) -> Self {
        Self::leaf(value)
    }
}

impl RoomBinaryTreeNode {
    pub fn generate_adjacency_graph(&self) -> AdjacencyGraph {
        let mut to_return = InternalGraph::new_undirected();
        let mut nodes_to_rooms: Vec<_> = self
            .get_leaf_rooms()
            .into_iter()
            .map(|room| (to_return.add_node(room), room))
            .collect();
        assert_eq!(nodes_to_rooms.len(), N_ROOMS_PER_FLOOR);

        while let Some((l_node, l_room)) = nodes_to_rooms.pop() {
            for (r_node, r_room) in nodes_to_rooms.iter() {
                match Room::get_potential_door_position(l_room, r_room.clone()) {
                    None => {}
                    Some(position) => {
                        to_return.update_edge(l_node, *r_node, position);
                    }
                }
            }
        }

        assert_eq!(connected_components(&to_return), 1);
        return AdjacencyGraph::new(to_return);
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

    pub fn get_room_rects(&self) -> Vec<Rect> {
        self.0
            .clone()
            .into_nodes_edges()
            .0
            .into_iter()
            .map(|node| node.weight.rect)
            .collect()
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Room {
    rect: Rect,
}

impl Default for Room {
    fn default() -> Self {
        Self::new(Rect::from_corners(
            Vec2::ZERO,
            Vec2::new(MAP_FLOOR_SIZE.0, MAP_FLOOR_SIZE.1),
        ))
    }
}

impl Room {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
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
        match is_vertical {
            true => {
                let left = Rect::from_corners(self.min(), Vec2::new(point.x, self.max().y));
                let right = Rect::from_corners(Vec2::new(point.x, self.min().y), self.max());
                (Room::new(left), Room::new(right))
            }
            false => {
                let bottom = Rect::from_corners(self.min(), Vec2::new(self.max().x, point.y));
                let top = Rect::from_corners(Vec2::new(self.min().x, point.y), self.max());
                (Room::new(bottom), Room::new(top))
            }
        }
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapGenerationError {
    RandomPointOverconstrained,
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
    rect: Rect,
    global_translation: Vec2,
) -> (Entity, Entity) {
    let blueprint_blue = Color::rgb(BLUEPRINT_BLUE.0, BLUEPRINT_BLUE.1, BLUEPRINT_BLUE.2);
    let back_sprite = get_rect_sprite(commands, rect, 0., global_translation, Color::WHITE);
    let front_rect = Rect::from_corners(
        rect.min + WALL_WIDTH * Vec2::ONE,
        rect.max - WALL_WIDTH * Vec2::ONE,
    );
    let front_sprite =
        get_rect_sprite(commands, front_rect, 1., global_translation, blueprint_blue);

    return (front_sprite, back_sprite);
}

fn get_rect_sprite(
    commands: &mut Commands,
    rect: Rect,
    z_value: f32,
    global_translation: Vec2,
    color: Color,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Some(rect.size()),
                ..default()
            },
            transform: Transform::from_translation(
                rect.center().extend(z_value) + global_translation.extend(0.),
            ),
            ..default()
        })
        .id()
}

fn round_to_nearest(f: f32, nearest: f32) -> f32 {
    (f / nearest).round() * nearest
}
//End Helper Functions

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_room_split() {
        let room = Room::new(Rect::from_corners(Vec2::new(1.0, 2.0), Vec2::new(3.0, 5.0)));
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
        let room = Room::new(Rect::from_corners(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)));
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
