#[macro_export]
macro_rules! log {
    (debug, $($arg:tt)*) => {
        // If this is a release do not log debug messages
        if(cfg!(not(debug_assertions))) {
            return;
        }
        println!(
            "[{}] {} {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            "[DEBUG]".bold().white(),
            format!($($arg)*)
        );
    };
    (info, $($arg:tt)*) => {
        println!(
            "[{}] {} {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            "[INFO]".bold().green(),
            format!($($arg)*)
        );
    };
    (warning, $($arg:tt)*) => {
        println!(
            "[{}] {} {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            "[WARNING]".bold().truecolor(255, 165, 0),
            format!($($arg)*)
        );
    };
    (error, $($arg:tt)*) => {
        println!(
            "[{}] {} {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            "[ERROR]".bold().red(),
            format!($($arg)*)
        );
    };
}
