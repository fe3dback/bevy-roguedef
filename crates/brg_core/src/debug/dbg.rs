#[macro_export]
macro_rules! dlog {
    () => {
        use ::std::io::Write;
        let content = ::std::format!("\n----------------------------------------------\n--- {}:{}\n----------------------------------------------\n", ::std::file!(), ::std::line!());
        ::std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("debug-dev-bevy-gm.dbg.log")
            .and_then(|mut f| f.write_all(content.into_bytes().as_ref()))
            .unwrap();
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                use ::std::io::Write;
                let content = ::std::format!("\n----------------------------------------------\n--- {}:{}\n----------------------------------------------\n {} = {:#?}\n",
                    ::std::file!(), ::std::line!(), ::std::stringify!($val), &tmp
                );
                ::std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("debug-dev-bevy-gm.dbg.log")
                    .and_then(|mut f| f.write_all(content.into_bytes().as_ref()))
                    .unwrap();

                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dlog!($val)),+,)
    };
}

pub fn clear_debug_log_file() {
    _ = ::std::fs::remove_file("debug-dev-bevy-gm.dbg.log");
}
