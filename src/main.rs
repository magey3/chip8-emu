mod decode;
mod execute;
mod opcode;

use crate::decode::*;
use crate::execute::*;
use crate::opcode::*;

#[cfg(test)]
mod test {
	use crate::decode::*;
	use crate::execute::*;
	use crate::opcode::*;
	use crate::*;

	#[test]
	fn subroutines() {
		let mut cpu = State::new();
		assert_eq!(cpu.program_counter, 0x200);
		let x = decode(&cpu, 0x2fff);
		execute(&mut cpu, x);
		assert_eq!(0xfff, cpu.program_counter);
		let x = decode(&cpu, 0x00ee);
		execute(&mut cpu, x);
		assert_eq!(cpu.program_counter, 0x200);
	}

	#[test]
	fn goto() {
		let mut cpu = State::new();
		assert_eq!(cpu.program_counter, 0x200);
		let x = decode(&cpu, 0x1fff);
		execute(&mut cpu, x);
		assert_eq!(0xfff, cpu.program_counter);
	}

	#[test]
	fn skip_next_op() {
		let mut cpu = State::new();
		let x = decode(&cpu, 0x30ff);
		cpu.reg[0] = 0xff;
		assert_eq!(cpu.program_counter, 0x200);
		execute(&mut cpu, x);
		assert_eq!(cpu.program_counter, 0x200 + 1);
		let x = decode(&cpu, 0x4000);
		execute(&mut cpu, x);
		assert_eq!(cpu.program_counter, 0x200 + 2);
		cpu.reg[1] = 0xff;
		let x = decode(&cpu, 0x5010);
		execute(&mut cpu, x);
		assert_eq!(cpu.program_counter, 0x200 + 3);
	}

	#[test]
	fn set_reg() {
		let mut cpu = State::new();
		let x = decode(&cpu, 0x60ff);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 0xff);
		cpu.reg[0] = 0;
		cpu.reg[1] = 1;
		let x = decode(&cpu, 0x8010);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], cpu.reg[1]);
	}

	#[test]
	fn add_to_reg() {
		let mut cpu = State::new();
		cpu.reg[0] = 0;
		let x = decode(&cpu, 0x70ff);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 0xff);

		cpu.reg[0] = 5;
		cpu.reg[1] = 10;
		let x = decode(&cpu, 0x8014);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 15);
		let x = decode(&cpu, 0x8014);
		cpu.reg[1] = 255;
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[15], 1);
	}

	#[test]
	fn subtract_from_reg() {
		let mut cpu = State::new();
		cpu.reg[0] = 20;
		cpu.reg[1] = 5;
		let x = decode(&cpu, 0x8015);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 15);
		cpu.reg[0] = 0;
		cpu.reg[1] = 5;
		let x = decode(&cpu, 0x8015);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[15], 1);

		cpu.reg[0] = 2;
		cpu.reg[1] = 3;
		let x = decode(&cpu, 0x8017);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 1);

		cpu.reg[0] = 5;
		cpu.reg[1] = 0;
		let x = decode(&cpu, 0x8017);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[15], 1);
	}

	#[test]
	fn set_reg_or_and_xor() {
		let mut cpu = State::new();
		cpu.reg[0] = 0xe;
		cpu.reg[1] = 0x4;
		let x = decode(&cpu, 0x8011);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 0xe | 0x4);
		cpu.reg[0] = 0xe;
		cpu.reg[1] = 0x4;
		let x = decode(&cpu, 0x8012);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 0xe & 0x4);
		cpu.reg[0] = 0xe;
		cpu.reg[1] = 0x4;
		let x = decode(&cpu, 0x8013);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], 0xe ^ 0x4);
	}

	#[test]
	fn set_index_reg() {
		let mut cpu = State::new();
		cpu.index_reg = 0;
		let x = decode(&cpu, 0xa123);
		execute(&mut cpu, x);
		assert_eq!(cpu.index_reg, 0x123);
	}

	#[test]
	fn jump_to_n_plus_v0() {
		let mut cpu = State::new();
		cpu.index_reg = 0;
		cpu.reg[0] = 0xff;
		let x = decode(&cpu, 0xb123);
		execute(&mut cpu, x);
		assert_eq!(cpu.program_counter, 0xff + 0x123);
	}

	#[test]
	fn rand() {
		let mut cpu = State::new();
		cpu.reg[0] = 0;
		let x = decode(&cpu, 0xC00f);
		execute(&mut cpu, x);
		assert_eq!(cpu.reg[0], cpu.reg[0] & 0x0f);
	}
}

pub struct State {
	pub memory: [u8; 4096],
	pub reg: [u8; 16],
	pub index_reg: u16,
	pub program_counter: u16,
	pub screen: [bool; 64 * 32],
	pub delay_timer: u8,
	pub sound_timer: u8,
	pub stack: Vec<u16>,
	pub stack_poiner: u16,
}
impl State {
	fn new() -> Self {
		Self {
			memory: [0; 4096],
			reg: [0; 16],
			screen: [false; 64 * 32],
			stack: Vec::new(),
			program_counter: 0x200,
			index_reg: 0,
			delay_timer: 0,
			sound_timer: 0,
			stack_poiner: 0,
		}
	}
}

fn fetch(state: &mut State) -> u16 {
	0
}

fn main() {}
