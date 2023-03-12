use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`Polygon`] in 2d space
pub struct Polygon {
    coordinates: Vec<Vec2>,
}
