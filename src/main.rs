use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
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
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(5)))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player_paddle_movement,
                run_cpu_logic,
                apply_velocity,
                check_collision,
            )
                .chain(),
        )
        // .add_systems(FixedUpdate, apply_velocity)
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
        Velocity(Vec2::new(400.0, 00.0)),
        Collider,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(15.0, 15.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn run_cpu_logic(
    ball_query: Query<(&Velocity, &Transform), (With<Ball>, Without<Paddle>)>,
    mut cpu_query: Query<&mut Transform, (With<CpuPlayer>, Without<Ball>)>,
    time: Res<Time>,
) {
    // debug this here: https://github.com/bevyengine/bevy/issues/2198

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

    let new_cpu_transform = cpu_transform.translation.y + direction * 100.0 * time.delta_seconds();

    let upper_bound = WALL_TOP - WALL_THICKNESS / 2.0 - 100.0 / 2.0 - PADDLE_PADDING;
    let lower_bound = WALL_BOTTOM + WALL_THICKNESS / 2.0 + 100.0 / 2.0 + PADDLE_PADDING;

    cpu_transform.translation.y = new_cpu_transform.clamp(lower_bound, upper_bound);
}

fn check_collision(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for collider in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            collider.translation,
            collider.scale.truncate(),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                _ => {}
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x
            }
        }
    }
}
