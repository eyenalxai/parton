use anyhow::Result;
use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub fn spawn_and_wait(cmd: Command) -> Result<()> {
    spawn_and_wait_wine(cmd, None, None)
}

pub fn spawn_and_wait_wine(
    mut cmd: Command,
    wine64: Option<&Path>,
    exe_to_kill: Option<&str>,
) -> Result<()> {
    unsafe {
        cmd.pre_exec(|| {
            nix::unistd::setpgid(Pid::from_raw(0), Pid::from_raw(0))?;
            Ok(())
        });
    }

    let mut child = cmd.spawn()?;
    let pid = Pid::from_raw(child.id() as i32);

    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = Arc::clone(&interrupted);
    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    })?;

    loop {
        match child.try_wait()? {
            Some(_) => return Ok(()),
            None if interrupted.load(Ordering::SeqCst) => {
                if let (Some(wine), Some(exe)) = (wine64, exe_to_kill) {
                    let _ = Command::new(wine)
                        .args(["taskkill", "/F", "/IM", exe])
                        .output();
                }
                let _ = kill(Pid::from_raw(-pid.as_raw()), Signal::SIGTERM);
                std::thread::sleep(Duration::from_millis(500));
                let _ = kill(Pid::from_raw(-pid.as_raw()), Signal::SIGKILL);
                let _ = child.wait();
                return Ok(());
            }
            None => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}
