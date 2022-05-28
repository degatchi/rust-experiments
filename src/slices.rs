

#[allow(dead_code)]
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // Byte of a space
        if item == b' ' {
            return i
        }
    }

    // If no space, return byte len + 1.
    s.len()
}