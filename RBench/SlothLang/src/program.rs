pub type UByte = u8;

pub struct SlothProgram {
    pub codes: Vec<UByte>,
    pub pc: usize,
}
