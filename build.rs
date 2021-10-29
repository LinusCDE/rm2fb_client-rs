//extern crate link_cplusplus;

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=remarkable2-framebuffer/src/client/client.pro");
    println!("cargo:rerun-if-changed=remarkable2-framebuffer/src/client/main.cpp");


    std::env::set_current_dir("remarkable2-framebuffer/src/client")?;
/*
    // Add extra line to the end of "client.pro" if missing
    let client_pro_extra_line = "CONFIG += staticlib\n";
    let mut client_pro_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("client.pro")?;
println!("cargo:warning={:?}", std::env::var("PATH"));
println!("cargo:warning=1 {:?}", std::env::current_dir());
    let mut client_pro_content = String::new();
    client_pro_file.read_to_string(&mut client_pro_content)?;
    if !client_pro_content.contains(client_pro_extra_line) {
        client_pro_file.seek(SeekFrom::End(0))?;
        client_pro_file.write_all(client_pro_extra_line.as_bytes())?;
    }
println!("cargo:warning=2");
    drop(client_pro_file);

    println!("cargo:warning={:?}", Command::new("/usr/bin/env")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?);


    println!("cargo:warning={:?}", Command::new("/usr/bin/env")
        .arg("qmake")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?);

println!("cargo:warning=3");

    println!("cargo:warning={:?}", Command::new("/usr/bin/env")
        .arg("make")
        .arg("clean")
        .arg("all")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?);
println!("cargo:warning=4");
*/
    // Copy generated file to OUT_DIR and tell rust to link it statically
    let gum_static_lib_filename = "frida/libfrida_gum.a";
    let static_lib_filename = "librm2fb_client.a";
    let out_dir_path = PathBuf::from(env::var("OUT_DIR")?);
println!("cargo:warning=5");
    fs::copy(static_lib_filename, out_dir_path.join(static_lib_filename))?;
println!("cargo:warning=6");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=rm2fb_client");

/*
    std::env::set_current_dir("frida")?;
    let gum_static_lib_filename = "libfrida-gum.a";
println!("cargo:warning=7");
    fs::copy(gum_static_lib_filename, out_dir_path.join(gum_static_lib_filename))?;
println!("cargo:warning=8");
    println!("cargo:rustc-link-lib=static=frida-gum");
*/
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir_path.to_str().unwrap()
    );

    //println!("cargo:rustc-link-lib=static=libstdc++");
    Ok(())
}
