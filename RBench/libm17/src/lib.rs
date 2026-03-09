pub mod decode;
pub mod encode;
pub mod math;
pub mod payload;
pub mod phy;
pub mod types;

pub fn send_data(_out: &mut [f32], _cnt: &mut u32, _input: &[u8]) {
    todo!()
}

pub fn send_eot(_out: &mut [f32], _cnt: &mut u32) {
    todo!()
}

pub fn send_preamble(_out: &mut [f32], _cnt: &mut u32, _ptype: u8) {
    todo!()
}

pub fn send_syncword(_out: &mut [f32], _cnt: &mut u32, _syncword: u16) {
    todo!()
}

pub fn send_frame(
    _out: &mut [f32],
    _data: &[u8],
    _frame_type: u8,
    _lsf: &[u8],
    _lich_cnt: u8,
    _fn_num: u16,
) {
    todo!()
}
