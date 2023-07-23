use crate::*;

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
fn pipe_from_reader() {
    let input = &b"xyz"[..];
    let (mut stdout, handle) = input.pipe(cmd!("rev")).read_spawn().expect("fail to spawn");
    assert_eq!(stdout.read_to_end().unwrap(), b"zyx\n");
    let (status, _) = handle.wait().unwrap();
    assert!(status.success());
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

#[test]
fn write_spawn() {
    let (mut stdin, handle) = cmd!("echo")
        .pipe(std::io::sink())
        .write_spawn()
        .expect("fail to spawn");
    stdin.write_all(b"abc").expect("fail to write_all");
    let (status, _sink) = handle.wait().expect("fail to wait");
    assert!(status.success());
}

#[test]
fn read_spawn() {
    let (mut stdout, handle) = cmd!("echo", "xyz").read_spawn().expect("fail to spawn");
    let buf = stdout.read_to_end().expect("fail to read_to_vec");
    assert_eq!(&buf[..], b"xyz\n");
    let status = handle.wait().expect("fail to wait");
    assert!(status.success());
}

#[test]
fn write_read_spawn() {
    let (mut stdin, mut stdout, handle) = cmd!("rev")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .write_read_spawn()
        .expect("fail to spawn");

    std::thread::spawn(move || write!(stdin, "xyz"));
    let out = stdout.read_to_string().expect("fail to read_to_string");
    assert_eq!(out, "ZYX\n");

    let status = handle.wait().expect("fail to wait");
    assert!(status.success());
    assert!(status.code().all(|code| code == 0));
}

// #[test]
// fn pipeline() {
//     let (mut stdin, mut stdout, children) = PipeWriter
//         .to(cmd!("rev"))
//         .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
//         .pipe_out()
//         .spawn()
//         .expect("fail to spawn");

//     std::thread::spawn(move || write!(stdin, "xyz"));
//     let mut out = String::new();
//     stdout.read_to_string(&mut out).unwrap();
//     assert_eq!(out.trim(), "ZYX");

//     let status = children.wait().expect("fail to wait");
//     assert!(status.iter().all(std::process::ExitStatus::success));
//     assert!(status.iter().all(|s| s.code() == Some(0)));
// }
