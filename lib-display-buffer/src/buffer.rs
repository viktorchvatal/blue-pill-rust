/// Display frame buffer, width is in pixels, height is is octets
pub struct ArrayDisplayBuffer<const W: usize, const H: usize> {
    pixels: [[u8; W]; H]
}

pub trait DisplayBuffer {
    fn width(&self) -> usize;
    fn line_count(&self) -> usize;
    fn get_line(&self, y: usize) -> Option<&[u8]>;
    fn get_line_mut(&mut self, y: usize) -> Option<&mut [u8]>;
}

impl<const W: usize, const H: usize> ArrayDisplayBuffer<W, H> {
    pub fn new() -> Self {
        Self {
            pixels: [[0; W]; H],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.pixels.get(y).and_then(|row| row.get(x)).cloned()
    }
}

impl<const W: usize, const H: usize> DisplayBuffer for ArrayDisplayBuffer<W, H> {
    fn get_line(&self, y: usize) -> Option<&[u8]> {
        self.pixels.get(y).map(|array| &array[..])
    }

    fn get_line_mut(&mut self, y: usize) -> Option<&mut [u8]> {
        self.pixels.get_mut(y).map(|array| &mut array[..])
    }

    fn width(&self) -> usize {
        W
    }

    fn line_count(&self) -> usize {
        H
    }
}