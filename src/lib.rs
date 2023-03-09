#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "circle")]
/// Contains the [`Circle`] structure and related methods
pub mod circle;
#[cfg(feature = "ellipse")]
/// Contains the [`Ellipse`] structure and related methods
pub mod ellipse;
#[cfg(feature = "line")]
/// Contains the [`Line`] structure and related methods
pub mod line;
#[cfg(feature = "mesh")]
/// Contains the [`Mesh`] structure and related methods
pub mod mesh;
#[cfg(feature = "point")]
/// Contains the [`Point`] structure and related methods
pub mod point;
#[cfg(feature = "ray")]
/// Contains the [`Ray`] structure and related methods
pub mod ray;
#[cfg(feature = "rectangle")]
/// Contains the [`Rectangle`] structure and related methods
pub mod rectangle;
#[cfg(feature = "triangle")]
/// Contains the [`Triangle`] structure and related methods
pub mod triangle;

/// Contains the included shapes
pub mod prelude {
    #[cfg(feature = "circle")]
    pub use crate::circle::Circle;
    #[cfg(feature = "ellipse")]
    pub use crate::ellipse::Ellipse;
    #[cfg(feature = "line")]
    pub use crate::line::Line;
    #[cfg(feature = "mesh")]
    pub use crate::mesh::Mesh;
    #[cfg(feature = "point")]
    pub use crate::point::Point;
    #[cfg(feature = "ray")]
    pub use crate::ray::Ray;
    #[cfg(feature = "rectangle")]
    pub use crate::rectangle::Rectangle;
    #[cfg(feature = "triangle")]
    pub use crate::triangle::Triangle;
}
