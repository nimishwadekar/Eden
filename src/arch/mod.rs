#[cfg(feature = "x86_64")]
pub mod x86_64;

#[cfg(feature = "x86_64")]
pub fn init() {
    x86_64::init()
}