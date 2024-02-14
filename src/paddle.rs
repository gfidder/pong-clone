use super::Collider;
use bevy::prelude::*;

const LEFT_PADDLE: f32 = -250.0;
const RIGHT_PADDLE: f32 = 250.0;
pub const PADDLE_PADDING: f32 = 1.0;

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
}

impl PaddleBundle {
    pub fn new(paddle_location: PaddleLocation) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: paddle_location.position().extend(0.0),
                    scale: Vec3::new(5.0, 100.0, 1.0),
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
        }
    }
}
