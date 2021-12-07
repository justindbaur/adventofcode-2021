use std::path::Path;
use std::{env, fs};

fn main() {
    println!("Building");
    let out_dir = env::var("OUT_DIR").unwrap();
    copy(&out_dir, "input1.txt");
}

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    fs::copy(file_name, Path::new(target_dir_path).join("../../..").join(file_name)).unwrap();
}
