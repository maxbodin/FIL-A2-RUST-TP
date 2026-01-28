fn main() {
    println!("Hello, world!");

    let x = 5;
    let x = x + 1; // x is not mutable: another variable is defined
    let x = x * 2;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // pattern matching

    let a = [1, 2];
    let first = a[0];

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn stupid() -> u32 {
    if true { 5 } else { 5 } // {} required
}

