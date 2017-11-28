const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    (2..num)
        .filter(|&i| num % i == 0)
        .take(1)
        .count() == 0
}

fn main() {
    if is_prime(BIG_PRIME) {
        println!("Prime");
    } else {
        println!("Not prime");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_prime(BIG_PRIME));
    }
}
