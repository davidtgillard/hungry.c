fn main() {
    loop {
        Box::leak(Box::new(1));
    }
}
