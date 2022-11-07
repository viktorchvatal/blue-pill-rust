pub trait Hx1230Driver {
    /// Send data to display
    fn data(&mut self, data: &[u8]) -> Result<(), ()>;

    /// Send commands to display
    fn commands(&mut self, commands: &[u8]) -> Result<(), ()>;

    /// Send a single command
    fn command(&mut self, command: u8) -> Result<(), ()> {
        self.commands(&[command])
    }
}

