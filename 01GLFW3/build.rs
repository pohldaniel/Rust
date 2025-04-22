fn main() {
    println!("cargo:rustc-link-search=./lib/");
    println!("cargo:rustc-link-lib=static=msvcrt");
    println!("cargo:rustc-link-lib=static=user32");
    println!("cargo:rustc-link-lib=static=gdi32");
    println!("cargo:rustc-link-lib=static=shell32");
    println!("cargo:rustc-link-lib=static=libglfw3");   
    println!("cargo:rustc-link-lib=static=libglew");
}