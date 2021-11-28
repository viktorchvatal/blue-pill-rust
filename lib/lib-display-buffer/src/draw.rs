use crate::DisplayBuffer;

#[inline(never)]
pub fn clear(buf: &mut dyn DisplayBuffer, value: u8) {
    for line_id in 0..buf.line_count() {
        if let Some(ref mut line) = buf.get_line_mut(line_id) {
            line_clear(line, value)
        }
    }
}

#[inline(never)]
pub fn clear_pattern(buf: &mut dyn DisplayBuffer, pattern: &[u8]) {
    for line_id in 0..buf.line_count() {
        if let Some(ref mut line) = buf.get_line_mut(line_id) {
            line_set_pattern(line, pattern)
        }
    }
}

#[inline(never)]
fn line_clear(pixels: &mut [u8], value: u8) {
    pixels.iter_mut().for_each(|pixel| *pixel = value);
}

#[inline(never)]
fn line_set_pattern(pixels: &mut [u8], pattern: &[u8]) {
    pixels.iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| *pixel = pattern[index % pattern.len()]);
}