mod protocol;
mod pty;

use std::io::{self, ErrorKind, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};

use crate::protocol::{read_message, write_message};
use crate::pty::PtySession;

fn send_message(stdout: &Arc<Mutex<io::Stdout>>, message: &Value) -> io::Result<()> {
    let mut stdout = stdout.lock().expect("stdout mutex poisoned");
    write_message(&mut *stdout, message)
}

fn send_error(stdout: &Arc<Mutex<io::Stdout>>, message: impl Into<String>) {
    let _ = send_message(
        stdout,
        &json!({
            "type": "error",
            "message": message.into(),
        }),
    );
}

fn parse_dimension(message: &Value, field: &str) -> Result<u16> {
    let value = message
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("missing or invalid {field}"))?;

    if value == 0 {
        return Err(anyhow!("{field} must be greater than zero"));
    }

    Ok(value.min(u16::MAX as u64) as u16)
}

fn handle_message(
    message: Value,
    writer: &Arc<Mutex<Box<dyn Write + Send>>>,
    session: &Arc<Mutex<PtySession>>,
) -> Result<()> {
    let message_type = message
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing message type"))?;

    match message_type {
        "input" => {
            let data = message
                .get("data")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("input message requires string data"))?;

            let mut writer = writer.lock().expect("writer mutex poisoned");
            writer.write_all(data.as_bytes())?;
            writer.flush()?;
        }
        "resize" => {
            let cols = parse_dimension(&message, "cols")?;
            let rows = parse_dimension(&message, "rows")?;
            let session = session.lock().expect("session mutex poisoned");
            session.resize(cols, rows)?;
        }
        other => {
            return Err(anyhow!("unsupported message type: {other}"));
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut session = PtySession::new(80, 24)?;
    let mut reader = session.reader()?;
    let writer = Arc::new(Mutex::new(session.writer()?));
    let session = Arc::new(Mutex::new(session));
    let stdout = Arc::new(Mutex::new(io::stdout()));

    {
        let stdout = Arc::clone(&stdout);
        let session = Arc::clone(&session);

        thread::spawn(move || {
            let mut buffer = [0_u8; 4096];

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(count) => {
                        let payload = STANDARD.encode(&buffer[..count]);
                        if send_message(&stdout, &json!({ "type": "output", "data": payload })).is_err() {
                            return;
                        }
                    }
                    Err(error) if error.kind() == ErrorKind::Interrupted => continue,
                    Err(error) => {
                        send_error(&stdout, format!("PTY read failed: {error}"));
                        break;
                    }
                }
            }

            let exit_code = match session.lock().expect("session mutex poisoned").wait() {
                Ok(code) => code,
                Err(error) => {
                    send_error(&stdout, error.to_string());
                    0
                }
            };

            let _ = send_message(&stdout, &json!({ "type": "exit", "code": exit_code }));
        });
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        match read_message(&mut stdin) {
            Ok(message) => {
                if let Err(error) = handle_message(message, &writer, &session) {
                    send_error(&stdout, error.to_string());
                }
            }
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
            Err(error) if error.kind() == ErrorKind::InvalidData => {
                send_error(&stdout, format!("invalid message: {error}"));
            }
            Err(error) => {
                send_error(&stdout, format!("stdin read failed: {error}"));
                break;
            }
        }
    }

    Ok(())
}
