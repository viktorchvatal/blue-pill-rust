/// Minimalistic result only containing information whether a function
/// succeeded or not
pub type MiniResult = Result<(), ()>;

pub trait ResultExt {
    /// Check if there is an error in result and panic if there is,
    /// but without any formatted message so it does not generate formatting
    /// code
    fn check(&self);
}

impl<T, E> ResultExt for Result<T, E> {
    fn check(&self) {
        if self.is_err() {
            panic!();
        }
    }
}