use chrono::Utc;
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    panic::Location,
    path::{Path, PathBuf},
    sync::{
        Arc, Once, OnceLock,
        atomic::{AtomicU64, Ordering},
        mpsc::{Receiver, SyncSender, TrySendError, sync_channel},
    },
    time::{Duration, Instant},
};

const LOG_CHANNEL_CAPACITY: usize = 8_192;
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
const AGGREGATE_LOG_NAME: &str = "forge_crate.log";

static LOGGER: OnceLock<AsyncLogger> = OnceLock::new();
static PANIC_HOOK: Once = Once::new();

struct AsyncLogger {
    sender: SyncSender<LogRecord>,
    dropped: Arc<AtomicU64>,
}

struct LogRecord {
    level: Level,
    domain: &'static str,
    message: String,
    source: &'static str,
    line: u32,
}

#[derive(Clone, Copy)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

pub fn init() {
    if LOGGER.get().is_none() {
        let aggregate_path = path();
        let directory = aggregate_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        let _ = fs::create_dir_all(&directory);
        let (sender, receiver) = sync_channel(LOG_CHANNEL_CAPACITY);
        let dropped = Arc::new(AtomicU64::new(0));
        let writer_dropped = Arc::clone(&dropped);
        let writer_directory = directory.clone();

        let writer = std::thread::Builder::new()
            .name("forge-log-writer".to_string())
            .spawn(move || writer_loop(writer_directory, aggregate_path, receiver, writer_dropped));

        if writer.is_ok() {
            let _ = LOGGER.set(AsyncLogger { sender, dropped });
        }
    }

    info_in(
        "framework",
        format_args!("logger initialized at {}", path().display()),
    );
    PANIC_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|panic_info| {
            error_in("framework", format_args!("panic: {panic_info}"));
        }));
    });
}

#[track_caller]
pub fn debug(args: std::fmt::Arguments<'_>) {
    submit(
        Level::Debug,
        domain_from_source(Location::caller().file()),
        args,
    );
}

#[track_caller]
pub fn info(args: std::fmt::Arguments<'_>) {
    submit(
        Level::Info,
        domain_from_source(Location::caller().file()),
        args,
    );
}

#[track_caller]
pub fn warn(args: std::fmt::Arguments<'_>) {
    submit(
        Level::Warn,
        domain_from_source(Location::caller().file()),
        args,
    );
}

#[track_caller]
pub fn error(args: std::fmt::Arguments<'_>) {
    submit(
        Level::Error,
        domain_from_source(Location::caller().file()),
        args,
    );
}

#[track_caller]
pub fn debug_in(domain: &'static str, args: std::fmt::Arguments<'_>) {
    submit(Level::Debug, domain, args);
}

#[track_caller]
pub fn info_in(domain: &'static str, args: std::fmt::Arguments<'_>) {
    submit(Level::Info, domain, args);
}

#[track_caller]
pub fn error_in(domain: &'static str, args: std::fmt::Arguments<'_>) {
    submit(Level::Error, domain, args);
}

#[track_caller]
fn submit(level: Level, domain: &'static str, args: std::fmt::Arguments<'_>) {
    let Some(logger) = LOGGER.get() else {
        return;
    };
    let caller = Location::caller();
    let record = LogRecord {
        level,
        domain,
        message: args.to_string(),
        source: caller.file(),
        line: caller.line(),
    };

    if let Err(error) = logger.sender.try_send(record) {
        match error {
            TrySendError::Full(_) | TrySendError::Disconnected(_) => {
                logger.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

fn writer_loop(
    directory: PathBuf,
    aggregate_path: PathBuf,
    receiver: Receiver<LogRecord>,
    dropped: Arc<AtomicU64>,
) {
    let Some(mut aggregate) = open_log_file(&aggregate_path) else {
        return;
    };
    let mut domain_files = HashMap::<&'static str, BufWriter<File>>::new();
    let mut last_flush = Instant::now();

    loop {
        match receiver.recv_timeout(FLUSH_INTERVAL) {
            Ok(record) => {
                write_record(&mut aggregate, &mut domain_files, &directory, record);
                while let Ok(record) = receiver.try_recv() {
                    write_record(&mut aggregate, &mut domain_files, &directory, record);
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }

        let dropped_count = dropped.swap(0, Ordering::Relaxed);
        if dropped_count > 0 {
            let timestamp = Utc::now().to_rfc3339();
            let _ = writeln!(
                aggregate,
                "{timestamp} [WARN] [framework] dropped {dropped_count} log record(s): channel full or disconnected"
            );
        }

        if last_flush.elapsed() >= FLUSH_INTERVAL {
            flush_files(&mut aggregate, &mut domain_files);
            last_flush = Instant::now();
        }
    }

    flush_files(&mut aggregate, &mut domain_files);
}

fn write_record(
    aggregate: &mut BufWriter<File>,
    domain_files: &mut HashMap<&'static str, BufWriter<File>>,
    directory: &Path,
    record: LogRecord,
) {
    let timestamp = Utc::now().to_rfc3339();
    let line = format!(
        "{timestamp} [{}] [{}] {} ({}:{})",
        record.level.as_str(),
        record.domain,
        record.message,
        record.source,
        record.line
    );
    let _ = writeln!(aggregate, "{line}");

    if !domain_files.contains_key(record.domain) {
        let path = directory.join(format!("{}.log", safe_domain(record.domain)));
        if let Some(file) = open_log_file(&path) {
            domain_files.insert(record.domain, file);
        }
    }
    if let Some(file) = domain_files.get_mut(record.domain) {
        let _ = writeln!(file, "{line}");
    }
}

fn flush_files(
    aggregate: &mut BufWriter<File>,
    domain_files: &mut HashMap<&'static str, BufWriter<File>>,
) {
    let _ = aggregate.flush();
    for file in domain_files.values_mut() {
        let _ = file.flush();
    }
}

fn open_log_file(path: &Path) -> Option<BufWriter<File>> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map(BufWriter::new)
        .ok()
}

fn safe_domain(domain: &str) -> String {
    domain
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' || character == '-' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn domain_from_source(source: &str) -> &'static str {
    let source = source.replace('\\', "/");
    if source.contains("/persistence/") {
        return "persistence";
    }
    if source.contains("/features/actor/") {
        return "actor";
    }
    if source.contains("/features/bank/") {
        return "bank";
    }

    match source.rsplit('/').next().unwrap_or_default() {
        "actor.rs" => "actor",
        "bank.rs" => "bank",
        "events.rs" => "events",
        "garage.rs" => "garage",
        "locker.rs" => "locker",
        "medical.rs" => "medical",
        "notification.rs" => "notification",
        "organization.rs" => "organization",
        "rearm.rs" => "rearm",
        "refuel.rs" => "refuel",
        "repair.rs" => "repair",
        "v_garage.rs" => "v_garage",
        "v_locker.rs" => "v_locker",
        _ => "framework",
    }
}

pub fn directory() -> PathBuf {
    if let Some(path) = env::var_os("FORGE_SERVER_LOG_DIR") {
        return PathBuf::from(path);
    }
    if let Some(path) = env::var_os("FORGE_SERVER_LOG") {
        let path = PathBuf::from(path);
        return path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
    }
    PathBuf::from("@forge_crate/logs")
}

pub fn path() -> PathBuf {
    env::var_os("FORGE_SERVER_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|| directory().join(AGGREGATE_LOG_NAME))
}

#[cfg(test)]
mod tests {
    use super::{Level, LogRecord, domain_from_source, safe_domain, writer_loop};
    use std::sync::{Arc, atomic::AtomicU64, mpsc::sync_channel};

    #[test]
    fn source_paths_route_to_expected_domains() {
        assert_eq!(domain_from_source("arma/crate/src/bank.rs"), "bank");
        assert_eq!(
            domain_from_source("arma/crate/src/persistence/service.rs"),
            "persistence"
        );
        assert_eq!(
            domain_from_source("arma/crate/src/features/actor/lifecycle.rs"),
            "actor"
        );
        assert_eq!(domain_from_source("arma/crate/src/lib.rs"), "framework");
    }

    #[test]
    fn domain_names_are_safe_file_names() {
        assert_eq!(safe_domain("v_garage"), "v_garage");
        assert_eq!(safe_domain("bad/domain"), "bad_domain");
    }

    #[test]
    fn writer_creates_aggregate_and_domain_logs() {
        let directory = std::env::temp_dir().join(format!("forge-log-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create test log directory");
        let aggregate = directory.join("forge_crate.log");
        let (sender, receiver) = sync_channel(4);
        let writer_directory = directory.clone();
        let writer_aggregate = aggregate.clone();
        let thread = std::thread::spawn(move || {
            writer_loop(
                writer_directory,
                writer_aggregate,
                receiver,
                Arc::new(AtomicU64::new(0)),
            );
        });

        sender
            .send(LogRecord {
                level: Level::Debug,
                domain: "bank",
                message: "cache pull key=test result=hit".to_string(),
                source: "repository.rs",
                line: 42,
            })
            .expect("send test record");
        drop(sender);
        thread.join().expect("join test writer");

        let aggregate_contents = std::fs::read_to_string(aggregate).expect("read aggregate log");
        let domain_contents =
            std::fs::read_to_string(directory.join("bank.log")).expect("read domain log");
        assert!(aggregate_contents.contains("[DEBUG] [bank]"));
        assert!(domain_contents.contains("cache pull key=test result=hit"));

        std::fs::remove_dir_all(directory).expect("remove test log directory");
    }
}
