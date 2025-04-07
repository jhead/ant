use bevy::prelude::*;

pub const CAMERA_SPEED: f32 = 50.0;
pub const CAMERA_ACCELERATION: f32 = 200.0;
pub const CAMERA_DECELERATION: f32 = 300.0;
pub const CAMERA_ZOOM: f32 = 0.25;

#[derive(Component)]
pub struct CameraController {
    pub velocity: Vec2,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            projection: OrthographicProjection {
                scale: CAMERA_ZOOM,
                near: -1000.0,
                far: 1000.0,
                ..default()
            },
            ..default()
        },
        CameraController {
            velocity: Vec2::ZERO,
        },
    ));
}

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    if let Ok((mut camera_transform, mut controller)) = camera_query.get_single_mut() {
        let mut target_velocity = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            target_velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            target_velocity.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            target_velocity.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            target_velocity.y -= 1.0;
        }

        if target_velocity != Vec2::ZERO {
            target_velocity = target_velocity.normalize();
        }

        let dt = time.delta_seconds().min(0.016); // Cap delta time to prevent large jumps

        // Smoother deceleration
        let deceleration_factor = 1.0 - (CAMERA_DECELERATION * dt).min(0.95);
        controller.velocity *= deceleration_factor;

        if target_velocity != Vec2::ZERO {
            // Smoother acceleration with interpolation
            let target_speed = target_velocity * CAMERA_SPEED;
            let acceleration_factor = (CAMERA_ACCELERATION * dt).min(0.8);
            controller.velocity = controller.velocity.lerp(target_speed, acceleration_factor);

            // Smoother speed limiting
            let current_speed = controller.velocity.length();
            if current_speed > CAMERA_SPEED {
                controller.velocity = controller.velocity.normalize() * CAMERA_SPEED;
            }
        }

        // Apply movement with delta time capping
        camera_transform.translation +=
            Vec3::new(controller.velocity.x, controller.velocity.y, 0.0) * dt;
    }
}
