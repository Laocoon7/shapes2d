use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`Circle`] in 2d space
pub struct Circle {
    center: Vec2,
    radius: f32,
}

// ##########
// Constructors
// ##########
impl Circle {
    /// Creates a new [`Circle`] with a `radius`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let circle = Circle::new(Vec2::ZERO, 1.);
    ///
    /// assert_eq!(circle.diameter(), 2.);
    /// ```
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Creates a new [`Circle`] with a `diameter`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let circle = Circle::new_diameter(Vec2::ZERO, 2.);
    ///
    /// assert_eq!(circle.radius(), 1.);
    /// ```
    pub fn new_diameter(center: Vec2, diameter: f32) -> Self {
        Self::new(center, diameter * 0.5)
    }
}

// ##########
// Getters/Setters
// ##########
impl Circle {
    /// Get the `center` of the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let circle = Circle::new(Vec2::ZERO, 1.);
    /// let center = circle.center();
    ///
    /// assert_eq!(center, Vec2::ZERO);
    /// ```
    pub fn center(&self) -> Vec2 {
        self.center
    }

    /// Get the `radius` of the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let circle = Circle::new(Vec2::ZERO, 1.);
    /// let radius = circle.radius();
    ///
    /// assert_eq!(radius, 1.);
    /// ```
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get the `diameter` of the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let circle = Circle::new(Vec2::ZERO, 1.);
    /// let diameter = circle.diameter();
    ///
    /// assert_eq!(diameter, 2.);
    /// ```
    pub fn diameter(&self) -> f32 {
        self.radius * 2.
    }

    /// Set a new `center` for the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let mut circle = Circle::new(Vec2::ZERO, 1.);
    /// circle.set_center(Vec2::ONE);
    ///
    /// assert_eq!(circle.center(), Vec2::ONE);
    /// ```
    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
    }

    /// Set a new `radius` for the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let mut circle = Circle::new(Vec2::ZERO, 1.);
    /// circle.set_radius(2.);
    ///
    /// assert_eq!(circle.diameter(), 4.);
    /// ```
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    /// Set a new `diameter` for the [`Circle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Circle;
    ///
    /// let mut circle = Circle::new(Vec2::ZERO, 1.);
    /// circle.set_diameter(4.);
    ///
    /// assert_eq!(circle.radius(), 2.);
    /// ```
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
        write!(
            f,
            "Circle {{ center: {}, radius: {} }}",
            self.center(),
            self.radius()
        )
    }
}
