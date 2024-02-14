use bevy::prelude::*;
use paddle::{Paddle, PaddleBundle, PaddleLocation};
use std::time::Duration;

mod paddle;

const WALL_TOP: f32 = 300.0;
const WALL_BOTTOM: f32 = -300.0;
const WALL_LEFT: f32 = -450.0;
const WALL_RIGHT: f32 = 450.0;
const WALL_THICKNESS: f32 = 10.0;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct CpuPlayer;

#[derive(Component)]
struct ColorText;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(WALL_LEFT, 0.0),
            WallLocation::Right => Vec2::new(WALL_RIGHT, 0.0),
            WallLocation::Top => Vec2::new(0.0, WALL_TOP),
            WallLocation::Bottom => Vec2::new(0.0, WALL_BOTTOM),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = WALL_TOP - WALL_BOTTOM;
        let arena_width = WALL_RIGHT - WALL_LEFT;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(10)))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .add_systems(FixedUpdate, apply_velocity)
        .run();
}

fn setup(mut commands: Commands) {
    info!("Hello Bevy!");

    commands.spawn(Camera2dBundle::default());

    commands.spawn((PaddleBundle::new(PaddleLocation::Left), Player));

    commands.spawn((PaddleBundle::new(PaddleLocation::Right), CpuPlayer));

    // commands.spawn((
    //     TextBundle::from_section(
    //         "hello\nbevy",
    //         TextStyle {
    //             font_size: 50.0,
    //             ..Default::default()
    //         },
    //     )
    //     .with_text_alignment(TextAlignment::Center)
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(5.0),
    //         left: Val::Px(5.0),
    //         ..Default::default()
    //     }),
    //     ColorText,
    // ));

    spawn_walls(&mut commands);
}

fn spawn_walls(commands: &mut Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // if keyboard_input.pressed(KeyCode::A) {
    //     info!("A is pressed");
    // }

    let mut player_velocity = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_released(KeyCode::Down) {
        player_velocity.y += 500.0
    }

    if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Down) {
        player_velocity.y -= 500.0
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// fn check_collision(mut paddle_query)
