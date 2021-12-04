use crate::command::set_position;

pub trait Hx1230Driver {
    fn clear_data(&mut self) -> Result<(), ()> {
        self.send_commands(&set_position(0, 0))?;

        for _ in 0..12*9 { 
            self.send_data(&[0; 8])?; 
        }

        self.send_commands(&set_position(0, 0))
    }

    #[inline(never)]
    fn command(&mut self, command: u8) -> Result<(), ()> {
        self.send_commands(&[command])
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), ()>;

    fn send_commands(&mut self, commands: &[u8]) -> Result<(), ()>;
}

