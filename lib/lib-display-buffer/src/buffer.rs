use embedded_graphics_core::{prelude::{DrawTarget, Size, OriginDimensions}, pixelcolor::BinaryColor, Pixel};

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

impl<const W: usize, const H: usize> DrawTarget for ArrayDisplayBuffer<W, H> {
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < W as i32 && coord.y >= 0 && coord.y/8 < H as i32 {
                let line = coord.y as usize / 8;
                let column = coord.x as usize;
                let shift = (coord.y as usize) % 8;
    
                self.pixels[line][column] = self.pixels[line][column] 
                    & (!(1 << shift)) 
                    | ((color.is_on() as u8) << shift);        
            }
        }

        Ok(())
    }
}

impl<const W: usize, const H: usize> OriginDimensions for ArrayDisplayBuffer<W, H> {
    fn size(&self) -> Size {
        Size::new(W as u32, (H*8) as u32)
    }
}