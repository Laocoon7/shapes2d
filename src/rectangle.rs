use std::fmt::Display;

use glam::Vec2;

pub struct Rectangle {
    min: Vec2,
    max: Vec2,
}
// ##########
// Constructors
// ##########
impl Rectangle {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self::new_coordinates(Vec2 { x: min_x, y: min_y }, Vec2 { x: max_x, y: max_y })
    }
    pub fn new_coordinates(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub fn new_dimensions(min: Vec2, width: f32, height: f32) -> Self {
        Self::new_coordinates(min, Vec2::new(min.x + width, min.y + height))
    }
}

// ##########
// Getters/Setters
// ##########
impl Rectangle {
    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn set_min(&mut self, min: Vec2) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: Vec2) {
        self.max = max;
    }
}

// ##########
// Attributes
// ##########
impl Rectangle {
    pub fn x(&self) -> f32 {
        self.min.x
    }

    pub fn min_x(&self) -> f32 {
        self.min.x
    }

    pub fn max_x(&self) -> f32 {
        self.max.x
    }

    pub fn y(&self) -> f32 {
        self.min.y
    }

    pub fn min_y(&self) -> f32 {
        self.min.y
    }

    pub fn max_y(&self) -> f32 {
        self.max.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {
            x: self.width(),
            y: self.height(),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.min
    }

    pub fn center(&self) -> Vec2 {
        Vec2 {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.max.x += x - self.min.x;
        self.min.x = x;
    }

    pub fn set_min_x(&mut self, x: f32) {
        self.min.x = x;
    }

    pub fn set_max_x(&mut self, x: f32) {
        self.max.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.max.y += y - self.min.y;
        self.min.y = y;
    }

    pub fn set_min_y(&mut self, y: f32) {
        self.min.y = y;
    }

    pub fn set_max_y(&mut self, y: f32) {
        self.max.y = y;
    }

    // Changes max
    pub fn set_width(&mut self, width: f32) {
        self.max.x = self.min.x + width;
    }

    // Changes max
    pub fn set_height(&mut self, height: f32) {
        self.max.y = self.min.y + height;
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.set_width(size.x);
        self.set_height(size.y);
    }

    // changes min and max
    pub fn set_position(&mut self, position: Vec2) {
        self.set_x(position.x);
        self.set_y(position.y);
    }

    // Changes min and max
    pub fn set_center(&mut self, center: Vec2) {
        let half_width = self.width() * 0.5;
        let half_height = self.height() * 0.5;
        self.set_position(Vec2 {
            x: center.x - half_width,
            y: center.y - half_height,
        });
    }
}

// ##########
// Default impl
// ##########
impl Default for Rectangle {
    fn default() -> Self {
        Self {
            min: Vec2::ZERO,
            max: Vec2::ONE,
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
            "Rectangle {{ MinX: {}, MinY: {}, MaxX: {}, MaxY: {} }}",
            self.min_x(),
            self.min_y(),
            self.max_x(),
            self.max_y(),
        )
    }
}
