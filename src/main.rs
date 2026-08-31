use crate::app::run;

pub mod app;
pub mod state;
pub mod vertex;

fn main() {
    run().unwrap();
}
