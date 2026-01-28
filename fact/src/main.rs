use num_bigint::BigInt;
use num_traits::One;

fn main() {
    println!("Hello, world!");

    println!("The value of 10! is {}", fact64(10));
    println!("The value of 30! is {}", fact64(30));
    println!("The value of 100! is {}", fact64(100));

    println!("The value of 10! is {}", fact128(10));
    println!("The value of 30! is {}", fact128(30));
    println!("The value of 100! is {}", fact128(100));

    println!("The value of 10! is {}", fact_big_int(BigInt::from(10)));
    println!("The value of 30! is {}", fact_big_int(BigInt::from(30)));
    println!("The value of 100! is {}", fact_big_int(BigInt::from(100)));
    println!("The value of 200! is {}", fact_big_int(BigInt::from(200)));
}


pub fn fact64(n: u64) -> u64 {
    fn _fact(n: u64, so_far: u64) -> u64 {
        if n <= 1 { so_far } else { _fact(n - 1, so_far * n) }
    }
    _fact(n, 1)
}


pub fn fact128(n: u128) -> u128 {
    fn _fact(n: u128, so_far: u128) -> u128 {
        if n <= 1 { so_far } else { _fact(n - 1, so_far * n) }
    }
    _fact(n, 1)
}

pub fn fact_big_int(n: BigInt) -> BigInt {
    fn _fact(n: BigInt, so_far: BigInt) -> BigInt {
        if n <= BigInt::one() { so_far } else { _fact(&n - 1, so_far * n) }
    }
    _fact(n, BigInt::one())
}