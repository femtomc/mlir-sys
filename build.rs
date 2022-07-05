extern crate bindgen;

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search={}", llvm_config("--libdir")?);

    for name in fs::read_dir(llvm_config("--libdir")?)?
        .map(|entry| {
            Ok(if let Some(name) = entry?.path().file_name() {
                name.to_str().map(String::from)
            } else {
                None
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .flatten()
    {
        if name == "libMLIRSupport.a"
            || name.starts_with("libMLIR")
                && name.ends_with(".a")
                && !["Main", "Support"]
                    .iter()
                    .any(|pattern| name.contains(pattern))
        {
            println!(
                "cargo:rustc-link-lib=static={}",
                name.trim_start_matches("lib").trim_end_matches(".a")
            );
        }
    }

    for name in llvm_config("--libnames")?.trim().split(' ') {
        println!(
            "cargo:rustc-link-lib={}",
            name.trim_start_matches("lib").trim_end_matches(".a")
        );
    }

    for flag in llvm_config("--system-libs")?.trim().split(' ') {
        println!("cargo:rustc-link-lib={}", flag.trim_start_matches("-l"));
    }

    if let Some(name) = get_system_libcpp() {
        println!("cargo:rustc-link-lib={}", name);
    }

    bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", llvm_config("--includedir")?))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(Path::new(&env::var("OUT_DIR")?).join("bindings.rs"))?;

    Ok(())
}

fn get_system_libcpp() -> Option<&'static str> {
    if cfg!(target_env = "msvc") {
        None
    } else if cfg!(target_os = "macos") {
        Some("c++")
    } else {
        Some("stdc++")
    }
}

fn llvm_config(argument: &str) -> Result<String, Box<dyn Error>> {
    let call = format!("llvm-config --link-static {}", argument);

    Ok(str::from_utf8(
        &if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &call]).output()?
        } else {
            Command::new("sh").arg("-c").arg(&call).output()?
        }
        .stdout,
    )?
    .trim()
    .to_string())
}
