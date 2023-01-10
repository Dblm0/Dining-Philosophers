use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct Fork {
    number: u8,
    pick_count: u16,
}
impl Fork {
    pub fn new(number: u8) -> Self {
        Fork {
            number,
            pick_count: 0,
        }
    }
}
pub type SharedFork = Arc<Mutex<Fork>>;

impl From<Fork> for SharedFork {
    fn from(source: Fork) -> Self {
        Arc::new(Mutex::new(source))
    }
}
