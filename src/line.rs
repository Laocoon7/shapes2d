use std::fmt::Display;

use glam::Vec2;

/// Represents a [`Line`] in 2d space
pub struct Line {
    origin: Vec2,
    end: Vec2,
}

// ##########
// Constructors
// ##########
impl Line {
    /// Creates a new [`Line`] starting at the `origin` and ending at the `end`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// 
    /// assert_eq!(line.direction(), Vec2::ONE);
    /// ```
    pub fn new(origin: Vec2, end: Vec2) -> Self {
        Self {
            origin,
            end
        }
    }

    /// Creates a new [`Line`] starting at the `origin` and ending in a `direction`  at a `distance`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line = Line::new_direction(Vec2::ZERO, Vec2 { x: 1., y: 0. }, 5.);
    /// 
    /// assert_eq!(line.end(), Vec2 { x: 5., y: 0. });
    /// ```
    pub fn new_direction(origin: Vec2, direction: Vec2, distance: f32) -> Self {
        Self {
            origin,
            end: origin + direction.normalize_or_zero() * distance,
        }
    }
}

// ##########
// Getters/Setters
// ##########
impl Line {
    /// Get the origin for the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// let origin = line.origin();
    /// 
    /// assert_eq!(origin, Vec2::ZERO);
    /// ```
    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    /// Get the end of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// let end = line.end();
    /// 
    /// assert_eq!(end, Vec2::ONE);
    /// ```
    pub fn end(&self) -> Vec2 {
        self.end
    }

    /// Set the origin of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let mut line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// line.set_origin(Vec2::NEG_ONE);
    /// 
    /// assert_eq!(line.origin(), Vec2::NEG_ONE);
    /// ```
    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }

    /// Set the end of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let mut line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// line.set_end(Vec2::NEG_ONE);
    /// 
    /// assert_eq!(line.end(), Vec2::NEG_ONE);
    /// ```
    pub fn set_end(&mut self, offset: Vec2) {
        self.end = offset;
    }
}

// ##########
// Attributes
// ##########
impl Line {
    /// Get the center of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let mut line = Line::new(Vec2::ZERO, Vec2::ONE);
    /// let center = line.center();
    /// 
    /// assert_eq!(center, Vec2 { x: 0.5, y: 0.5 });
    /// ```
    pub fn center(&self) -> Vec2 {
        (self.origin() + self.end()) * 0.5
    }

    /// Get the non-normalized direction of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let mut line = Line::new(Vec2::ZERO, Vec2 { x: 2., y: 0. });
    /// let direction = line.direction();
    /// 
    /// assert_eq!(direction, Vec2 { x: 2., y: 0. });
    /// ```
    pub fn direction(&self) -> Vec2 {
        self.end() - self.origin()
    }

    /// Get the length of the [`Line`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let mut line = Line::new(Vec2::ZERO, Vec2 { x: 2., y: 0. });
    /// let length = line.length();
    /// 
    /// assert_eq!(length, 2.);
    /// ```
    pub fn length(&self) -> f32 {
        // TODO: is this right?
        self.direction().max_element()
    }
}

// ##########
// Consts
// ##########
impl Line {
    /// A basic wrapper to set the [`Line`] to origin `(0., 0.)` and the end to `(0., 1.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line1 = Line::UP;
    /// let line2 = Line::new(Vec2::ZERO, Vec2 { x: 0., y: 1. });
    /// 
    /// assert_eq!(line1.origin(), line2.origin());
    /// assert_eq!(line1.end(), line2.end());
    /// ```
    pub const UP: Self = Self{ origin: Vec2::ZERO, end: Vec2 { x: 0., y: 1. } };

    /// A basic wrapper to set the [`Line`] to origin `(0., 0.)` and the end to `(1., 0.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line1 = Line::RIGHT;
    /// let line2 = Line::new(Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// 
    /// assert_eq!(line1.origin(), line2.origin());
    /// assert_eq!(line1.end(), line2.end());
    /// ```
    pub const RIGHT: Self = Self{ origin: Vec2::ZERO, end: Vec2 { x: 1., y: 0. } };

    /// A basic wrapper to set the [`Line`] to origin `(0., 0.)` and the end to `(0., -1.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line1 = Line::DOWN;
    /// let line2 = Line::new(Vec2::ZERO, Vec2 { x: 0., y: -1. });
    /// 
    /// assert_eq!(line1.origin(), line2.origin());
    /// assert_eq!(line1.end(), line2.end());
    /// ```
    pub const DOWN: Self = Self{ origin: Vec2::ZERO, end: Vec2 { x: 0., y: -1. } };
    
    /// A basic wrapper to set the [`Line`] to origin `(0., 0.)` and the end to `(-1., 0.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Line;
    /// 
    /// let line1 = Line::LEFT;
    /// let line2 = Line::new(Vec2::ZERO, Vec2 { x: -1., y: 0. });
    /// 
    /// assert_eq!(line1.origin(), line2.origin());
    /// assert_eq!(line1.end(), line2.end());
    /// ```
    pub const LEFT: Self = Self{ origin: Vec2::ZERO, end: Vec2 { x: -1., y: 0. } };
}

// ##########
// Default impl
// ##########
impl Default for Line {
    fn default() -> Self {
        Self {
            origin: Vec2::ZERO,
            end: Vec2::ONE,
        }
    }
}

// ##########
// Display impl
// ##########
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{ origin: {}, end: {} }}", self.origin(), self.end())
    }
}