use chrono::prelude::*;

#[macro_export]
macro_rules! logformat {
    ($text:expr, $status:expr) =>{
        format!("[{} | {}] >{}", crate::macros::log_macros::_now(), $status, $text)
    };
}

/// this function is a helper function for returning the current date
pub fn _now() -> String{
    let date = Local::now();
    let formated = date.format("%H:%M:%S");
    format!("{}", formated)
}
