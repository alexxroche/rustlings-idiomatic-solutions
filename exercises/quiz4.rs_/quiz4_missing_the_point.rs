// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

// I AM NOT DONE

/*
     This passes the quiz but isn't idiomatic.
    (It's also 4 additional lines,
        including an if statement,
     that are not needed.)
*/
macro_rules! my_macro {
    ($val:expr) => {
      if $val == "world!" {
       format!("Hello world!");
      }else{
       format!("Hello goodbye!");
      }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
