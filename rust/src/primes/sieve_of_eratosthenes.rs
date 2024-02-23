pub fn sieve_of_eratosthenes(up_to: usize) -> Vec<i32> {

    let mut sieve: Vec<bool> = vec![true; up_to];
    sieve[0] = false;
    sieve[1] = false;
    
    let sqrt_limit = (up_to as f64).sqrt() as usize;
    for number in 2..sqrt_limit + 1 {
        if sieve[number] == true {
            let mut multiple = number * number;
            while multiple < up_to {
                sieve[multiple] = false;
                multiple += number;
            }
        }
    }
    sieve.iter().enumerate()
        .filter_map(|(number, &is_prime)| if is_prime { Some(number as i32) } else { None })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes: Vec<i32> = sieve_of_eratosthenes(30);
        assert!(!primes.is_empty());
        assert_eq!(primes.len(), 10);
        assert_eq!(primes.first().unwrap(), &2);
        assert_eq!(primes.last().unwrap(), &29);
    }
}
