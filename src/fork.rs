use std::sync::{Arc, Mutex, MutexGuard};
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

impl std::fmt::Debug for Fork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fork")
            .field("number", &self.number)
            .field("pick_count", &self.pick_count)
            .finish()
    }
}

pub type SharedFork = Arc<Mutex<Fork>>;

impl From<Fork> for SharedFork {
    fn from(source: Fork) -> Self {
        Arc::new(Mutex::new(source))
    }
}
pub trait PickableFork {
    fn pick(&mut self) -> MutexGuard<Fork>;
}

impl PickableFork for SharedFork {
    fn pick(&mut self) -> MutexGuard<Fork> {
        let mut guard = self.lock().unwrap();
        guard.pick_count += 1;
        guard
    }
}
