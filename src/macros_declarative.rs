#[macro_export]
macro_rules! call_functions {
    ( $( $func:ident ),* ) => {
        (
            $(
                $func(),
            )*
        )
    };
}
