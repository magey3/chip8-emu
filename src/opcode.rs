#[derive(Debug)]
pub enum OpCode {
	CallMCodeSubroutine(u16),
	ClearScreen,
	SubroutineRet,
	Goto(u16),
	Call(u16),
	SkipNextIfEqRegN {
		vx: u8,
		n: u8,
	},
	SkipNextIfNotEqRegN {
		vx: u8,
		n: u8,
	},
	SkipNextIfEqRegReg {
		vx: u8,
		vy: u8,
	},
	SetRegToN {
		vx: u8,
		n: u8,
	},
	AddNToRegNoCarry {
		vx: u8,
		n: u8,
	},
	SetRegToReg {
		vx: u8,
		vy: u8,
	},
	SetRegToRegOrReg {
		vx: u8,
		vy: u8,
	},
	SetRegToRegAndReg {
		vx: u8,
		vy: u8,
	},
	SetRegToRegXorReg {
		vx: u8,
		vy: u8,
	},
	AddRegToReg {
		vx: u8,
		vy: u8,
	},
	SubtractRegFromReg {
		vx: u8,
		vy: u8,
	},
	StoreLeastSigBitAndRightShift {
		vx: u8,
	},
	///vX = vY - vX
	SubtractRegFromRegAndStoreInReg {
		vy: u8,
		vx: u8,
	},
	StoreMostSigBitAndLeftShift {
		vx: u8,
	},
	SkipNextIfNotEqRegReg {
		vx: u8,
		vy: u8,
	},
	SetIndexRegToN(u16),
	JumpToAddrNPlusV0(u16),
	Rand {
		vx: u8,
		n: u8,
	},
	DrawSprite {
		vx: u8,
		vy: u8,
		height: u8,
	},
	SkipNextIfKeyPressed(u8),
	SkipNextIfNotPressed(u8),
	GetDelayTimerValue(u8),
	GetKey(u8),
	SetDelayTimerValue(u8),
	SetSoundTimerValue(u8),
	AddRegToIndexReg(u8),
	SetIndexToSpriteLocation(u8),
	BinaryCodedDecimalConversion(u8),
	StoreV0ToVXToAddrAtIndex(u8),
	LoadV0ToVXFromAddrAtIndex(u8),
}
