pub static LOCKED_BIT: u8 = 0;
pub static UNLOCKED_BIT: u8 = 1;
// pub static READERS_BIT: u8 = 2;

#[macro_export]
macro_rules! style_panic {
    ($($input:tt)*) => {
        panic!("[Rapidsync] A critical error occurred: {}\nPlease report this if it's unintended behavior", format!($($input)*))
    };
}