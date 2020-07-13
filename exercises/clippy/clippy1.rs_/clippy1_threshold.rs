// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

// I AM NOT DONE

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let _threshold = 0.0001f64; 
    /* 
        we can set a threshold if we require significant difference.
    */
    if (y-x).abs() > _threshold {  // <- This is better than hard coding a value:  if (y-x).abs() > 0.0f64 {
        println!("Success!");
    }
}
