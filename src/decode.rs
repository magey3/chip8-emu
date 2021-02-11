use crate::*;

pub fn decode(state: &State, opcode: u16) -> OpCode {
	let code = (
		(opcode & 0x00f0) >> 4,
		opcode & 0x000f,
		(opcode & 0xf000) >> 12,
		(opcode & 0x0f00) >> 8,
	);
	match code {
		(0, 0, 0xE, 0) => OpCode::ClearScreen,
		(0, 0, 0xE, 0xE) => OpCode::SubroutineRet,
		(0, _, _, _) => OpCode::CallMCodeSubroutine(code.3 | (code.2 << 4) | (code.1 << 8)),
		(1, _, _, _) => OpCode::Goto(code.3 | (code.2 << 4) | (code.1 << 8)),
		(2, _, _, _) => OpCode::Call(code.3 | (code.2 << 4) | (code.1 << 8)),
		(3, _, _, _) => OpCode::SkipNextIfEqRegN {
			vx: code.1 as u8,
			n: ((code.2 << 4) | code.3) as u8,
		},
		(4, _, _, _) => OpCode::SkipNextIfNotEqRegN {
			vx: code.1 as u8,
			n: ((code.2 << 4) | code.3) as u8,
		},
		(5, _, _, _) => OpCode::SkipNextIfEqRegReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(6, _, _, _) => OpCode::SetRegToN {
			vx: code.1 as u8,
			n: ((code.2 << 4) | code.3) as u8,
		},
		(7, _, _, _) => OpCode::AddNToRegNoCarry {
			vx: code.1 as u8,
			n: ((code.2 << 4) | code.3) as u8,
		},
		(8, _, _, 0) => OpCode::SetRegToReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 1) => OpCode::SetRegToRegOrReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 2) => OpCode::SetRegToRegAndReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 3) => OpCode::SetRegToRegXorReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 4) => OpCode::AddRegToReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 5) => OpCode::SubtractRegFromReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 6) => OpCode::StoreLeastSigBitAndRightShift { vx: code.1 as u8 },
		(8, _, _, 7) => OpCode::SubtractRegFromRegAndStoreInReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(8, _, _, 0xE) => OpCode::StoreMostSigBitAndLeftShift { vx: code.1 as u8 },
		(9, _, _, 0) => OpCode::SkipNextIfNotEqRegReg {
			vx: code.1 as u8,
			vy: code.2 as u8,
		},
		(0xA, _, _, _) => OpCode::SetIndexRegToN(code.3 | (code.2 << 4) | (code.1 << 8)),
		(0xB, _, _, _) => OpCode::JumpToAddrNPlusV0(code.3 | (code.2 << 4) | (code.1 << 8)),
		(0xC, _, _, _) => OpCode::Rand {
			vx: code.1 as u8,
			n: ((code.2 << 4) | code.3) as u8,
		},
		(0xD, _, _, _) => OpCode::DrawSprite {
			vx: code.1 as u8,
			vy: code.2 as u8,
			height: code.3 as u8,
		},
		(0xE, _, 9, 0xE) => OpCode::SkipNextIfKeyPressed(code.1 as u8),
		(0xE, _, 0xA, 1) => OpCode::SkipNextIfNotPressed(code.1 as u8),
		(0xF, _, 0, 7) => OpCode::GetDelayTimerValue(code.1 as u8),
		(0xF, _, 0, 0xA) => OpCode::GetKey(code.1 as u8),
		(0xF, _, 1, 5) => OpCode::SetDelayTimerValue(code.1 as u8),
		(0xF, _, 1, 8) => OpCode::SetSoundTimerValue(code.1 as u8),
		(0xF, _, 1, 0xE) => OpCode::AddRegToIndexReg(code.1 as u8),
		(0xF, _, 2, 9) => OpCode::SetIndexToSpriteLocation(code.1 as u8),
		(0xF, _, 3, 3) => OpCode::BinaryCodedDecimalConversion(code.1 as u8),
		(0xF, _, 5, 5) => OpCode::StoreV0ToVXToAddrAtIndex(code.1 as u8),
		(0xF, _, 6, 5) => OpCode::LoadV0ToVXFromAddrAtIndex(code.1 as u8),
		(_, _, _, _) => panic!(
			"Unsupported opcode({:#x}) at {:#x}",
			opcode, state.program_counter
		),
	}
}
