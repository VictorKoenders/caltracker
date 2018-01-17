#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate rocket;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate uuid;
extern crate shared;

pub mod schema;
pub mod model;
mod pg_pool;

pub use pg_pool::*;

