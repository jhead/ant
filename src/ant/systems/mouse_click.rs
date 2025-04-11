use crate::ant::components::{Ant, WorkerState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn handle_mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&Transform, &mut Ant, &mut Velocity)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.get_single() {
                    if let Some(world_pos) =
                        camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        println!("Issuing new move command to position: {:?}", world_pos);
                        for (transform, mut ant, mut velocity) in query.iter_mut() {
                            // Immediately stop current movement
                            velocity.linvel = Vec2::ZERO;

                            // Clear existing path
                            ant.current_path = None;
                            ant.current_path_index = 0;

                            // Set new target
                            ant.target_position = Some(world_pos);
                            ant.worker_state = WorkerState::SearchingForDigSite;

                            println!(
                                "Interrupted ant at {:?}, setting new target",
                                transform.translation.truncate()
                            );
                        }
                    }
                }
            }
        }
    }
}
