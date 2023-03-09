use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`point`] in 2d space
pub struct Point {
    coordinate: Vec2,
}

// ##########
// Constructors
// ##########
impl Point {
    /// Creates a new [`Point`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let point = Point::new(Vec2::ZERO);
    /// 
    /// assert_eq!(point.coordinate(), Vec2::ZERO);
    /// ```
    pub fn new(coordinate: Vec2) -> Self {
        Self {
            coordinate
        }
    }
}

// ##########
// Getters/Setters
// ##########
impl Point {
    /// Get the coordinate for the [`Point`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let point = Point::new(Vec2::ZERO);
    /// let coordinate = point.coordinate();
    /// 
    /// assert_eq!(coordinate, Vec2::ZERO);
    /// ```
    pub fn coordinate(&self) -> Vec2 {
        self.coordinate
    }

    /// Set the coordinate for the [`Point`]
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let mut point = Point::new(Vec2::ZERO);
    /// point.set_coordinate(Vec2::ONE);
    /// 
    /// assert_eq!(point.coordinate(), Vec2::ONE);
    /// ```
    pub fn set_coordinate(&mut self, coordinate: Vec2) {
        self.coordinate = coordinate;
    }
}

// ##########
// Consts
// ##########
impl Point {
    /// A basic wrapper to set the [`Point`] to coordinate `(0., 0.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let point1 = Point::ZERO;
    /// let point2 = Point::new(Vec2::ZERO);
    /// 
    /// assert_eq!(point1.coordinate(), point2.coordinate());
    /// ```
    pub const ZERO: Self = Self{ coordinate: Vec2::ZERO };
    /// A basic wrapper to set the [`Point`] to coordinate `(1., 1.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let point1 = Point::ONE;
    /// let point2 = Point::new(Vec2::ONE);
    /// 
    /// assert_eq!(point1.coordinate(), point2.coordinate());
    /// ```
    pub const ONE: Self = Self{ coordinate: Vec2::ONE };
    /// A basic wrapper to set the [`Point`] to coordinate `(-1., -1.)`
    /// 
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Point;
    /// 
    /// let point1 = Point::NEG_ONE;
    /// let point2 = Point::new(Vec2::NEG_ONE);
    /// 
    /// assert_eq!(point1.coordinate(), point2.coordinate());
    /// ```
    pub const NEG_ONE: Self = Self{ coordinate: Vec2::NEG_ONE };
}

// ##########
// Default impl
// ##########
impl Default for Point {
    fn default() -> Self {
        Self {
            coordinate: Vec2::ZERO,
        }
    }
}

// ##########
// Display impl
// ##########
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {{ coordinate: {} }}", self.coordinate())
    }
}