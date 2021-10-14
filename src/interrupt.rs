use alloc::boxed::Box;
pub type InterruptHandle = Box<dyn Fn() + Send + Sync>;