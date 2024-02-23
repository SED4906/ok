#[macro_export]
macro_rules! return_if {
    ($e:expr) => {
        if $e {
            return;
        }
    };
    ($e:expr, $d:expr) => {
        if $e {
            return $d;
        }
    };
}
