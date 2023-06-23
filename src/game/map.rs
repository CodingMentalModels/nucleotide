use crate::game::constants::*;
use bevy::prelude::*;
use petgraph::graph::UnGraph;

type XCoordinate = u32;
type YCoordinate = u32;
type InternalGraph = UnGraph<Room, ()>;

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
