//solution to https://google.github.io/comprehensive-rust/exercises/day-4/dining-philosophers.html
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
struct Fork(u8);
type SharedFork = Arc<Mutex<Fork>>;

impl From<Fork> for SharedFork {
    fn from(source: Fork) -> Self {
        Arc::new(Mutex::new(source))
    }
}
#[derive(Debug)]
struct Philosopher {
    name: String,
    left_fork: SharedFork,
    right_fork: SharedFork,
    thoughts: Sender<String>,

    eat_count: u16,
    thought_count: u16,
}

impl Philosopher {
    fn new(
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

    fn think(&mut self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
        self.thought_count += 1;
    }

    fn eat(&mut self) {
        let _l = self.left_fork.lock().unwrap();
        thread::sleep(Duration::from_millis(1_000));
        let _r = self.right_fork.lock().unwrap();
        println!("{} is eating ...", &self.name);

        thread::sleep(Duration::from_millis(10));
        self.eat_count += 1;
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];
const CYCLE_COUNT: u8 = 5;
fn main() {
    let (thoughts, thoughts_sink) = mpsc::channel::<String>();
    // Create forks
    let forks: Vec<SharedFork> = (0..PHILOSOPHERS.len())
        .map(|x| Fork(x as u8).into())
        .collect();

    let fork_pairs_idxs: Vec<(usize, usize)> = {
        let original: Vec<usize> = (0..PHILOSOPHERS.len()).collect();
        let mut rotated = original.clone();
        rotated.rotate_right(1);
        std::iter::zip(original, rotated).collect()
    };

    // Create philosophers
    let mut dining_philosophers: Vec<Philosopher> = Vec::new();
    for (name, (left, right)) in PHILOSOPHERS.iter().zip(fork_pairs_idxs) {
        dining_philosophers.push(Philosopher::new(
            name.to_string(),
            forks[left].clone(),
            forks[right].clone(),
            thoughts.clone(),
        ));
    }

    // Make them think and eat
    let handles: Vec<JoinHandle<Philosopher>> = dining_philosophers
        .into_iter()
        .map(|mut philosopher| {
            thread::spawn(move || {
                for _ in 0..CYCLE_COUNT {
                    philosopher.think();
                    philosopher.eat();
                }
                philosopher
            })
        })
        .collect();

    // Output their thoughts
    while let Ok(received_thoughts) = thoughts_sink.recv_timeout(Duration::from_secs(1)) {
        println!("{received_thoughts}");
    }

    for h in handles {
        let philosopher = h.join().unwrap();
        println!("{philosopher:?}");
    }
}
