// traits2.rs
// 
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
// 
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
// 
// No boiler plate code this time,
// you can do this!

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String>{

  // I'm fairly sure this is the least idomatic way to do this
  fn append_bar(self) -> Self {
      // initialise new_vec to be AT LEAST as long as the Vec self
      let mut new_vec: Vec<String> = vec!["".to_string(); self.len()];
      // clone each part of self into new_vec
      for i in self.iter() {
        new_vec.push(i.clone());
      }
      // do the actual append_bar
      new_vec.push("Bar".to_string());
      return new_vec
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

}
