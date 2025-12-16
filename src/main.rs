use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, move_units))
        .run();
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.8, 0.3),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Velocity { x: 0.0, y: 0.0 },
        Player,
    ));
}


fn move_units(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        velocity.x = 0.0;
        velocity.y = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            velocity.x -= 200.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            velocity.x += 200.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            velocity.y += 200.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            velocity.y -= 200.0;
        }
    }
}

