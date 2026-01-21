use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsString;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

pub fn socket_path(appid: &str, exe_path: &Path) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    exe_path.to_string_lossy().hash(&mut hasher);
    let hash = hasher.finish();
    PathBuf::from(format!("/tmp/prex-{appid}-{hash:x}.sock"))
}

pub fn send_to_daemon(socket_path: &Path, args: &[OsString]) -> Result<()> {
    let mut stream = UnixStream::connect(socket_path)
        .with_context(|| format!("Failed to connect to {}", socket_path.display()))?;
    write_args(&mut stream, args)
}

pub fn bind_listener(socket_path: &Path) -> Result<UnixListener> {
    if let Some(parent) = socket_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let listener = UnixListener::bind(socket_path)
        .with_context(|| format!("Failed to bind {}", socket_path.display()))?;
    listener.set_nonblocking(true)?;
    Ok(listener)
}

pub fn try_receive_args(listener: &UnixListener) -> Result<Option<Vec<OsString>>> {
    match listener.accept() {
        Ok((mut stream, _addr)) => Ok(Some(read_args(&mut stream)?)),
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
        Err(err) => Err(err.into()),
    }
}

fn write_args(stream: &mut UnixStream, args: &[OsString]) -> Result<()> {
    let count = u32::try_from(args.len()).context("Too many arguments")?;
    stream.write_all(&count.to_le_bytes())?;
    for arg in args {
        let bytes = arg.as_bytes();
        let len = u32::try_from(bytes.len()).context("Argument too large")?;
        stream.write_all(&len.to_le_bytes())?;
        stream.write_all(bytes)?;
    }
    stream.flush()?;
    Ok(())
}

fn read_args(stream: &mut UnixStream) -> Result<Vec<OsString>> {
    let mut count_buf = [0u8; 4];
    stream.read_exact(&mut count_buf)?;
    let count = u32::from_le_bytes(count_buf) as usize;
    let mut args = Vec::with_capacity(count);
    for _ in 0..count {
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf)?;
        args.push(OsString::from_vec(buf));
    }
    Ok(args)
}
