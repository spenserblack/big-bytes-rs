pub const AVAILABLE_UNIT_SIZES: [char;4] = ['K', 'M', 'G', 'T'];

pub trait LargestByteUnit {
    fn largest_byte_unit(&self, precision: u8) -> String;
}

impl LargestByteUnit for f32 {
    fn largest_byte_unit(&self, precision: u8) -> String {
        String::new()
    }
}
impl LargestByteUnit for f64 {
    fn largest_byte_unit(&self, precision: u8) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gibibyte_test() {
        assert_eq!(2_635_000_987.0.largest_byte_unit(1), "2.6 GiB");
    }
}
