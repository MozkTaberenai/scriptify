use super::*;
use crate::*;
use std::io::{Read, Write};

#[test]
fn cmd() {
    let status = cmd!("sh", "-c", "exit 0")
        .spawn()
        .expect("fail to spawn")
        .wait()
        .expect("fail to wait");
    assert!(status.success());
    assert_eq!(status.code().next(), Some(0));

    let status = cmd!("sh", "-c", "exit 1")
        .spawn()
        .expect("fail to spawn")
        .wait()
        .expect("fail to wait");
    assert!(!status.success());
    assert_eq!(status.code().next(), Some(1));
}

#[test]
fn pipe() {
    let status = cmd!("echo", "xyz")
        .pipe(cmd!("rev"))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe(cmd!("echo"))
        .spawn()
        .expect("fail to spawn")
        .wait()
        .expect("fail to wait");

    assert!(status.success());
    assert!(status.code().all(|code| code == 0));
}

#[test]
fn write_spawn() {
    let (mut stdin, handle) = cmd!("echo")
        .pipe(std::io::sink())
        .write_spawn()
        .expect("fail to write_spawn");
    stdin.write_all(b"abc").expect("fail to write_all");
    let (status, _sink) = handle.wait().expect("fail to wait");
    assert!(status.success());
}

#[test]
fn read_spawn() {
    let (mut stdout, handle) = cmd!("echo", "xyz")
        .read_spawn()
        .expect("fail to read_spawn");
    let mut buf = vec![];
    stdout.read_to_end(&mut buf).expect("fail to read_to_end");
    assert_eq!(&buf, b"xyz\n");
    let status = handle.wait().expect("fail to wait");
    assert!(status.success());
}

#[test]
fn write_read_spawn() {
    let (mut stdin, mut stdout, handle) = cmd!("rev")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .write_read_spawn()
        .expect("fail to write_read_spawn");

    std::thread::spawn(move || write!(stdin, "xyz"));
    let mut buf = String::new();
    stdout
        .read_to_string(&mut buf)
        .expect("fail to read_to_string");
    assert_eq!(&buf, "ZYX\n");

    let status = handle.wait().expect("fail to wait");
    assert!(status.success());
    assert!(status.code().all(|code| code == 0));
}

#[test]
fn pipe_from_reader() {
    let input = &b"xyz"[..];
    let output = input
        .pipe(cmd!("rev"))
        .read_to_string()
        .expect("fail to read_to_string");
    assert_eq!(&output, "zyx\n");
}

#[test]
fn pipe_to_writer() {
    let handle = cmd!("echo", "-n", "abcde")
        .pipe(vec![])
        .spawn()
        .expect("fail to spawn");
    let (status, output) = handle.wait().expect("fail to wait");
    assert!(status.success());
    assert_eq!(&output, b"abcde");
}

// ToDo: echo tests
