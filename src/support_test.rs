use crate::config::{self, Config};
use crate::log::{self, Ctx};
use serde_json::Value;
use std::io::Write;
use std::sync::{Arc, Mutex};

static LOCK: Mutex<()> = Mutex::new(());
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone, Default)]
pub struct Buf(Arc<Mutex<Vec<u8>>>);

impl Write for Buf {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Buf {
    type Writer = Buf;

    fn make_writer(&'a self) -> Buf {
        self.clone()
    }
}

pub fn capture(level: LevelFilter, f: impl FnOnce()) -> Vec<Value> {
    let _guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let buf = Buf::default();
    tracing::subscriber::with_default(log::subscriber(level, buf.clone()), f);
    let text = String::from_utf8(buf.0.lock().unwrap().clone()).unwrap();
    text.lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

pub fn cfg() -> Config {
    config::embedded()
}

pub fn ctx() -> Ctx {
    Ctx {
        request_id: "rid".into(),
    }
}
