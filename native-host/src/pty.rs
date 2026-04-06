use std::env;
use std::io::{Read, Write};

use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    child: Box<dyn Child + Send>,
    writer: Option<Box<dyn Write + Send>>,
}

impl PtySession {
    pub fn new(cols: u16, rows: u16) -> Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
        let mut command = CommandBuilder::new(shell);
        command.env("TERM", "xterm-256color");

        let child = pair
            .slave
            .spawn_command(command)
            .context("failed to spawn shell in PTY")?;
        let writer = pair.master.take_writer().context("failed to open PTY writer")?;

        Ok(Self {
            master: pair.master,
            child,
            writer: Some(writer),
        })
    }

    pub fn writer(&mut self) -> Result<Box<dyn Write + Send>> {
        self.writer.take().context("PTY writer already taken")
    }

    pub fn reader(&self) -> Result<Box<dyn Read + Send>> {
        self.master
            .try_clone_reader()
            .context("failed to clone PTY reader")
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        Ok(())
    }

    pub fn wait(&mut self) -> Result<i32> {
        let status = self.child.wait().context("failed waiting for shell")?;
        Ok(status.exit_code() as i32)
    }
}
