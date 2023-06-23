use crate::game::constants::*;
use crate::game::resources::*;
use bevy::prelude::*;
use petgraph::graph::UnGraph;

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
    commands.insert_resource(MapState::default());
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
                transform: Transform::from_translation(rect.center().extend(0.)),
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

    pub fn get_rects(&self) -> Vec<Rect> {
        self.adjacency_graph.get_rects()
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
            Vec2::new(MAP_ROOM_SIZE.0, MAP_ROOM_SIZE.1),
        ))
    }
}

impl Room {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WallSegment {
    Vertical(XCoordinate, (YCoordinate, YCoordinate)),
    Horizontal((XCoordinate, XCoordinate), YCoordinate),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MapTile {
    Floor,
    Wall,
}

// End Helper Structs

//Helper Functions

//End Helper Functions
