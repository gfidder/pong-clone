use bevy::{
    // sprite::collide_aabb::{collide, Collision},
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use paddle::{Paddle, PaddleBundle, PaddleLocation, PADDLE_HEIGHT, PADDLE_PADDING};
use std::time::Duration;

mod paddle;

const WALL_TOP: f32 = 300.0;
const WALL_BOTTOM: f32 = -300.0;
const WALL_LEFT: f32 = -450.0;
const WALL_RIGHT: f32 = 450.0;
const WALL_THICKNESS: f32 = 10.0;

const AI_SPEED: f32 = 200.0;

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

#[derive(Component)]
struct Wall;

#[derive(Component)]
enum ScoreType {
    Player,
    Computer,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Resource, Default)]
struct Scoreboard {
    player_score: u8,
    computer_score: u8,
}

#[derive(Component)]
struct ScoreboardUi;

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
                    color: Color::GRAY,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider,
            wall: Wall,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong Clone".into(),
                resolution: (900., 600.).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(5)))
        .insert_resource(Scoreboard::default())
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
        .add_systems(Update, update_scoreboard)
        // .add_systems(FixedUpdate, apply_velocity)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("PixelOperator.ttf");

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddles
    commands.spawn((PaddleBundle::new(PaddleLocation::Left), Player));
    commands.spawn((PaddleBundle::new(PaddleLocation::Right), CpuPlayer));

    // Ball
    commands.spawn((
        Ball,
        Velocity(Vec2::new(200.0, 200.0)),
        Collider,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    // Scoreboard
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ScoreboardUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Score: ",
                        TextStyle {
                            font_size: 40.0,
                            font: font_handle.clone(),
                            ..Default::default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: 40.0,
                        font: font_handle.clone(),
                        ..Default::default()
                    }),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.0),
                    left: Val::Percent(5.0),
                    ..Default::default()
                }),
                ScoreType::Player,
                ScoreboardUi,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Score: ",
                        TextStyle {
                            font_size: 40.0,
                            font: font_handle.clone(),
                            ..Default::default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font_size: 40.0,
                        font: font_handle.clone(),
                        color: Color::RED,
                    }),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.0),
                    right: Val::Percent(5.0),
                    ..Default::default()
                }),
                ScoreType::Computer,
                ScoreboardUi,
            ));
        });

    spawn_walls(&mut commands);
}

fn spawn_walls(commands: &mut Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}

fn player_paddle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_velocity) = query.single_mut();
    let paddle_velocity: f32 = 500.0;
    let mut direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
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

    let upper_bound = WALL_TOP - WALL_THICKNESS / 2.0 - PADDLE_HEIGHT / 2.0 - PADDLE_PADDING;
    let lower_bound = WALL_BOTTOM + WALL_THICKNESS / 2.0 + PADDLE_HEIGHT / 2.0 + PADDLE_PADDING;

    player_transform.translation.y = new_transform.clamp(lower_bound, upper_bound);
    player_velocity.y = direction * paddle_velocity;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<(&mut Text, &ScoreType), With<ScoreboardUi>>,
) {
    for (mut text, score_type) in query.iter_mut() {
        match score_type {
            ScoreType::Player => {
                text.sections[1].value = scoreboard.player_score.to_string();
            }
            ScoreType::Computer => {
                text.sections[1].value = scoreboard.computer_score.to_string();
            }
        }
    }
}

fn run_cpu_logic(
    ball_query: Query<(&Velocity, &Transform), (With<Ball>, Without<Paddle>)>,
    mut cpu_query: Query<(&mut Transform, &mut Velocity), (With<CpuPlayer>, Without<Ball>)>,
    time: Res<Time>,
) {
    let movement_margin = PADDLE_HEIGHT / 20.0;

    let (ball_velocity, ball_transform) = ball_query.single();
    let (mut cpu_transform, mut cpu_velocity) = cpu_query.single_mut();
    let mut direction = 0.0;

    if ball_velocity.x < 0.0 {
        // ball moving away, do nothing
        return;
    }

    if (ball_transform.translation.y - cpu_transform.translation.y).abs() > movement_margin {
        if ball_transform.translation.y > cpu_transform.translation.y {
            direction += 1.0;
        } else if ball_transform.translation.y < cpu_transform.translation.y {
            direction -= 1.0;
        }
    }

    let new_cpu_transform =
        cpu_transform.translation.y + direction * AI_SPEED * time.delta_seconds();

    let upper_bound = WALL_TOP - WALL_THICKNESS / 2.0 - PADDLE_HEIGHT / 2.0 - PADDLE_PADDING;
    let lower_bound = WALL_BOTTOM + WALL_THICKNESS / 2.0 + PADDLE_HEIGHT / 2.0 + PADDLE_PADDING;

    cpu_transform.translation.y = new_cpu_transform.clamp(lower_bound, upper_bound);
    cpu_velocity.y = direction * AI_SPEED;
}

fn check_collision(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<
        (&Transform, Option<&Wall>, Option<&Velocity>),
        (With<Collider>, Without<Ball>),
    >,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (collider, wall, velocity) in collider_query.iter() {
        let collision = collide_with_side(
            Aabb2d::new(ball_transform.translation.truncate(), ball_size / 2.),
            Aabb2d::new(
                collider.translation.truncate(),
                collider.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            if wall.is_some() {
                match collision {
                    Collision::Right => scoreboard.computer_score += 1,
                    Collision::Left => scoreboard.player_score += 1,
                    _ => {}
                }
            }

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
            }

            let mut new_y_ball_velocity = ball_velocity.y;

            if let Some(velocity) = velocity {
                // add 1/5 of the velocity to ball speed if present
                new_y_ball_velocity += velocity.y / 5.0;
            }

            if reflect_y {
                new_y_ball_velocity = -new_y_ball_velocity
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x
            }

            ball_velocity.y = new_y_ball_velocity;
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: Aabb2d, collider: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&collider) {
        return None;
    }

    let closest = collider.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
