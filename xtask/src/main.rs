use std::fs;
use std::io::{Write, stdout, stderr};
// use std::path::PathBuf;
use std::process::Command;

fn main() {
    let dir = fs::canonicalize("xcheck-ebpf").expect("failed to canoicalize xcheck-ebpf");
    // let dir = PathBuf::from("xcheck-ebpf");
    println!("Run 'rustup show':");
    let output = Command::new("rustup")
        .current_dir(dir.clone())
        .arg("show")
        .env_remove("RUSTUP_TOOLCHAIN")
        .output().expect("failed to run command");
    
        println!("status: {}", output.status);
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
    
        println!("Run 'cargo -V' in the xcheck-ebpf subdirectory:");
        let mut cmd = Command::new("cargo");
        cmd.current_dir(dir)
            .arg("-V");

        let cur_dir = cmd.get_current_dir().unwrap();
        eprintln!("Current dir: {cur_dir:#?}");

        let output = cmd.output().expect("failed to run command");
    
        println!("status: {}", output.status);
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
}
