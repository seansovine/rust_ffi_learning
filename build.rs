fn main() {
  println!("cargo:rustc-link-search=native=c/build/");
  println!("cargo:rustc-link-lib=static=TestLib");
}
