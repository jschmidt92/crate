use chrono::Utc;
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{LazyLock, Mutex, Once},
};

static LOG_FILE: LazyLock<Mutex<Option<File>>> = LazyLock::new(|| Mutex::new(open_log_file()));
static PANIC_HOOK: Once = Once::new();

pub fn init() {
    let path = path();
    info(format_args!("logger initialized at {}", path.display()));
    PANIC_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|panic_info| {
            error(format_args!("panic: {panic_info}"));
        }));
    });
}

pub fn info(args: std::fmt::Arguments<'_>) {
    write("INFO", args);
}

pub fn error(args: std::fmt::Arguments<'_>) {
    write("ERROR", args);
}

fn write(level: &str, args: std::fmt::Arguments<'_>) {
    let timestamp = Utc::now().to_rfc3339();
    let Ok(mut file) = LOG_FILE.lock() else {
        return;
    };
    let Some(file) = file.as_mut() else {
        return;
    };
    let _ = writeln!(file, "{timestamp} [{level}] {args}");
    let _ = file.flush();
}

fn open_log_file() -> Option<File> {
    let path = path();
    OpenOptions::new().create(true).append(true).open(path).ok()
}

pub fn path() -> PathBuf {
    env::var_os("FORGE_SERVER_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("forge_server.log"))
}
