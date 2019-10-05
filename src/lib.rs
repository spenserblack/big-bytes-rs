pub const AVAILABLE_UNIT_SIZES: [Option<char>;5] = [
    None,
    Some('K'),
    Some('M'),
    Some('G'),
    Some('T'),
];

pub trait LargestByteUnit {
    fn largest_byte_unit(&self, precision: usize) -> String;
}

impl LargestByteUnit for f32 {
    fn largest_byte_unit(&self, precision: usize) -> String {
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
impl LargestByteUnit for f64 {
    fn largest_byte_unit(&self, precision: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_test() {
        assert_eq!(2.001.largest_byte_unit(3), "2.001 B");
    }

    #[test]
    fn gibibyte_test() {
        assert_eq!(2_635_000_987.0.largest_byte_unit(1), "2.5 GiB");
    }

    #[test]
    fn too_big_test() {
        assert_eq!(2_635_000_987_000_000.0.largest_byte_unit(1), "2396.5 TiB");
    }
}
