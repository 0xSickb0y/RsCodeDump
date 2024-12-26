// run this crate with:
// RUST_BACKTRACE=1 cargo run --release
// RUST_BACKTRACE=full cargo run --release
fn main() {
    panic!("Panic without cleaning up the stack.")
}
