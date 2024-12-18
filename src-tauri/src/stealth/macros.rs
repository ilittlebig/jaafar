#[macro_export]
macro_rules! value_or_undefined {
    ($condition:expr, $value:expr) => {
        if $condition {
            $value.to_string()
        } else {
            "undefined".to_string()
        }
    };
}

#[macro_export]
macro_rules! choose_string {
    ($condition:expr, $if_true:expr, $if_false:expr) => {
        if $condition {
            $if_true.to_string()
        } else {
            $if_false.to_string()
        }
    };
}
