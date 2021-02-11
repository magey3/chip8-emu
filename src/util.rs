pub fn get_bit(input: u8, bit: u8) -> bool {
	if bit < 8 {
		input & (1 << bit) != 0
	} else {
		false
	}
}
