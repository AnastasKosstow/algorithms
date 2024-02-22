pub fn sieve_of_eratosthenes() -> Vec<i32> {

    let primes_up_to = 100;
    let mut sieve: Vec<bool> = vec![true; primes_up_to];
    sieve[0] = false;
    sieve[1] = false;
    
    let sqrt_limit = (primes_up_to as f64).sqrt() as usize;
    for number in 2..sqrt_limit {
        if sieve[number] == true {
            let mut multiple = number * number;
            while multiple < primes_up_to {
                sieve[multiple] = false;
                multiple += number;
            }
        }
    }
    sieve.iter().enumerate()
        .filter_map(|(number, &is_prime)| if is_prime { Some(number as i32) } else { None })
        .collect()
}
