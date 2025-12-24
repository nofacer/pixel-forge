use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct ColorData {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NodeData {
    pub label: String,
    pub color: Option<ColorData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String, // "colorNode" or "outputNode"
    pub data: NodeData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
}
