use std::{cmp::max, io::LineWriter};

pub fn transform(input: &str, line_width: u32) -> String {
    // Check if input is empty.
    if input.chars().count() == 0 {
        return "".to_string();
    }

    // Get max word length from input.
    let some = input.split_whitespace();
    let max_len = max(some.map(|x| x.len() as u32).max(), Some(line_width)).unwrap();

    let mut result: String = "".to_owned();
    let mut chunk: String = "".to_owned();

    // for i in 0..max_len {
    //     let curr = some.into_iter()[i];
    // }

    for word in input.split_whitespace() {
        if ((chunk.len() + word.len()) as u32) < max_len {
            chunk.push_str(word);
        } else {
            let free_spaces = max_len - word.len() as u32;
            chunk.push_str(&" ".repeat(free_spaces.try_into().unwrap()));

            result.push_str(&(chunk.clone() + &"\n"));
        }

        println!("'{}'", chunk);
        println!("'{}'", result);
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
