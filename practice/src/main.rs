fn main() {
    let t = "one two three four five";

    println!("the first word is {}", first_word(&t));

    println!("the third word is {}", get_word(&t, 3));
}

// gets the first word of a slice
fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    // the whole string is the first word
    &s[..]
}

// need to work on debugging this and finding the solution
fn get_word(s: &str, word: i32) -> &str {
    let s_bytes = s.as_bytes();

    let mut words_found = 0;
    let mut last_index = 0;

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            if words_found == word {
                return &s[last_index..i]
            }
            words_found += 1;
            last_index = i;
        }
    }

    &s
}
