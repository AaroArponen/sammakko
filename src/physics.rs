use na::Vector2;
use specs::{Component};

#[derive(Component)]
pub struct Position(Vector2<f64>);