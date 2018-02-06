use std::env;
use std::process::Command; 

fn main() {

    // Only build honggfuzz binaries if we are in the process of building an instrumentized binary
    let honggfuzz_target_dir =  match env::var("CARGO_HONGGFUZZ_TARGET_DIR") {
        Ok(path) => path,
        Err(_) => return
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    let pwd = env::var("PWD").unwrap();

    // build honggfuzz command and hfuzz static library
    let status = Command::new("make")
        .args(&["-C", "honggfuzz", "clean", "honggfuzz", "libhfuzz/libhfuzz.a"])
        .status()
        .expect("failed to run \"make -C honggfuzz clean hongfuzz libhfuzz/libhfuzz.a\"");
    assert!(status.success());

    // copy hfuzz static library to output directory
    let status = Command::new("cp")
        .args(&["honggfuzz/libhfuzz/libhfuzz.a", &out_dir])
        .status()
        .expect(&format!("failed to run \"cp honggfuzz/libhfuzz/libhfuzz.a {}\"", &out_dir));
    assert!(status.success());

    // copy honggfuzz executable to honggfuzz target directory
    let status = Command::new("cp")
        .args(&["honggfuzz/honggfuzz", &format!("{}/{}", &pwd, &honggfuzz_target_dir)])
        .status()
        .expect(&format!("failed to run \"cp honggfuzz/honggfuzz {}\"", &honggfuzz_target_dir));
    assert!(status.success());

    println!("cargo:rustc-link-lib=static={}", "hfuzz");
    println!("cargo:rustc-link-search=native={}", &out_dir);
}
