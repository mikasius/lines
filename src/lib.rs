use std::cmp::max;

pub fn transform(input: &str, line_width: u32) -> String {
    // Check if input is empty.
    if input.chars().count() == 0 {
        return "".to_string();
    }

    // Get max word length from input.
    let splitted: Vec<_> = input.split_whitespace().collect();
    let width = line_width as usize;
    let max_len = max(splitted.iter().map(|x| x.len()).max(), Some(width));

    // Initiate variables.
    let mut result: String = "".to_owned();
    let mut chunk: String = "".to_owned();

    for word in splitted {
        chunk.push_str(word);
        let is_chunk_overflew = chunk.len() > max_len.unwrap();

        // Overflow chunk.
        // max_len: 5
        // chunk: 'onethree' -> spaces: 5 - 8 = -3
        if is_chunk_overflew {
            let drain_range = chunk.find(word).unwrap_or(chunk.len());
            let prev_chunk: String = chunk.drain(..drain_range).collect();
            let spaces_to_add = chunk.len() - (prev_chunk.len() - 1);

            chunk.push_str(&(" ".repeat(spaces_to_add) + &"\n".to_owned()));
            result.push_str(&chunk);
        } else {
            // max_len: 9
            // chunk: 'onethree' -> spaces: 9 - 8 = 1
            let drain_range = chunk.find(word).unwrap_or(chunk.len());
            let prev_chunk: String = chunk.drain(..drain_range).collect();

            let spaces_to_add = chunk.len() - prev_chunk.len();

            chunk.push_str(&(" ".repeat(spaces_to_add) + &"\n".to_owned()));
            result.push_str(&chunk);
        }

        // if (chunk.len()) < max_len.unwrap() {
        //     // chunk.push_str(word);
        // } else {
        //     chunk.push_str(&" ".repeat(spaces.try_into().unwrap()));

        //     result.push_str(&(chunk.to_owned() + "n"));
        //     chunk = word.to_string();
        // }

        println!("result -> {}: '{}'", word, result);
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
