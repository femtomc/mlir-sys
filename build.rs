use std::{
    env,
    error::Error,
    fs, io,
    path::Path,
    process::{exit, Command},
    str,
};

const LLVM_MAJOR_VERSION: usize = 19;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let version = llvm_config("--version")?;
    if !version.starts_with(&format!("{}.", LLVM_MAJOR_VERSION)) {
        return Err(format!(
            "failed to find correct version ({}.x.x) of llvm-config (found {})",
            LLVM_MAJOR_VERSION, version
        )
        .into());
    }

    let lib_dir = llvm_config("--libdir")?;
    println!("cargo:rustc-link-search={lib_dir}");
    for name in fs::read_dir(lib_dir)?
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
        if name.starts_with("libMLIR")
            && name.ends_with(".a")
            && !name.contains("Main")
            && name != "libMLIRSupportIndentedOstream.a"
        {
            if let Some(name) = trim_library_name(&name) {
                println!("cargo:rustc-link-lib=static={}", name);
            }
        }
    }

    for name in llvm_config("--libnames")?.split(' ') {
        if let Some(name) = trim_library_name(name) {
            println!("cargo:rustc-link-lib={}", name);
        }
    }

    for flag in llvm_config("--system-libs")?.split(' ') {
        let flag = flag.trim_start_matches("-l");

        if flag.starts_with('/') {
            // llvm-config returns absolute paths for dynamically linked libraries.
            let path = Path::new(flag);

            println!(
                "cargo:rustc-link-search={}",
                path.parent().unwrap().display()
            );
            println!(
                "cargo:rustc-link-lib={}",
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split_once('.')
                    .unwrap()
                    .0
                    .trim_start_matches("lib")
            );
        } else {
            println!("cargo:rustc-link-lib={}", flag);
        }
    }

    if let Some(name) = get_system_libcpp() {
        println!("cargo:rustc-link-lib={}", name);
    }

    bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", llvm_config("--includedir")?))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .write_to_file(Path::new(&env::var("OUT_DIR")?).join("bindings.rs"))?;

    Ok(())
}

fn get_system_libcpp() -> Option<&'static str> {
    if env::var("CARGO_CFG_TARGET_ENV").ok()? == "msvc" {
        None
    } else if env::var("CARGO_CFG_TARGET_VENDOR").ok()? == "apple" {
        Some("c++")
    } else {
        Some("stdc++")
    }
}

fn llvm_config(argument: &str) -> Result<String, Box<dyn Error>> {
    let prefix = env::var_os(format!("MLIR_SYS_{}0_PREFIX", LLVM_MAJOR_VERSION))
        .map(|path| Path::new(&path).join("bin"))
        .unwrap_or_default();
    let mut cmd = Command::new(prefix.join("llvm-config"));
    cmd.arg("--link-static").arg(argument);
    let output = cmd
        .output()
        .map_err(|e| format!("failed to run `{cmd:?}`: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "failed to run `{cmd:?}`: {}; stderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
        )
        .into());
    }
    Ok(str::from_utf8(&output.stdout)?.trim().to_string())
}

fn trim_library_name(name: &str) -> Option<&str> {
    if let Some(name) = name.strip_prefix("lib") {
        name.strip_suffix(".a")
    } else {
        None
    }
}
