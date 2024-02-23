pub fn trieal_division(up_to: usize) -> Vec<i32> {
    
    let mut primes: Vec<i32> = Vec::new();

    for number in 2..(up_to as i32) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trieal_division() {
        let primes: Vec<i32> = trieal_division(30);
        assert!(!primes.is_empty());
        assert_eq!(primes.len(), 10);
        assert_eq!(primes.first().unwrap(), &2);
        assert_eq!(primes.last().unwrap(), &29);
    }
}
