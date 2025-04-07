use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount: f32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ResourceType {
    Food,
    Water,
    Minerals,
}

#[derive(Component, Clone, Copy)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ObstacleType {
    Rock,
    Root,
    Void,
}
