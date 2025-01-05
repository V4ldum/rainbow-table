use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub(crate) struct Hash(String);

#[derive(Clone)]
pub(crate) struct Clear(String);

impl Hash {
    pub(crate) fn new(clear: &Clear) -> Self {
        Hash(format!("{:x}", md5::compute(&clear.0)))
    }

    pub(crate) fn from_hash_str(hash: &str) -> Self {
        Hash(hash.to_owned())
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Clear {
    pub(crate) fn new(clear: &str) -> Self {
        Clear(clear.into())
    }
    pub(crate) fn from_hash(
        hash: &Hash,
        length: u32,
        current_iteration: u32,
        charset: &str,
    ) -> Self {
        let charset: Vec<char> = charset.chars().collect();
        let mut hash = hash.0.as_bytes().iter().cycle();

        // Skip first {current_iteration} bytes
        (0..current_iteration).for_each(|_| {
            hash.next().expect("hash is a cycle and shouldn't panic");
        });

        Clear(
            hash.take(length as usize)
                .map(|&byte| {
                    // We want map the byte to the corresponding hex value
                    let hex_value = byte_to_hex(byte) as f64;
                    let index = (hex_value / 16.0 * (charset.len() as f64)).round() as usize;
                    charset[index]
                })
                .collect(),
        )
    }
}

impl PartialEq for Clear {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Clear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn byte_to_hex(byte: u8) -> u8 {
    match byte {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'A' | b'a' => 10,
        b'B' | b'b' => 11,
        b'C' | b'c' => 12,
        b'D' | b'd' => 13,
        b'E' | b'e' => 14,
        b'F' | b'f' => 15,
        _ => panic!("unsupported value {byte}"),
    }
}
