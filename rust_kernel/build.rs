fn main() {
    rerun();

    generate_fonts();
    generate_images();
}

fn rerun() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rust_kernel/assets/render/fonts.py");
    println!("cargo:rerun-if-changed=rust_kernel/assets/fonts/*");
    println!("cargo:rerun-if-changed=rust_kernel/assets/images/*");
    println!("cargo:rerun-if-changed=rust_kernel/assets/render/images.py");

    #[cfg(unix)]
    println!("cargo:rerun-if-changed=/dev/null");
    #[cfg(windows)]
    println!("cargo:rerun-if-changed=NUL");
}

fn generate_fonts() {
    let output = std::process::Command::new("python3")
        .arg("assets/render/fonts.py")
        .arg("assets/fonts")
        .arg("13")
        .output()
        .expect("Failed to generate fonts");

    if !output.status.success() {
        eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Python script execution failed");
    }
}

fn generate_images() {
    let output = std::process::Command::new("python3")
        .arg("assets/render/images.py")
        .arg("assets/images")
        .arg("ppm")
        .output()
        .expect("Failed to generate fonts");

    if !output.status.success() {
        eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Python script execution failed");
    }
}
