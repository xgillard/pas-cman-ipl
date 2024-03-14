//! This was flat out copy pasted
#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/pas-cman-icon.ico");
    res.compile().unwrap();
  }
}