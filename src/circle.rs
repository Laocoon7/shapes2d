use std::fmt::Display;

use glam::Vec2;

pub struct Circle {
    center: Vec2,
    radius: f32,
}

// ##########
// Constructors
// ##########
impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }
    pub fn new_diameter(center: Vec2, diameter: f32) -> Self {
        Self::new(center, diameter * 0.5)
    }
}

// ##########
// Getters/Setters
// ##########
impl Circle {
    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_diameter(&mut self, diameter: f32) {
        self.radius = diameter * 0.5;
    }
}

// ##########
// Default impl
// ##########
impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Vec2::ZERO,
            radius: 1.,
        }
    }
}

// ##########
// Display impl
// ##########
impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle {{ center: {}, radius: {} }}", self.center(), self.radius())
    }
}