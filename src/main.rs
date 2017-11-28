extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    (2..num)
        .filter(|&i| num % i == 0)
        .take(1)
        .count() == 0
}

fn main() {
    let pool = CpuPool::new_num_cpus();
    let prime_future = pool.spawn_fn(
        || -> Result<bool, ()> { Ok(is_prime(BIG_PRIME))});

    println!("Created in the future");

    if prime_future.wait().unwrap() {
        println!("Prime")
    } else {
        println!("Not prime")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_prime(BIG_PRIME));
    }
}
