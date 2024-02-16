use super::{Collider, Velocity, WALL_LEFT, WALL_RIGHT};
use bevy::prelude::*;

const LEFT_PADDLE: f32 = -425.0;
const RIGHT_PADDLE: f32 = 425.0;
pub const PADDLE_PADDING: f32 = 1.0;
pub const PADDLE_HEIGHT: f32 = 175.0;

#[derive(Component)]
pub struct Paddle;

pub enum PaddleLocation {
    Left,
    Right,
}

impl PaddleLocation {
    fn position(&self) -> Vec2 {
        match self {
            PaddleLocation::Left => Vec2::new(LEFT_PADDLE, 0.0),
            PaddleLocation::Right => Vec2::new(RIGHT_PADDLE, 0.0),
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    collider: Collider,
    sprite: SpriteBundle,
    velocity: Velocity,
}

impl PaddleBundle {
    pub fn new(paddle_location: PaddleLocation) -> Self {
        // check paddle and wall math
        assert!(WALL_LEFT - LEFT_PADDLE < 0.0);
        assert!(WALL_RIGHT - RIGHT_PADDLE > 0.0);
        assert!(LEFT_PADDLE.abs() == RIGHT_PADDLE.abs());

        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: paddle_location.position().extend(0.0),
                    scale: Vec3::new(5.0, PADDLE_HEIGHT, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            },
            paddle: Paddle,
            collider: Collider,
            velocity: Velocity(Vec2::new(0.0, 0.0)),
        }
    }
}
