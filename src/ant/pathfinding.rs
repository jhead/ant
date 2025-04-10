use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const GRID_SIZE: f32 = 8.0; // Same as TILE_SIZE
const BASE_DIG_COST: i32 = 10; // Base cost for digging
const MAX_DIG_DISTANCE: f32 = 50.0; // Maximum distance to consider direct digging

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn from_vec2(pos: Vec2) -> Self {
        GridPos {
            x: (pos.x / GRID_SIZE).round() as i32,
            y: (pos.y / GRID_SIZE).round() as i32,
        }
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32 * GRID_SIZE, self.y as f32 * GRID_SIZE)
    }

    pub fn distance(&self, other: &GridPos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    pos: GridPos,
    f_cost: i32,
    g_cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Find the nearest accessible point to a target
pub fn find_nearest_accessible_point(
    start: Vec2,
    target: Vec2,
    solid_tiles: &[Vec2],
) -> Option<Vec2> {
    let start_pos = GridPos::from_vec2(start);
    let target_pos = GridPos::from_vec2(target);
    let direct_distance = start_pos.distance(&target_pos);

    // If target is very close, prefer digging directly
    if direct_distance * GRID_SIZE <= MAX_DIG_DISTANCE {
        return Some(target);
    }

    // Convert solid tiles to grid positions
    let obstacles: HashSet<GridPos> = solid_tiles
        .iter()
        .map(|&pos| GridPos::from_vec2(pos))
        .collect();

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_costs = HashMap::new();
    let mut nearest_point = None;
    let mut min_distance = f32::MAX;

    // Initialize start node
    let start_node = Node {
        pos: start_pos,
        f_cost: 0,
        g_cost: 0,
    };
    open_set.push(start_node);
    g_costs.insert(start_pos, 0);

    while let Some(current) = open_set.pop() {
        // Check if this is the nearest point we've found so far
        let distance_to_target = current.pos.distance(&target_pos);
        if distance_to_target < min_distance {
            min_distance = distance_to_target;
            nearest_point = Some(current.pos.to_vec2());
        }

        // If we're close enough to the target, we can stop
        if distance_to_target < 2.0 {
            break;
        }

        closed_set.insert(current.pos);

        // Check neighbors
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor_pos = GridPos {
                    x: current.pos.x + dx,
                    y: current.pos.y + dy,
                };

                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                // Calculate movement cost - diagonal movement costs more
                let movement_cost = if dx.abs() == 1 && dy.abs() == 1 {
                    14 // Approximately sqrt(2) * 10
                } else {
                    10 // Base movement cost
                };

                // Calculate digging cost based on distance to target
                let total_cost = if obstacles.contains(&neighbor_pos) {
                    let distance_factor = (direct_distance * GRID_SIZE / MAX_DIG_DISTANCE).min(1.0);
                    let dig_cost = (BASE_DIG_COST as f32 * (1.0 + distance_factor)) as i32;
                    movement_cost + dig_cost
                } else {
                    movement_cost
                };

                let new_g_cost = g_costs[&current.pos] + total_cost;
                if !g_costs.contains_key(&neighbor_pos) || new_g_cost < g_costs[&neighbor_pos] {
                    came_from.insert(neighbor_pos, current.pos);
                    g_costs.insert(neighbor_pos, new_g_cost);
                    let h_cost = (neighbor_pos.distance(&target_pos) * 10.0) as i32;
                    let f_cost = new_g_cost + h_cost;

                    let neighbor = Node {
                        pos: neighbor_pos,
                        f_cost,
                        g_cost: new_g_cost,
                    };
                    open_set.push(neighbor);
                }
            }
        }
    }

    nearest_point
}

pub fn find_path(start: Vec2, end: Vec2, solid_tiles: &[Vec2]) -> Option<Vec<Vec2>> {
    let start_pos = GridPos::from_vec2(start);
    let end_pos = GridPos::from_vec2(end);
    let direct_distance = start_pos.distance(&end_pos);

    // If target is very close, prefer digging directly
    if direct_distance * GRID_SIZE <= MAX_DIG_DISTANCE {
        return Some(vec![start, end]);
    }

    // Convert solid tiles to grid positions
    let obstacles: HashSet<GridPos> = solid_tiles
        .iter()
        .map(|&pos| GridPos::from_vec2(pos))
        .collect();

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_costs = HashMap::new();

    // Initialize start node
    let start_node = Node {
        pos: start_pos,
        f_cost: 0,
        g_cost: 0,
    };
    open_set.push(start_node);
    g_costs.insert(start_pos, 0);

    while let Some(current) = open_set.pop() {
        if current.pos == end_pos {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current_pos = end_pos;
            while current_pos != start_pos {
                path.push(current_pos.to_vec2());
                current_pos = came_from[&current_pos];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        closed_set.insert(current.pos);

        // Check neighbors
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor_pos = GridPos {
                    x: current.pos.x + dx,
                    y: current.pos.y + dy,
                };

                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                // Calculate movement cost - diagonal movement costs more
                let movement_cost = if dx.abs() == 1 && dy.abs() == 1 {
                    14 // Approximately sqrt(2) * 10
                } else {
                    10 // Base movement cost
                };

                // Calculate digging cost based on distance to target
                let total_cost = if obstacles.contains(&neighbor_pos) {
                    let distance_factor = (direct_distance * GRID_SIZE / MAX_DIG_DISTANCE).min(1.0);
                    let dig_cost = (BASE_DIG_COST as f32 * (1.0 + distance_factor)) as i32;
                    movement_cost + dig_cost
                } else {
                    movement_cost
                };

                let new_g_cost = g_costs[&current.pos] + total_cost;
                if !g_costs.contains_key(&neighbor_pos) || new_g_cost < g_costs[&neighbor_pos] {
                    came_from.insert(neighbor_pos, current.pos);
                    g_costs.insert(neighbor_pos, new_g_cost);
                    let h_cost = (neighbor_pos.distance(&end_pos) * 10.0) as i32;
                    let f_cost = new_g_cost + h_cost;

                    let neighbor = Node {
                        pos: neighbor_pos,
                        f_cost,
                        g_cost: new_g_cost,
                    };
                    open_set.push(neighbor);
                }
            }
        }
    }

    None // No path found
}
