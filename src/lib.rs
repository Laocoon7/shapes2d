#![warn(missing_docs)]

#[cfg(feature = "point")]
/// Contains the [`Point`] structure and related methods
pub mod point;
#[cfg(feature = "ray")]
pub mod ray;
#[cfg(feature = "line")]
pub mod line;
#[cfg(feature = "triangle")]
pub mod triangle;
#[cfg(feature = "rectangle")]
pub mod rectangle;
#[cfg(feature = "circle")]
pub mod circle;
#[cfg(feature = "ellipse")]
pub mod ellipse;
#[cfg(feature = "mesh")]
pub mod mesh;

pub mod prelude {
    #[cfg(feature = "point")]
    pub use crate::point::Point;
    #[cfg(feature = "ray")]
    pub use crate::ray::Ray;
    #[cfg(feature = "line")]
    pub use crate::line::Line;
    #[cfg(feature = "triangle")]
    pub use crate::triangle::Triangle;
    #[cfg(feature = "rectangle")]
    pub use crate::rectangle::Rectangle;
    #[cfg(feature = "circle")]
    pub use crate::circle::Circle;
    #[cfg(feature = "ellipse")]
    pub use crate::ellipse::Ellipse;
    #[cfg(feature = "mesh")]
    pub use crate::mesh::Mesh;
}