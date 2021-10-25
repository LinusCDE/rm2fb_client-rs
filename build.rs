use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_current_dir("remarkable2-framebuffer/src/client")?;

    // Add extra line to the end of "client.pro" if missing
    let client_pro_extra_line = "CONFIG += staticlib\n";
    let mut client_pro_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("client.pro")?;
    let mut client_pro_content = String::new();
    client_pro_file.read_to_string(&mut client_pro_content)?;
    if !client_pro_content.contains(client_pro_extra_line) {
        client_pro_file.seek(SeekFrom::End(0))?;
        client_pro_file.write_all(client_pro_extra_line.as_bytes())?;
    }
    drop(client_pro_file);

    Command::new("/usr/bin/env")
        .arg("qmake")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

    Command::new("/usr/bin/env")
        .arg("make")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

    // Copy generated file to OUT_DIR and tell rust to link it statically
    let static_lib_filename = "librm2fb_client.a";
    fs::copy(
        static_lib_filename,
        PathBuf::from(env::var("OUT_DIR")?).join(static_lib_filename),
    )?;
    println!("rustc-link-lib=static=librm2fb_client");

    Ok(())
}
