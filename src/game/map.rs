use std::collections::HashMap;

use crate::game::constants::*;
use crate::game::resources::*;
use bevy::prelude::*;
use petgraph::graph::UnGraph;
use rand::rngs::ThreadRng;
use rand::Rng;

type XCoordinate = u32;
type YCoordinate = u32;
type InternalGraph = UnGraph<Room, ()>;

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
    let rects = map_state.0.get_rects();
    let mut map_sprites = Vec::new();
    for rect in rects {
        let entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(rect.size()),
                    ..default()
                },
                transform: Transform::from_translation(rect.center().extend(0.))
                    .with_scale(0.95 * Vec3::ONE),
                ..default()
            })
            .id();
        map_sprites.push(entity);
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

#[derive(Debug, Clone, Resource)]
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

    pub fn get_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_rects()
    }

    fn generate_room_tree(config: MapGenerationConfig) -> RoomBinaryTreeNode {
        let mut rng = rand::thread_rng();

        let room = Room::new(Rect::from_corners(
            Vec2::ZERO,
            Vec2::new(MAP_FLOOR_SIZE.0, MAP_FLOOR_SIZE.1),
        ));
        return RoomBinaryTreeNode::generate(&mut rng, config, room);
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
        Self::from(value)
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
        let nodes: Vec<_> = self
            .get_leaf_rooms()
            .into_iter()
            .map(|node| to_return.add_node(node))
            .collect();
        assert_eq!(nodes.len(), N_ROOMS_PER_FLOOR);
        // TODO: Generate the edges
        return AdjacencyGraph::new(to_return);
    }

    pub fn generate(rng: &mut ThreadRng, config: MapGenerationConfig, room: Room) -> Self {
        match config.split() {
            None => {
                return Self::from(room);
            }
            Some((left_config, right_config)) => {
                let (left_room, right_room) = Self::split(room, rng, config);

                let left = Self::generate(rng, left_config, left_room);
                let right = Self::generate(rng, right_config, right_room);
                return Self::new(left, right, room);
            }
        };
    }

    pub fn split(room: Room, rng: &mut ThreadRng, config: MapGenerationConfig) -> (Room, Room) {
        let point = room.random_point(rng);
        let is_vertical: bool = rng.gen_bool(MAP_WALLS_VERTICAL_PROPORTION);

        room.split(point, is_vertical)
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

    pub fn get_rects(&self) -> Vec<Rect> {
        self.0
            .clone()
            .into_nodes_edges()
            .0
            .into_iter()
            .map(|node| node.weight.rect)
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
}

// End Helper Structs

//Helper Functions

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
