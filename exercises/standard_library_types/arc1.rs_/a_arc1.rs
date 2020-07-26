// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)
// and/or read https://doc.rust-lang.org/std/sync/struct.Arc.html

// I AM NOT DONE
// Read https://medium.com/@DylanKerler1/how-arc-works-in-rust-b06192acd0a6 
//  if you need help understanding Arc (and how Arc::clone(&arc_ref) "unwraps" the Arc)

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);// TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    println!("\nnow that child_numbers is out of scope we can use shared_numbers again: {:?}", shared_numbers.iter());
    println!("shared_numbers: {:?}", shared_numbers);
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
