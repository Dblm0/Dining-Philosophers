use crate::fork::SharedFork;
use std::{sync::mpsc::Sender, thread, time::Duration};

#[derive(Debug)]
pub struct Philosopher {
    name: String,
    left_fork: SharedFork,
    right_fork: SharedFork,
    thoughts: Sender<String>,

    eat_count: u16,
    thought_count: u16,
}

impl Philosopher {
    pub fn new(
        name: String,
        left_fork: SharedFork,
        right_fork: SharedFork,
        thoughts: Sender<String>,
    ) -> Self {
        Philosopher {
            name,
            left_fork,
            right_fork,
            thoughts,

            eat_count: 0,
            thought_count: 0,
        }
    }

    pub fn think(&mut self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
        self.thought_count += 1;
    }

    pub fn eat(&mut self) {
        let _l = self.left_fork.lock().unwrap();
        thread::sleep(Duration::from_millis(1_000));
        let _r = self.right_fork.lock().unwrap();
        println!("{} is eating ...", &self.name);

        thread::sleep(Duration::from_millis(10));
        self.eat_count += 1;
    }
}
