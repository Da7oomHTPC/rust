// aux-build:issue_54059.rs
// ignore-wasm32-bare no libc for ffi testing
// ignore-windows - dealing with weird symbols issues on dylibs isn't worth it
// revisions: rpass1

extern crate issue_54059;

fn main() {}
