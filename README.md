# big-bytes
Converts float to the largest possible multiple of the byte unit

## Example

```rust
use big_bytes::LargestByteUnit;

let bytes = 2.456 * 1024_f32.powi(3);

assert_eq!("2.46 GiB", bytes.largest_byte_unit(2));
```
