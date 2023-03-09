use std::fmt::Display;

use glam::Vec2;

/// Represents a [`Ray`] in 2d space
pub struct Ray {
    origin: Vec2,
    direction: Vec2,
}

// ##########
// Constructors
// ##########
impl Ray {
    /// Creates a new [`Ray`] starting at the `origin` and pointing at an `offset`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray = Ray::new_offset(Vec2::ZERO, Vec2::ONE);
    ///
    /// assert_eq!(ray.direction(), Vec2::ONE.normalize_or_zero());
    /// ```
    pub fn new_offset(origin: Vec2, offset: Vec2) -> Self {
        Self {
            origin,
            direction: (offset - origin).normalize_or_zero(),
        }
    }

    /// Creates a new [`Ray`] starting at the `origin` and pointing in a `direction`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray = Ray::new_direction(Vec2::ZERO, Vec2::ONE);
    ///
    /// assert_eq!(ray.offset(), Vec2::ONE.normalize_or_zero());
    /// ```
    pub fn new_direction(origin: Vec2, direction: Vec2) -> Self {
        Self {
            origin,
            direction: direction.normalize_or_zero(),
        }
    }
}

// ##########
// Getters/Setters
// ##########
impl Ray {
    /// Get the origin for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray = Ray::new_offset(Vec2::ZERO, Vec2::ONE);
    /// let origin = ray.origin();
    ///
    /// assert_eq!(origin, Vec2::ZERO);
    /// ```
    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    /// Get the normalized offset for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let mut ray = Ray::UP;
    /// ray.set_origin(Vec2::ONE);
    /// let offset = ray.offset();
    ///
    /// assert_eq!(offset, Vec2 { x: 1., y: 2. });
    /// ```
    pub fn offset(&self) -> Vec2 {
        self.direction + self.origin
    }

    /// Get the normalized direction for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray = Ray::UP;
    /// let direction = ray.direction();
    ///
    /// assert_eq!(direction, Vec2 { x: 0., y: 1. });
    /// ```
    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    /// Set the origin for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let mut ray = Ray::new_offset(Vec2::ZERO, Vec2::ONE);
    /// ray.set_origin(Vec2::ONE);
    ///
    /// assert_eq!(ray.origin(), Vec2::ONE);
    /// ```
    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }

    /// Set the normalized offset for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let mut ray = Ray::new_offset(Vec2::ONE, Vec2::ONE);
    /// ray.set_offset(Vec2 { x: 1., y: 2. });
    ///
    /// assert_eq!(ray.direction(), Vec2 { x: 0., y: 1. });
    /// ```
    pub fn set_offset(&mut self, offset: Vec2) {
        self.direction = (offset - self.origin).normalize_or_zero();
    }

    /// Get the normalized direction for the [`Ray`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let mut ray = Ray::new_offset(Vec2::ONE, Vec2::ONE);
    /// ray.set_direction(Vec2 { x:0., y: 1. });
    ///
    /// assert_eq!(ray.offset(), Vec2 { x: 1., y: 2. });
    /// ```
    pub fn set_direction(&mut self, direction: Vec2) {
        self.direction = direction.normalize_or_zero()
    }
}

// ##########
// Consts
// ##########
impl Ray {
    /// A basic wrapper to set the [`Ray`] to origin `(0., 0.)` and the direction pointing upwards
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray1 = Ray::UP;
    /// let ray2 = Ray::new_offset(Vec2::ZERO, Vec2 { x: 0., y: 1. });
    ///
    /// assert_eq!(ray1.direction(), ray2.direction());
    /// ```
    pub const UP: Self = Self {
        origin: Vec2::ZERO,
        direction: Vec2 { x: 0., y: 1. },
    };

    /// A basic wrapper to set the [`Ray`] to origin `(0., 0.)` and the direction pointing to the right
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray1 = Ray::RIGHT;
    /// let ray2 = Ray::new_offset(Vec2::ZERO, Vec2 { x: 1., y: 0. });
    ///
    /// assert_eq!(ray1.direction(), ray2.direction());
    /// ```
    ///
    pub const RIGHT: Self = Self {
        origin: Vec2::ZERO,
        direction: Vec2 { x: 1., y: 0. },
    };

    /// A basic wrapper to set the [`Ray`] to origin `(0., 0.)` and the direction pointing downwards
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray1 = Ray::DOWN;
    /// let ray2 = Ray::new_offset(Vec2::ZERO, Vec2 { x: 0., y: -1. });
    ///
    /// assert_eq!(ray1.direction(), ray2.direction());
    /// ```
    pub const DOWN: Self = Self {
        origin: Vec2::ZERO,
        direction: Vec2 { x: 0., y: -1. },
    };

    /// A basic wrapper to set the [`Ray`] to origin `(0., 0.)` and the direction pointing to the left
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Ray;
    ///
    /// let ray1 = Ray::LEFT;
    /// let ray2 = Ray::new_offset(Vec2::ZERO, Vec2 { x: -1., y: 0. });
    ///
    /// assert_eq!(ray1.direction(), ray2.direction());
    /// ```
    pub const LEFT: Self = Self {
        origin: Vec2::ZERO,
        direction: Vec2 { x: -1., y: 0. },
    };
}

// ##########
// Default impl
// ##########
impl Default for Ray {
    fn default() -> Self {
        Self::new_offset(Vec2::ZERO, Vec2::ONE)
    }
}

// ##########
// Display impl
// ##########
impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ray {{ origin: {}, direction: {} }}",
            self.origin(),
            self.direction()
        )
    }
}
