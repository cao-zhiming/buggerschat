
#[macro_export]
macro_rules! fatalf {
    ($($args: tt)*) => {
        {
            use crossterm::style::Stylize;
            eprintln!("{}", format!("[FATAL] {}", format!($($args)*)).red().bold());
            std::process::exit(-1);
        }
    };
}

#[macro_export]
macro_rules! infof {
    ($($args: tt)*) => {
        {
            use crossterm::style::Stylize;
            eprintln!("{}", format!("[INFO] {}", format!($($args)*)).green().bold());
        }
    };
}

#[macro_export]
macro_rules! warnf {
    ($($args: tt)*) => {
        {
            use crossterm::style::Stylize;
            eprintln!("{}", format!("[WARNING] {}", format!($($args)*)).yellow().bold());
        }
    };
}

