// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`

// I AM NOT DONE

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => format!("{}", first.to_uppercase()) + c.as_str(),
    }
}

pub fn words_to_capitalized_first<'a>(input: &'a Vec<&str>) -> Vec<String> {
    let mut return_vec = vec![];
    for i in input.iter() {
       return_vec.push(capitalize_first(i));
    }
    return_vec
}

/*
// this isn't needed
pub fn vec_to_capitalized_str<'a>(input: &'a Vec<&str>) -> str {
    for i in input.iter() {
       match capitalize_first(i).as_str(){
          None => String::new(),
          Some(s) => s.unwrap().to_string(),
       }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words_to_capitalized_first(&words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        //let capitalized_words = vec_to_capitalized_str(&words);
        //println!("{:?}", words_to_capitalized_first(&words).as_mut_slice());
        //let mut capitalized_words: str = words_to_capitalized_first(&words).as_mut_slice();
        //let capitalized_words = &*words_to_capitalized_first(&words).as_mut_slice().as_ref()[0..];
        //let capitalized_words: &str = words_to_capitalized_first(&words).as_ptr();
        //let capitalized_words = words_to_capitalized_first(&words).as_ptr();
        //let capitalized_words = words_to_capitalized_first(&words);
        //let capitalized_words: String = words_to_capitalized_first(&words);
        //let capitalized_words: String = words_to_capitalized_first(&words).unwrap();
        //let capitalized_words: str = Box::leak(words_to_capitalized_first(&words).into_boxed_str());
        //let capitalized_words = words_to_capitalized_first(&words).as_slice(); // &[std::string::String] == std::string::String
        //let capitalized_words = words_to_capitalized_first(&words).concat();  // WORKS!!!!!!!!!!!
        //let capitalized_words = words_to_capitalized_first(&words).join(''); // FAIL
        //let capitalized_words = words_to_capitalized_first(&words).join(""); // WORKS
        //let capitalized_words = words_to_capitalized_first(&words).iter().parse::<str>(); // FAIL to use turbofish
        //let capitalized_words = words_to_capitalized_first(&words).iter::<str>(); // FAIL to use turbofish
        //let capitalized_words: String = *words_to_capitalized_first(&words).iter().collect(); // FAIL to use turbofish
        //let capitalized_words = words_to_capitalized_first(&words).into_iter().collect::<String>(); // TURBOFISH!
        let capitalized_words: String = words_to_capitalized_first(&words).into_iter().collect(); // fishless
        assert_eq!(capitalized_words, "Hello World".to_string());
    }
}
