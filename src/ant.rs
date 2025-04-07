use crate::colony::{Colony, ColonyMember};
use crate::terrain::{Tile, TILE_SIZE};
use bevy::prelude::*;
use rand::random;

const GRAVITY: f32 = 200.0;
const TERMINAL_VELOCITY: f32 = 200.0;
const MAX_COLONY_DISTANCE: f32 = 50.0;
const INVESTIGATION_COMPLETE_DISTANCE: f32 = 5.0; // Distance at which we consider target reached

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntState {
    Roam,
    Investigate(Vec2), // Includes target position
}

#[derive(Component)]
pub struct Ant {
    pub speed: f32,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    pub state: AntState,
}

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_initial_ant)
            .add_systems(Update, (ant_movement, handle_mouse_click));
    }
}

// Handle mouse clicks to set investigation targets
fn handle_mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ants: Query<&mut Ant>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.get_single() {
                    if let Some(world_pos) =
                        camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        println!("Setting investigation target to: {:?}", world_pos);
                        // Update all ants to investigate the clicked position
                        for mut ant in ants.iter_mut() {
                            ant.state = AntState::Investigate(world_pos);
                        }
                    }
                }
            }
        }
    }
}

fn spawn_initial_ant(mut commands: Commands, colony_query: Query<Entity, With<Colony>>) {
    // Get the first colony
    if let Ok(colony_id) = colony_query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            Ant {
                speed: 50.0,
                direction: Vec2::new(1.0, 0.0),
                velocity: Vec2::ZERO,
                on_ground: false,
                state: AntState::Roam,
            },
            ColonyMember { colony_id },
        ));

        println!("Ant spawned at (0,0) for colony {:?}", colony_id);
    } else {
        println!("Failed to spawn ant: no colony found");
    }
}

// Handle state-specific movement behavior
fn handle_roam_state(on_ground: bool, new_direction: &mut Vec2) {
    if on_ground && random::<f32>() < 0.02 {
        let angle = random::<f32>() * std::f32::consts::TAU;
        *new_direction = Vec2::new(angle.cos(), angle.sin());
    }
}

fn handle_investigate_state(current_pos: Vec2, target_pos: Vec2, new_direction: &mut Vec2) -> bool {
    // Move towards the target position
    let to_target = target_pos - current_pos;
    let distance_to_target = to_target.length();

    // Return true if we've reached the target
    if distance_to_target <= INVESTIGATION_COMPLETE_DISTANCE {
        return true;
    }

    // Otherwise, update direction and continue investigating
    *new_direction = to_target.normalize();
    false
}

fn ant_movement(
    time: Res<Time>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut Ant, &ColonyMember)>,
        Query<(&Transform, &Tile)>,
        Query<&Transform, With<Colony>>,
    )>,
) {
    let mut ant_positions = Vec::new();
    let mut ant_velocities = Vec::new();
    let mut ant_on_grounds = Vec::new();
    let mut ant_directions = Vec::new();
    let mut ant_states = Vec::new();

    // Get colony position
    let colony_pos = if let Ok(colony_transform) = query_set.p2().get_single() {
        colony_transform.translation.truncate()
    } else {
        Vec2::ZERO // Fallback to origin if no colony found
    };

    // First, gather all terrain positions
    let terrain_positions: Vec<_> = query_set
        .p1()
        .iter()
        .filter(|(_, tile)| tile.is_solid)
        .map(|(transform, _)| transform.translation.truncate())
        .collect();

    // Then process ant movement
    for (transform, ant, _colony_member) in query_set.p0().iter_mut() {
        let mut new_pos = transform.translation.truncate();
        let mut new_velocity = ant.velocity;
        let mut new_on_ground = false;
        let mut new_direction = ant.direction;
        let mut new_state = ant.state;

        // Check distance from colony (this overrides any state behavior)
        let distance_to_colony = (new_pos - colony_pos).length();
        if distance_to_colony > MAX_COLONY_DISTANCE {
            let to_colony = (colony_pos - new_pos).normalize();
            new_direction = to_colony;
            println!(
                "Ant too far from colony ({:.1}), returning",
                distance_to_colony
            );
        } else {
            // Apply state-specific behavior
            match ant.state {
                AntState::Roam => handle_roam_state(ant.on_ground, &mut new_direction),
                AntState::Investigate(target_pos) => {
                    if handle_investigate_state(new_pos, target_pos, &mut new_direction) {
                        // Target reached, return to roaming
                        new_state = AntState::Roam;
                        println!("Investigation complete, returning to roam state");
                    }
                }
            }
        }

        // Apply gravity if not on ground
        if !ant.on_ground {
            new_velocity.y -= GRAVITY * time.delta_seconds();
            new_velocity.y = new_velocity.y.max(-TERMINAL_VELOCITY);
        }

        // Calculate movement
        let mut movement = Vec2::ZERO;
        movement.x = new_direction.x * ant.speed * time.delta_seconds();
        movement += new_velocity * time.delta_seconds();

        // Apply movement
        new_pos += movement;

        let ant_size = 5.0;
        let half_size = ant_size / 2.0;
        let half_tile = TILE_SIZE / 2.0;

        // Check collision with terrain
        for &terrain_pos in &terrain_positions {
            if new_pos.x + half_size > terrain_pos.x - half_tile
                && new_pos.x - half_size < terrain_pos.x + half_tile
                && new_pos.y + half_size > terrain_pos.y - half_tile
                && new_pos.y - half_size < terrain_pos.y + half_tile
            {
                // Collision response
                if movement.y < 0.0 && transform.translation.y > terrain_pos.y {
                    // Landing on top of tile
                    new_pos.y = terrain_pos.y + half_tile + half_size;
                    new_velocity.y = 0.0;
                    new_on_ground = true;
                } else if movement.y > 0.0 && transform.translation.y < terrain_pos.y {
                    // Hitting bottom of tile
                    new_pos.y = terrain_pos.y - half_tile - half_size;
                    new_velocity.y = 0.0;
                } else if movement.x != 0.0 {
                    // Horizontal collision
                    if transform.translation.x < terrain_pos.x {
                        new_pos.x = terrain_pos.x - half_tile - half_size;
                    } else {
                        new_pos.x = terrain_pos.x + half_tile + half_size;
                    }
                    new_direction.x *= -1.0; // Reverse horizontal direction
                }
            }
        }

        ant_positions.push(new_pos);
        ant_velocities.push(new_velocity);
        ant_on_grounds.push(new_on_ground);
        ant_directions.push(new_direction);
        ant_states.push(new_state);
    }

    // Finally, update all ant states
    let mut ant_query = query_set.p0();
    for i in 0..ant_positions.len() {
        if let Some((mut transform, mut ant, _)) = ant_query.iter_mut().nth(i) {
            transform.translation.x = ant_positions[i].x;
            transform.translation.y = ant_positions[i].y;
            ant.velocity = ant_velocities[i];
            ant.on_ground = ant_on_grounds[i];
            ant.direction = ant_directions[i];
            ant.state = ant_states[i];
        }
    }
}
