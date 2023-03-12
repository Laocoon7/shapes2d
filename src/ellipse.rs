use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`Ellipse`] in 2d space
pub struct Ellipse {
    center: Vec2,
    radius_major: f32,
    radius_minor: f32,
}