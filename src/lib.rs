#![warn(clippy::all, rust_2018_idioms)]

pub mod walk_app;
pub use walk_app::render_random_walk;
pub use walk_app::WalkApp;
pub mod random_cordinates;
pub use random_cordinates::random_cordinates_one_dim;
pub use random_cordinates::random_cordinates_two_dim;
