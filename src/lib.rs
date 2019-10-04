pub trait LargestUnit {
    fn largest_unit(&self, precision: u8, unit: &str) -> String;
    fn largest_byte_unit(&self, precision: u8) -> String;
}

impl LargestUnit for f32 {
    fn largest_unit(&self, precision: u8, unit: &str) -> String {
        String::new()
    }

    fn largest_byte_unit(&self, precision: u8) -> String {
        String::new()
    }
}
impl LargestUnit for f64 {
    fn largest_unit(&self, precision: u8, unit: &str) -> String {
        String::new()
    }

    fn largest_byte_unit(&self, precision: u8) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kilometer_test() {
        assert_eq!(1_129.0.largest_unit(2, "m"), "1.13 km");
    }

    #[test]
    fn gibibyte_test() {
        assert_eq!(2_635_000_987.0.largest_byte_unit(1), "2.6 GiB");
    }
}
