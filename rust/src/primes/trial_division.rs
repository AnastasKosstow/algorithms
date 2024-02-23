pub fn trieal_division(up_to: usize) -> Vec<usize> {
    if up_to < 2 {
        return vec![];
    }
    
    let mut primes: Vec<usize> = Vec::new();

    for number in 2..up_to {
        let mut is_prime = true;
        let sqrt_number = (number as f64).sqrt() as usize;
        for prime_index in 0..sqrt_number {
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
        let primes: Vec<usize> = trieal_division(1000000);
        assert!(!primes.is_empty());
        assert_eq!(primes.len(), 78498);
        assert_eq!(primes.first().unwrap(), &2);
        assert_eq!(primes.last().unwrap(), &999983);
    }
}
