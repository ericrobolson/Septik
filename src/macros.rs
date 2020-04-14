#[macro_export]
macro_rules! div{
    ($( $t:tt ),* ) => {
            $(
            $t
            )*
        };
}
