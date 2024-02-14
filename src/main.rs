use bevy::prelude::*;
use paddle::{Paddle, PaddleBundle, PaddleLocation, PADDLE_PADDING};
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

#[derive(Component)]
struct Ball;

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
        .add_systems(Update, (player_paddle_movement, run_cpu_logic))
        .add_systems(FixedUpdate, apply_velocity)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddles
    commands.spawn((PaddleBundle::new(PaddleLocation::Left), Player));
    commands.spawn((PaddleBundle::new(PaddleLocation::Right), CpuPlayer));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(0.0, 0.0)),
        Collider,
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));

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

fn player_paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let paddle_velocity: f32 = 500.0;
    let mut direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_released(KeyCode::Down) {
    //     player_velocity.y += 500.0
    // }

    // if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Down) {
    //     player_velocity.y -= 500.0
    // }

    let new_transform =
        player_transform.translation.y + direction * paddle_velocity * time.delta_seconds();

    let upper_bound = WALL_TOP - WALL_THICKNESS / 2.0 - 100.0 / 2.0 - PADDLE_PADDING;
    let lower_bound = WALL_BOTTOM + WALL_THICKNESS / 2.0 + 100.0 / 2.0 + PADDLE_PADDING;

    player_transform.translation.y = new_transform.clamp(lower_bound, upper_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), Without<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn run_cpu_logic(
    ball_query: Query<(&Velocity, &Transform), With<Ball>>,
    mut cpu_query: Query<&mut Transform, (With<CpuPlayer>, Without<Player>)>,
    time: Res<Time>,
) {
    let (ball_velocity, ball_transform) = ball_query.single();
    let mut cpu_transform = cpu_query.single_mut();
    let mut direction = 0.0;

    if ball_velocity.x < 0.0 {
        // ball moving away, do nothing
        return;
    }

    if ball_transform.translation.y > cpu_transform.translation.y {
        direction += 1.0;
    } else if ball_transform.translation.y < cpu_transform.translation.y {
        direction -= 1.0;
    }
}

// fn check_collision(mut paddle_query)
