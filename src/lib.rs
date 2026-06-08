/// MathRC
extern crate core;

pub mod err;

mod calculator;
mod function;
mod math;
mod num;
mod parser;
mod sequence;
mod set;
mod vector;

pub use vector::Vector;
pub use vector::Vector2d;
pub use vector::Vector3d;
pub use vector::VectorOps;

pub use set::PredicateSet;
pub use set::Set;

pub use sequence::sum;
pub use sequence::Seq;

pub use num::Frac;
pub use num::Matrix;

pub use function::Func;

pub use math::Math;
