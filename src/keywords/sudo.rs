#[macro_export]
macro_rules! sudo {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}