//solution to https://google.github.io/comprehensive-rust/exercises/day-4/dining-philosophers.html
use std::sync::mpsc::{self};
use std::thread::{self, JoinHandle};
use std::time::Duration;

mod fork;
mod philosopher;
use fork::*;
use philosopher::*;

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];
const CYCLE_COUNT: u8 = 5;
fn main() {
    let (thoughts, thoughts_sink) = mpsc::channel::<String>();
    // Create forks
    let forks: Vec<SharedFork> = (0..PHILOSOPHERS.len())
        .map(|x| Fork::new(x as u8).into())
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

    let last = dining_philosophers.iter_mut().last().unwrap();
    last.swap_hands();

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
