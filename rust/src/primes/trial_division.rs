pub fn trieal_division() -> Vec<i32> {
    
    let primes_up_to = 100;
    let mut primes: Vec<i32> = Vec::new();

    for number in 2..primes_up_to {
        let mut is_prime = true;
        let vec_length = primes.len() as f64;
        for prime_index in 0..vec_length.sqrt() as usize {
            if primes.len() > 0 && number % primes[prime_index] == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(number);
        }
    }
    primes
}
