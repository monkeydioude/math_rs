fn wakarimasen(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}

fn main() {
    let a = 2i32;
    let b = 3i32;
    let c = wakarimasen(a, b);
    println!("{}", c);
}