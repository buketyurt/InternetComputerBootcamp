use std::io;

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> Result<usize, &'static str> {
        if self.text.is_empty() {
            Err("Error: The input text is empty.")
        } else {
            Ok(self.text.split_whitespace().count())
        }
    }
}

fn main() {
    println!("Enter a text:");

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let trimmed_input = user_input.trim();
            let word_counter = WordCounter::new(trimmed_input);

            match word_counter.count_words() {
                Ok(word_count) => println!("Word count: {}", word_count),
                Err(error_message) => println!("Error: {}", error_message),
            }
        }
        Err(error) => println!("Error reading input: {}", error),
    }
}
