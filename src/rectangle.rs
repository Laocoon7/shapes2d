use std::fmt::Display;

use glam::Vec2;

/// Represents a single [`Rectangle`] in 2d space
pub struct Rectangle {
    min: Vec2,
    max: Vec2,
}
// ##########
// Constructors
// ##########
impl Rectangle {
    /// Creates a new [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 1., 1.);
    ///
    /// assert_eq!(rect.x(), 0.);
    /// assert_eq!(rect.y(), 0.);
    /// assert_eq!(rect.max_x(), 1.);
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self::new_coordinates(Vec2 { x: min_x, y: min_y }, Vec2 { x: max_x, y: max_y })
    }

    /// Creates a new [`Rectangle`] given `min` and `max` coordinates
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new_coordinates(Vec2::ZERO, Vec2::ONE);
    ///
    /// assert_eq!(rect.x(), 0.);
    /// assert_eq!(rect.y(), 0.);
    /// assert_eq!(rect.max_x(), 1.);
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
    pub fn new_coordinates(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Creates a new [`Rectangle`] given a `min` coordinate and a `width` and a `height`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new_dimensions(Vec2::ZERO, 1., 1.);
    ///
    /// assert_eq!(rect.x(), 0.);
    /// assert_eq!(rect.y(), 0.);
    /// assert_eq!(rect.max_x(), 1.);
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
    pub fn new_dimensions(min: Vec2, width: f32, height: f32) -> Self {
        Self::new_coordinates(min, Vec2::new(min.x + width, min.y + height))
    }
}

// ##########
// Getters/Setters
// ##########
impl Rectangle {
    /// Get the minimum coordinate for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 1., 1.);
    /// let min = rect.min();
    ///
    /// assert_eq!(min, Vec2::ZERO);
    /// ```
    pub fn min(&self) -> Vec2 {
        self.min
    }

    /// Get the maximum coordinate for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 1., 1.);
    /// let max = rect.max();
    ///
    /// assert_eq!(max, Vec2::ONE);
    /// ```
    pub fn max(&self) -> Vec2 {
        self.max
    }

    /// Sets the minimum coordinate for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_min(Vec2::ONE);
    ///
    /// assert_eq!(rect.min(), Vec2::ONE);
    /// ```
    pub fn set_min(&mut self, min: Vec2) {
        self.min = min;
    }

    /// Sets the maximum coordinate for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_max(Vec2::ONE);
    ///
    /// assert_eq!(rect.max(), Vec2::ONE);
    /// ```
    pub fn set_max(&mut self, max: Vec2) {
        self.max = max;
    }
}

// ##########
// Attributes
// ##########
impl Rectangle {
    /// Get the minimum `x` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let x = rect.x();
    ///
    /// assert_eq!(x, 0.);
    /// ```
    pub fn x(&self) -> f32 {
        self.min.x
    }

    /// Get the minimum `x` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let x = rect.min_x();
    ///
    /// assert_eq!(x, 0.);
    /// ```
    pub fn min_x(&self) -> f32 {
        self.min.x
    }

    /// Get the maximum `x` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let x = rect.max_x();
    ///
    /// assert_eq!(x, 2.);
    /// ```
    pub fn max_x(&self) -> f32 {
        self.max.x
    }

    /// Get the minimum `y` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let y = rect.y();
    ///
    /// assert_eq!(y, 0.);
    /// ```
    pub fn y(&self) -> f32 {
        self.min.y
    }

    /// Get the minimum `y` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let y = rect.y();
    ///
    /// assert_eq!(y, 0.);
    /// ```
    pub fn min_y(&self) -> f32 {
        self.min.y
    }

    /// Get the maximum `y` value for the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let y = rect.max_y();
    ///
    /// assert_eq!(y, 2.);
    /// ```
    pub fn max_y(&self) -> f32 {
        self.max.y
    }

    /// Get the `width` of the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let width = rect.width();
    ///
    /// assert_eq!(width, 2.);
    /// ```
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Get the `height` of the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let height = rect.height();
    ///
    /// assert_eq!(height, 2.);
    /// ```
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Get the `size` of the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let size = rect.size();
    ///
    /// assert_eq!(size.x, 2.);
    /// assert_eq!(size.y, 2.);
    /// ```
    pub fn size(&self) -> Vec2 {
        Vec2 {
            x: self.width(),
            y: self.height(),
        }
    }

    /// Get the `position` of the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let position = rect.position();
    ///
    /// assert_eq!(position.x, 0.);
    /// assert_eq!(position.y, 0.);
    /// ```
    pub fn position(&self) -> Vec2 {
        self.min
    }

    /// Get the `center` of the [`Rectangle`]
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let rect = Rectangle::new(0., 0., 2., 2.);
    /// let center = rect.center();
    ///
    /// assert_eq!(center.x, 1.);
    /// assert_eq!(center.y, 1.);
    /// ```
    pub fn center(&self) -> Vec2 {
        Vec2 {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
        }
    }

    /// Sets the minimum `x` coordinate for the [`Rectangle`] while maintaining the width
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.width(), 2.);
    ///
    /// rect.set_x(1.);
    ///
    /// assert_eq!(rect.width(), 2.);
    /// ```
    pub fn set_x(&mut self, x: f32) {
        self.max.x += x - self.min.x;
        self.min.x = x;
    }

    /// Sets the minimum `x` coordinate for the [`Rectangle`] while changing the width
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.width(), 2.);
    ///
    /// rect.set_min_x(1.);
    ///
    /// assert_eq!(rect.width(), 1.);
    /// ```
    pub fn set_min_x(&mut self, x: f32) {
        self.min.x = x;
    }

    /// Sets the maximum `x` coordinate for the [`Rectangle`] while changing the width
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.width(), 2.);
    ///
    /// rect.set_max_x(1.);
    ///
    /// assert_eq!(rect.width(), 1.);
    /// ```
    pub fn set_max_x(&mut self, x: f32) {
        self.max.x = x;
    }

    /// Sets the minimum `y` coordinate for the [`Rectangle`] while maintaining the height
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.height(), 2.);
    ///
    /// rect.set_y(1.);
    ///
    /// assert_eq!(rect.height(), 2.);
    /// ```
    pub fn set_y(&mut self, y: f32) {
        self.max.y += y - self.min.y;
        self.min.y = y;
    }

    /// Sets the minimum `y` coordinate for the [`Rectangle`] while changing the height
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.height(), 2.);
    ///
    /// rect.set_min_y(1.);
    ///
    /// assert_eq!(rect.height(), 1.);
    /// ```
    pub fn set_min_y(&mut self, y: f32) {
        self.min.y = y;
    }

    /// Sets the maximum `y` coordinate for the [`Rectangle`] while changing the height
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// assert_eq!(rect.height(), 2.);
    ///
    /// rect.set_max_y(1.);
    ///
    /// assert_eq!(rect.height(), 1.);
    /// ```
    pub fn set_max_y(&mut self, y: f32) {
        self.max.y = y;
    }

    /// Sets the maximum `x` coordinate for the [`Rectangle`] relative to the minimum `x` coordinate
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_width(1.);
    ///
    /// assert_eq!(rect.max_x(), 1.);
    /// ```
    pub fn set_width(&mut self, width: f32) {
        self.max.x = self.min.x + width;
    }

    /// Sets the maximum `y` coordinate for the [`Rectangle`] relative to the minimum `y` coordinate
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_height(1.);
    ///
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
    pub fn set_height(&mut self, height: f32) {
        self.max.y = self.min.y + height;
    }

    /// Sets the maximum coordinate for the [`Rectangle`] relative to the minimum coordinate
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_size(Vec2::ONE);
    ///
    /// assert_eq!(rect.max_x(), 1.);
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
    pub fn set_size(&mut self, size: Vec2) {
        self.set_width(size.x);
        self.set_height(size.y);
    }

    /// Sets the minimum coordinate for the [`Rectangle`] moving the maximum coordinate
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_position(Vec2::ONE);
    ///
    /// assert_eq!(rect.max_x(), 3.);
    /// assert_eq!(rect.max_y(), 3.);
    /// ```
    pub fn set_position(&mut self, position: Vec2) {
        self.set_x(position.x);
        self.set_y(position.y);
    }

    /// Sets the minimum and maximum coordinates for the [`Rectangle`] relative to the `center`
    ///
    /// ```
    /// use glam::Vec2;
    /// use shapes2d::prelude::Rectangle;
    ///
    /// let mut rect = Rectangle::new(0., 0., 2., 2.);
    /// rect.set_center(Vec2::ZERO);
    ///
    /// assert_eq!(rect.min_x(), -1.);
    /// assert_eq!(rect.min_y(), -1.);
    /// assert_eq!(rect.max_x(), 1.);
    /// assert_eq!(rect.max_y(), 1.);
    /// ```
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
