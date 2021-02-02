fn main() {
    // bit shift
    println!("{}", 1 << 1);
    println!("{}", 1 << 2);
    println!("{}", 1 << 3);
    println!("{}", 1 << 4);

    // masking
    println!("{}", 15 & 1 << 0);
    println!("{}", 15 & 1 << 1);
    println!("{}", 15 & 1 << 2);
    println!("{}", 15 & 1 << 3);
    println!("{}", 15 & 1 << 4);
}
