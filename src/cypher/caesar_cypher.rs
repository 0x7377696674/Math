// caesar cypher

pub fn encode(plain: String, key: u8) -> String {
    // Make sure the key is within the alphabet range
    assert!(key > 0 && key < 26, "Key must be between 0 and 26");

    let mut cypher = String::new();

    for byte in plain.as_bytes() {
        match byte {
            b'a'..=b'z' => {
                let shift = (byte - b'a' + key) % 26 + b'a';
                cypher.push(shift as char);
            }
            b'A'..=b'Z' => {
                let shift = (byte - b'A' + key) % 26 + b'A';
                cypher.push(shift as char);
            }
            _ => {
                cypher.push(b'.' as char);
            }
        }
    }
    cypher
}
