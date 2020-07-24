// iterators4.rs

// I AM NOT DONE

fn tail_recursion(num: u64, another: u64) -> u64 {
    match num {
      1 => another,
      _ => tail_recursion(num-1, another*num)
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    tail_recusion(num, 1)
}

fn main(){
  println!("{}", factorial(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
