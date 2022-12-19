#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for value in values {
        let mut bytes = vec![(value & 0x7f) as u8];
        let mut number: u32 = value >> 7;
        while number != 0 {
            bytes.insert(0, (number & 0x7f | 0x80) as u8);
            number >>= 7;
        }
        result.extend(bytes);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut decoded: Vec<u32> = vec![];
    let mut number = 0;
    for (i, byte) in bytes.iter().enumerate() {
        // max u32, 4 bytes, so if num > 0x0FFF_FFF encoded is above u32
        if number >= 0xFFF_FFFF {
            return Err(Error::Overflow);
        }
        
        number <<= 7;
        number |= u32::from(byte & 0x7f);

        if 0x80 & byte == 0 {
            decoded.push(number);
            number = 0;
        } else if i + 1 == bytes.len() {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(decoded)
}
