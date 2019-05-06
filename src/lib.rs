use chrono::Local;
pub use log::{debug as debugln, info as println, Level};
use log::{Log, Metadata, Record};

pub const LTIME: u8 = 0b0000_0001;
pub const LPATH: u8 = 0b0000_0010;

struct Rog {
    lvl: Level,
    fmt: u8,
}

impl Log for Rog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut prefix = String::new();
            if self.fmt & LTIME != 0 {
                prefix.push_str(format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S")).as_str());
            }
            if self.fmt & LPATH != 0 {
                prefix.push_str(&record.module_path().unwrap_or_default());
            }
            std::println!("{} {}", prefix, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init(lvl: Level, fmt: u8) {
    let rog = Rog { lvl, fmt };
    let _ = log::set_boxed_logger(Box::new(rog));
    log::set_max_level(lvl.to_level_filter());
}
