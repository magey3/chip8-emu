use crate::*;

pub fn load_program(state: &mut State, path: &str) {
	let f = std::fs::read(path).expect("Couldn't read file");
	let mut i = 0;
	while i < f.len() {
		state.memory[i + 0x200] = f[i];
		state.memory[i + 0x201] = f[i + 1];
		i += 2;
	}
}
