//! Helps represent bytes with the largest possible mutliple of the byte unit.
//!
//! # Example
//!
//! ```
//! use big_bytes::BigByte;
//!
//! let bytes = 2.456 * 1024_f32.powi(3);
//!
//! assert_eq!("2.46 GiB", bytes.big_byte(2));
//! ```

/// Available multiples of the byte unit.
///
/// *`'K'` Represents __Kibi__, not __Kilo__*.
pub const AVAILABLE_UNIT_SIZES: [Option<char>;9] = [
    None,
    Some('K'),
    Some('M'),
    Some('G'),
    Some('T'),
    Some('P'),
    Some('E'),
    Some('Z'),
    Some('Y'),
];

/// Makes a type representable as a byte count.
pub trait BigByte {
    /// Represents `self` as a byte count.
    fn big_byte(&self, precision: usize) -> String;
}

impl BigByte for f32 {
    fn big_byte(&self, precision: usize) -> String {
        let mut value: f32 = *self;
        let mut counter: usize = 0;
        let (counter, value): (usize, f32) = loop {
            if counter >= AVAILABLE_UNIT_SIZES.len() {
                break (counter, value);
            }
            if value < 1024.0 {
                break (counter, value);
            }
            value /= 1024.0;
            counter += 1;
        };

        match AVAILABLE_UNIT_SIZES.get(counter) {
            Some(None) => format!("{:.*} B", precision, value),
            Some(Some(c)) => format!("{:.*} {}iB", precision, value, c),
            None => format!("{:.*} {}iB", precision, value, AVAILABLE_UNIT_SIZES.last().unwrap().unwrap()),
        }
    }
}
impl BigByte for f64 {
    fn big_byte(&self, precision: usize) -> String {
        let mut value: f64 = *self;
        let mut counter: usize = 0;
        let (counter, value): (usize, f64) = loop {
            if counter + 1 >= AVAILABLE_UNIT_SIZES.len() {
                break (counter, value);
            }
            if value < 1024.0 {
                break (counter, value);
            }
            value /= 1024.0;
            counter += 1;
        };

        match AVAILABLE_UNIT_SIZES.get(counter) {
            Some(None) => format!("{:.*} B", precision, value),
            Some(Some(c)) => format!("{:.*} {}iB", precision, value, c),
            None => format!("{:.*} {}iB", precision, value, AVAILABLE_UNIT_SIZES.last().unwrap().unwrap()),
        }
    }
}

impl BigByte for u8 {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f32).big_byte(precision)
    }
}

impl BigByte for u16 {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f32).big_byte(precision)
    }
}

impl BigByte for u32 {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f32).big_byte(precision)
    }
}

impl BigByte for u64 {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f32).big_byte(precision)
    }
}

impl BigByte for u128 {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f64).big_byte(precision)
    }
}

impl BigByte for usize {
    fn big_byte(&self, precision: usize) -> String {
        (*self as f64).big_byte(precision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_test() {
        assert_eq!(2.001.big_byte(3), "2.001 B");
    }

    #[test]
    #[allow(non_snake_case)]
    fn u16_two_KiB_test() {
        assert_eq!(2048_u16.big_byte(0), "2 KiB");
    }

    #[test]
    fn gibibyte_test() {
        assert_eq!(2_635_000_987.0.big_byte(1), "2.5 GiB");
    }

    #[test]
    fn too_big_test() {
        let bytes = 2_635.0 * 1024_f64.powi(8);
        assert_eq!(bytes.big_byte(1), "2635.0 YiB");
    }
}
