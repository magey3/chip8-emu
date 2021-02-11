use crate::opcode::*;
use crate::*;
use rand::Rng;
use util::get_bit;

pub fn execute(state: &mut State, opcode: OpCode) {
	match opcode {
		OpCode::CallMCodeSubroutine(n) => {
			eprintln!("Machine code subroutines are not supported");
		}
		//Clear screen
		OpCode::ClearScreen => {
			state.screen = [false; 64 * 32];
		}
		//Subroutine return
		OpCode::SubroutineRet => {
			state.program_counter = state.stack.last().copied().expect(&format!(
				"Subroutine is getting returned, but there is none to return from at {:#x}",
				state.program_counter
			));
			state.stack.pop();
		}
		//Goto
		OpCode::Goto(x) => {
			state.program_counter = x;
		}
		//Call subroutine
		OpCode::Call(x) => {
			state.stack.push(state.program_counter);
			state.program_counter = x;
		}
		//Skip instruction if vX == n
		OpCode::SkipNextIfEqRegN { vx, n } => {
			if state.reg[vx as usize] == n {
				state.program_counter += 2;
			}
		}
		//Skip instruction if vX != n
		OpCode::SkipNextIfNotEqRegN { vx, n } => {
			if state.reg[vx as usize] != n {
				state.program_counter += 2;
			}
		}
		//Skip instruction if vX == vY
		OpCode::SkipNextIfEqRegReg { vx, vy } => {
			if state.reg[vx as usize] == state.reg[vy as usize] {
				state.program_counter += 2;
			}
		}
		//Set vX to N
		OpCode::SetRegToN { vx, n } => {
			state.reg[vx as usize] = n;
		}
		//Add N to vX (carry flag not changed)
		OpCode::AddNToRegNoCarry { vx, n } => {
			state.reg[vx as usize] += n;
		}
		//Set vX to vY
		OpCode::SetRegToReg { vx, vy } => {
			state.reg[vx as usize] = vy;
		}
		//Set vX to vY OR Vx (bitwise OR)
		OpCode::SetRegToRegOrReg { vx, vy } => {
			state.reg[vx as usize] = state.reg[vx as usize] | state.reg[vy as usize];
		}
		//Set vX to vY AND Vx (bitwise AND)
		OpCode::SetRegToRegAndReg { vx, vy } => {
			state.reg[vx as usize] = state.reg[vx as usize] & state.reg[vy as usize];
		}
		//Set vX to vY XOR Vx (bitwise XOR)
		OpCode::SetRegToRegXorReg { vx, vy } => {
			state.reg[vx as usize] = state.reg[vx as usize] ^ state.reg[vy as usize];
		}
		//Add vY to vX and set v15 if carry
		OpCode::AddRegToReg { vx, vy } => {
			let x = state.reg[vx as usize].overflowing_add(state.reg[vy as usize]);
			state.reg[vx as usize] = x.0;
			state.reg[15] = if x.1 { 1 } else { 0 };
		}
		//Subtract vY from vX and set v15 if borrow
		OpCode::SubtractRegFromReg { vx, vy } => {
			let x = state.reg[vx as usize].overflowing_sub(state.reg[vy as usize]);
			state.reg[vx as usize] = x.0;
			state.reg[15] = if x.1 { 1 } else { 0 };
		}
		//Stores least significant bit in v15 and right shifts
		//TODO MAKE TEST
		OpCode::StoreLeastSigBitAndRightShift { vx } => {
			let x = state.reg[vx as usize].overflowing_shr(1).0;
			state.reg[15] = (state.reg[vx as usize] & (1 << 0) != 0) as u8;
			state.reg[vx as usize] = x;
		}
		// vX = vY - vX
		OpCode::SubtractRegFromRegAndStoreInReg { vx, vy } => {
			let x = state.reg[vy as usize].overflowing_sub(state.reg[vx as usize]);
			state.reg[vx as usize] = x.0;
			state.reg[15] = if x.1 { 1 } else { 0 };
		}
		//Stores most significant bit in v15 and left shift
		//TODO MAKE TEST
		OpCode::StoreMostSigBitAndLeftShift { vx } => {
			let x = state.reg[vx as usize].overflowing_shl(1).0;
			state.reg[15] = (state.reg[vx as usize] & (1 << 7) != 0) as u8;
			state.reg[vx as usize] = x;
		}
		//Skip next instruction if vX != vY
		OpCode::SkipNextIfNotEqRegReg { vx, vy } => {
			if state.reg[vx as usize] != state.reg[vy as usize] {
				state.program_counter += 2;
			}
		}
		//Sets index_reg to N
		OpCode::SetIndexRegToN(n) => {
			state.index_reg = n;
		}
		//Jumps to N + v0
		OpCode::JumpToAddrNPlusV0(n) => {
			state.program_counter = n + state.reg[0] as u16;
		}
		//Sets vX to to N & rand
		OpCode::Rand { vx, n } => {
			state.reg[vx as usize] = rand::thread_rng().gen_range(0..255) & n;
		}
		//Display sprite at vX, vY with width of 8px and height of height+1px
		OpCode::DrawSprite { vx, vy, height } => {
			let sprite = &state.memory
				[state.index_reg as usize..(height as usize + state.index_reg as usize)];
			let x = (state.reg[vx as usize] % 64) as u16;
			let y = (state.reg[vy as usize] % 32) as u16;
			state.reg[15] = 0;
			for (i, item) in sprite.iter().enumerate() {
				let bits = [
					get_bit(item.to_owned(), 7),
					get_bit(item.to_owned(), 6),
					get_bit(item.to_owned(), 5),
					get_bit(item.to_owned(), 4),
					get_bit(item.to_owned(), 3),
					get_bit(item.to_owned(), 2),
					get_bit(item.to_owned(), 1),
					get_bit(item.to_owned(), 0),
				];

				if y + i as u16 > 32 {
					break;
				}

				for (j, item) in bits.iter().enumerate() {
					if x + j as u16 > 64 {
						break;
					}
					let index: usize = ((x + j as u16) + (y + i as u16) * 64) as usize;
					if item.to_owned() {
						state.screen[index] = !state.screen[index];
					}
					if state.screen[index] == false {
						state.reg[15] = 1;
					}
				}
			}
		}
		OpCode::SkipNextIfKeyPressed(key) => {
			//TODO
		}
		OpCode::SkipNextIfNotPressed(key) => {
			//TODO
		}
		OpCode::GetDelayTimerValue(vx) => {
			state.reg[vx as usize] = state.delay_timer;
		}
		OpCode::GetKey(vx) => {
			//TODO
		}
		OpCode::SetDelayTimerValue(vx) => {
			state.delay_timer = state.reg[vx as usize];
		}
		OpCode::SetSoundTimerValue(vx) => {
			state.sound_timer = state.reg[vx as usize];
		}
		OpCode::AddRegToIndexReg(vx) => {
			state.index_reg += state.reg[vx as usize] as u16;
		}
		OpCode::SetIndexToSpriteLocation(vx) => {
			//TODO
		}
		OpCode::BinaryCodedDecimalConversion(vx) => {
			state.memory[state.index_reg as usize] = state.reg[vx as usize] / 100;
			state.memory[state.index_reg as usize + 1] = (state.reg[vx as usize] % 100) / 10;
			state.memory[state.index_reg as usize + 2] = state.reg[vx as usize] % 10;
		}
		OpCode::StoreV0ToVXToAddrAtIndex(vx) => {
			let mut index = state.index_reg as usize;
			for i in 0..vx + 1 {
				state.memory[index] = state.reg[i as usize];
				index += 1;
			}
		}
		OpCode::LoadV0ToVXFromAddrAtIndex(vx) => {
			let mut index = state.index_reg as usize;
			for i in 0..vx + 1 {
				state.reg[i as usize] = state.memory[index];
				index += 1;
			}
		}
		_ => panic!("Unsupported opcode at {:#x}", state.program_counter),
	}
}
