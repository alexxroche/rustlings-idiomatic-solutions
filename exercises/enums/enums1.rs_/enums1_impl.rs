// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// This is another valid soution to enums1.rs
// (Example of extra credit.)

#[derive(Debug)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(u32, u32, u32)
}

impl Message {
  fn Quit() -> i32{
    0
  } 
  fn Echo(msg: String) -> String {
    format!("{:?}", msg)
  }
  fn Move() -> String{
    format!("EaRtHQuaKE")
  }
  fn ChangeColor() -> String {
    format!("\x1b[01;38;5;202m quit \x1b[0m\n")
  }
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("This".to_string()));
    println!("{:?}", Message::Move{x:7,y: -2});
    println!("{:?}", Message::ChangeColor(134,78,11));
}
