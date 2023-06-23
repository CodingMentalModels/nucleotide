use bevy::prelude::*;
use petgraph::graph::UnGraph;

pub struct Map {
    adjacency_graph: AdjacencyGraph,
}

pub struct AdjacencyGraph(UnGraph<Room, ()>);

pub struct Room {
    rect: Rect,
}

pub enum MapTile {
    Floor,
    Wall,
}
