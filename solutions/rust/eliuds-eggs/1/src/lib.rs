pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .filter(|&i| display_value >> i & 0b00000000_00000000_00000000_00000001 == 1)
        .count()
}
