use crate::ant::components::{Ant, WorkerState, ANT_SPEED, MAX_COLONY_DISTANCE};
use crate::ant::pathfinding::{find_nearest_accessible_point, find_path, GridPos};
use crate::colony::{Colony, ColonyMember};
use crate::terrain::{AirTile, Tile, TileStore, TileUpdateEvent, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn ant_movement(
    mut query: Query<(&Transform, &mut Ant, &mut Velocity, &ColonyMember)>,
    colony_query: Query<&Colony>,
    _time: Res<Time>,
    mut tile_store: ResMut<TileStore>,
    mut tile_update_events: EventWriter<TileUpdateEvent>,
    tile_query: Query<(Entity, &Tile)>,
) {
    for (transform, mut ant, mut velocity, colony_member) in query.iter_mut() {
        if let Some(target_pos) = ant.target_position {
            let current_pos = transform.translation.truncate();

            // Get colony position and check distance
            if let Ok(colony) = colony_query.get(colony_member.colony_id) {
                let distance_to_colony = (colony.position - current_pos).length();
                if distance_to_colony > MAX_COLONY_DISTANCE {
                    println!(
                        "Target too far from colony ({} > {}), returning to colony",
                        distance_to_colony, MAX_COLONY_DISTANCE
                    );
                    ant.target_position = Some(colony.position);
                    continue;
                }
            }

            // Handle digging state first
            if let WorkerState::Digging(dig_target) = ant.worker_state {
                // Calculate direction to dig target
                let to_target = dig_target - current_pos;
                let dig_direction = to_target.normalize();

                // Find the next tile to dig in the direction of the target
                let next_pos = current_pos + dig_direction * TILE_SIZE;
                let grid_pos = GridPos::from_vec2(next_pos);

                // Check if we've reached the target
                if current_pos.distance(dig_target) < TILE_SIZE {
                    println!("Reached dig target at {:?}", dig_target);
                    velocity.linvel = Vec2::ZERO;

                    // Move to the next waypoint in the path
                    if let Some(path) = &ant.current_path {
                        if ant.current_path_index < path.len() {
                            ant.current_path_index += 1;
                            println!("Moving to next waypoint after digging");
                        }
                    }

                    ant.worker_state = WorkerState::SearchingForDigSite;
                    continue;
                }

                // Try to dig the tile
                if let Some(tile) = tile_store.get_tile_mut(&grid_pos.to_vec2()) {
                    if tile.tile_type.is_solid() {
                        println!("Digging tile at {:?}", grid_pos);
                        // Find the entity for this tile to update its visual
                        for (entity, tile) in tile_query.iter() {
                            if tile.position == grid_pos.to_vec2() {
                                tile_update_events.send(TileUpdateEvent {
                                    entity,
                                    new_type: Box::new(AirTile),
                                });
                                break;
                            }
                        }
                        // Update the tile in the store
                        tile.tile_type = Box::new(AirTile);

                        // Move towards the dug tile
                        velocity.linvel = dig_direction * ANT_SPEED * 0.5; // Move slower while digging
                    } else {
                        // If the tile is already dug, move towards it
                        velocity.linvel = dig_direction * ANT_SPEED;
                    }
                }
                continue;
            }

            // Check if we're close enough to the final destination
            let distance_to_target = (target_pos - current_pos).length();
            if distance_to_target < 5.0 {
                println!(
                    "Reached final destination at {:?}, distance: {}",
                    current_pos, distance_to_target
                );
                velocity.linvel = Vec2::ZERO;
                ant.target_position = None;
                ant.current_path = None;
                ant.current_path_index = 0;
                continue;
            }

            // If we don't have a path or need to recalculate
            if ant.current_path.is_none()
                || ant.current_path_index >= ant.current_path.as_ref().unwrap().len()
            {
                println!("Finding path to target at {:?}", target_pos);

                // First, find the nearest accessible point to the target
                let solid_tiles = tile_store.get_solid_tiles();
                if let Some(accessible_point) =
                    find_nearest_accessible_point(current_pos, target_pos, &solid_tiles)
                {
                    println!("Found nearest accessible point at {:?}", accessible_point);

                    // Find a path to the accessible point
                    if let Some(path) = find_path(current_pos, accessible_point, &solid_tiles) {
                        println!(
                            "Found path with {} waypoints to accessible point",
                            path.len()
                        );
                        ant.current_path = Some(path);
                        ant.current_path_index = 0;
                    } else {
                        println!("No path found to accessible point");
                        velocity.linvel = Vec2::ZERO;
                        ant.target_position = None;
                        continue;
                    }
                } else {
                    println!("No accessible point found near target");
                    velocity.linvel = Vec2::ZERO;
                    ant.target_position = None;
                    continue;
                }
            }

            // Follow the current path
            let path_len = ant.current_path.as_ref().map(|p| p.len()).unwrap_or(0);
            let current_index = ant.current_path_index;

            if current_index < path_len {
                let next_waypoint = ant.current_path.as_ref().unwrap()[current_index];
                let to_target = next_waypoint - current_pos;
                let distance = to_target.length();

                // Check if we need to dig to reach the next waypoint
                let next_pos = GridPos::from_vec2(next_waypoint);
                let solid_tiles = tile_store.get_solid_tiles();

                // Check for solid tiles in a small area around the next waypoint
                let needs_digging = solid_tiles.iter().any(|&pos| {
                    let tile_pos = GridPos::from_vec2(pos);
                    let dx = (tile_pos.x - next_pos.x).abs();
                    let dy = (tile_pos.y - next_pos.y).abs();
                    // Check if the tile is adjacent to or at the next waypoint
                    dx <= 1 && dy <= 1
                });

                if needs_digging {
                    println!("Need to dig to reach next waypoint at {:?}", next_waypoint);
                    ant.worker_state = WorkerState::Digging(next_waypoint);
                    velocity.linvel = Vec2::ZERO;
                    continue;
                }

                // Increased threshold for waypoint detection
                if distance < 2.0 {
                    println!(
                        "Reached waypoint {} at {:?}, distance: {}",
                        current_index, next_waypoint, distance
                    );

                    // Update path index
                    ant.current_path_index += 1;

                    // Only zero velocity if we're at the final waypoint
                    if ant.current_path_index >= path_len {
                        velocity.linvel = Vec2::ZERO;
                    }
                } else {
                    let direction = to_target.normalize();
                    // Apply a stronger impulse when starting to move
                    if velocity.linvel.length() < 10.0 {
                        velocity.linvel = direction * ANT_SPEED * 1.5;
                    } else {
                        velocity.linvel = direction * ANT_SPEED;
                    }
                }
            } else {
                // We've reached the end of our path
                println!("Reached final destination at {:?}", current_pos);
                velocity.linvel = Vec2::ZERO;
                ant.target_position = None;
                ant.current_path = None;
                ant.current_path_index = 0;
            }
        } else {
            velocity.linvel = Vec2::ZERO;
        }
    }
}
