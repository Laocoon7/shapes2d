use std::fmt::Display;

use glam::Vec2;

pub struct Triangle{
    coordinate1: Vec2,
    coordinate2: Vec2,
    coordinate3: Vec2,
}

// ##########
// Constructors
// ##########
impl Triangle {
    pub fn new(coordinate1: Vec2, coordinate2: Vec2, coordinate3: Vec2) -> Self {
        Self {
            coordinate1,
            coordinate2,
            coordinate3
        }
    }
}

// ##########
// Getters/Setters
// ##########
impl Triangle {
    pub fn coordinate1(&self) -> Vec2 {
        self.coordinate1
    }

    pub fn coordinate2(&self) -> Vec2 {
        self.coordinate2
    }

    pub fn coordinate3(&self) -> Vec2 {
        self.coordinate3
    }

    pub fn set_coordinate1(&mut self, coordinate1: Vec2) {
        self.coordinate1 = coordinate1;
    }

    pub fn set_coordinate2(&mut self, coordinate2: Vec2) {
        self.coordinate2 = coordinate2;
    }

    pub fn set_coordinate3(&mut self, coordinate3: Vec2) {
        self.coordinate3 = coordinate3;
    }
}

// ##########
// Attributes
// ##########
impl Triangle {}

// ##########
// Default impl
// ##########
impl Default for Triangle {
    fn default() -> Self {
        Self {
            coordinate1: Vec2::ONE,
            coordinate2: Vec2::ZERO,
            coordinate3: Vec2 { x: 1., y: 0. },
        }
    }
}

// ##########
// Display impl
// ##########
impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle {{ coordinate1: {}, coordinate2: {}, coordinate3: {} }}", self.coordinate1(), self.coordinate2(), self.coordinate3())
    }
}