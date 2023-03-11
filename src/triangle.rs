use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`Triangle`] in 2d space
pub struct Triangle {
    coordinate1: Vec2,
    coordinate2: Vec2,
    coordinate3: Vec2,
}

// ##########
// Constructors
// ##########
impl Triangle {
    /// Creates a new [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1.0, y: 0.0 });
    ///
    /// assert_eq!(triangle.coordinate1(), Vec2::ONE);
    /// assert_eq!(triangle.coordinate2(), Vec2::ZERO);
    /// assert_eq!(triangle.coordinate3(), Vec2 { x: 1.0, y: 0.0 });
    /// ```
    pub fn new(coordinate1: Vec2, coordinate2: Vec2, coordinate3: Vec2) -> Self {
        Self {
            coordinate1,
            coordinate2,
            coordinate3,
        }
    }
}

// ##########
// Getters/Setters
// ##########
impl Triangle {
    /// Get the first coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// let coordinate = triangle.coordinate1();
    ///
    /// assert_eq!(coordinate, Vec2::ONE);
    /// ```
    pub fn coordinate1(&self) -> Vec2 {
        self.coordinate1
    }

    /// Get the second coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// let coordinate = triangle.coordinate2();
    ///
    /// assert_eq!(coordinate, Vec2::ZERO);
    /// ```
    pub fn coordinate2(&self) -> Vec2 {
        self.coordinate2
    }

    /// Get the third coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// let coordinate = triangle.coordinate3();
    ///
    /// assert_eq!(coordinate, Vec2 { x: 1., y: 0. });
    /// ```
    pub fn coordinate3(&self) -> Vec2 {
        self.coordinate3
    }

    /// Set the first coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let mut triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// triangle.set_coordinate1( Vec2 { x: 2., y: 1. });
    ///
    /// assert_eq!(triangle.coordinate1(), Vec2 { x: 2., y: 1. });
    /// ```
    pub fn set_coordinate1(&mut self, coordinate1: Vec2) {
        self.coordinate1 = coordinate1;
    }

    /// Set the second coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let mut triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// triangle.set_coordinate2( Vec2 { x: 2., y: 1. });
    ///
    /// assert_eq!(triangle.coordinate2(), Vec2 { x: 2., y: 1. });
    /// ```
    pub fn set_coordinate2(&mut self, coordinate2: Vec2) {
        self.coordinate2 = coordinate2;
    }

    /// Set the third coordinate for the [`Triangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Triangle;
    ///
    /// let mut triangle = Triangle::new(Vec2::ONE, Vec2::ZERO, Vec2 { x: 1., y: 0. });
    /// triangle.set_coordinate3( Vec2 { x: 2., y: 1. });
    ///
    /// assert_eq!(triangle.coordinate3(), Vec2 { x: 2., y: 1. });
    /// ```
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
        write!(
            f,
            "Triangle {{ coordinate1: {}, coordinate2: {}, coordinate3: {} }}",
            self.coordinate1(),
            self.coordinate2(),
            self.coordinate3()
        )
    }
}
