use bevy::prelude::*;

#[derive(Component)]
pub struct Colony {
    pub position: Vec2,
}

#[derive(Component)]
pub struct ColonyMember {
    pub colony_id: Entity,
}

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_colony);
    }
}

fn spawn_initial_colony(mut commands: Commands) {
    // Spawn the colony at the center with a blue sprite
    let colony_id = commands
        .spawn((
            Colony {
                position: Vec2::ZERO,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(10.0, 10.0)), // 2x the ant size (5x5)
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 5.0), // Z=5 to be above terrain but below ants
                ..default()
            },
        ))
        .id();

    println!("Colony spawned at (0,0) with id {:?}", colony_id);
}
