#![feature(unix_sigpipe)]

#[unix_sigpipe = "sig_dfl"]
fn main() {
    println!("rust tooling for analyzing and trasforming rust program: v0.1.0");
    rustdoc::tooling_main();
}
