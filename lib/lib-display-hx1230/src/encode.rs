use core::cmp::min;

#[inline(never)]
pub fn encode_control_bit(data: &[u8], output: &mut [u8; 9], bit: u8) -> usize {
    let data = &data[0..min(data.len(), 8)];
    let len = data.len();

    for shift in 0..len {
        output[shift] |= bit << (7 - shift);

        if shift == 7 {
            output[shift + 1] = data[shift];
        } else {
            output[shift] |= data[shift] >> (shift + 1);
            output[shift + 1] |= data[shift] << (7 - shift);
        }
    }

    if len == 8 { 9 } else { len + 1 }
}