use std::fmt::Display;

use glam::Vec2;

pub struct Rectangle {
    bottom_left: Vec2,
    top_right: Vec2,
}
// ##########
// Constructors
// ##########
impl Rectangle {
    pub fn new(bottom: f32, left: f32, top: f32, right: f32) -> Self {
        Self::new_coordinates(Vec2::new(left, bottom), Vec2::new(right, top))
    }
    pub fn new_coordinates(bottom_left: Vec2, top_right: Vec2) -> Self {
        Self {
            bottom_left,
            top_right,
        }
    }
    pub fn new_dimensions(bottom_left: Vec2, width: f32, height: f32) -> Self {
        Self::new_coordinates(
            bottom_left,
            Vec2::new(bottom_left.x + width, bottom_left.y + height),
        )
    }
}

// ##########
// Getters/Setters
// ##########
impl Rectangle {
    pub fn bottom(&self) -> f32 {
        self.bottom_left.y
    }
    pub fn left(&self) -> f32 {
        self.bottom_left.x
    }
    pub fn top(&self) -> f32 {
        self.top_right.y
    }
    pub fn right(&self) -> f32 {
        self.top_right.x
    }

    pub fn set_bottom(&mut self, y: f32) {
        self.bottom_left.y = y;
    }
    pub fn set_left(&mut self, x: f32) {
        self.bottom_left.x = x;
    }
    pub fn set_top(&mut self, y: f32) {
        self.top_right.y = y;
    }
    pub fn set_right(&mut self, x: f32) {
        self.top_right.x = x;
    }
}

// ##########
// Attributes
// ##########
impl Rectangle {
    pub fn center(&self) -> Vec2 {
        Vec2 {
            x: (self.left() + self.right()) * 0.5,
            y: (self.bottom() + self.top()) * 0.5,
        }
    }
}

// ##########
// Default impl
// ##########
impl Default for Rectangle {
    fn default() -> Self {
        Self {
            bottom_left: Vec2::ZERO,
            top_right: Vec2::ONE,
        }
    }
}

// ##########
// Display impl
// ##########
impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {{ bottom: {}, left: {}, top: {}, right: {} }}",
            self.bottom(),
            self.left(),
            self.top(),
            self.right()
        )
    }
}
