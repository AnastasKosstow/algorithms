pub fn dijkstra_primes(up_to: usize) -> Vec<usize> {
    if up_to < 2 {
        return vec![];
    }

    let mut primes: Vec<usize> = Vec::new();
    let mut pool = Pool::new();
    
    primes.push(2);
    pool.insert(2);
    
    for number in 3..up_to {
        if let Some(multiplier) = pool.get_smallest_multiplier() {
            if number < multiplier {
                primes.push(number);
                pool.insert(number);
            } else {
                pool.update(number);
            }
        }
    }

    primes
}

struct Pool {
    pairs: Vec<(usize, usize)>
}

impl Pool {
    fn new() -> Self {
        Pool { pairs: vec![] }
    }

    fn insert(&mut self, v: usize) {
        self.pairs.push((v, v * v));
    }

    fn update(&mut self, v: usize) {
        let mut index = 0;
        for pair in self.pairs.iter_mut().take_while(|pair| pair.1 == v) {
            pair.1 += pair.0;
            index += 1;
        }

        if self.pairs.len() > 1 {
            index -= 1;
            loop {
                let mut i = index;
                while self.pairs[i].1 > self.pairs[i + 1].1 {
                    self.pairs.swap(i, i + 1);
                    i += 1;
                }
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        }
    }

    fn get_smallest_multiplier(&self) -> Option<usize> {
        if let Some(pair) = self.pairs.first() {
            return Some(pair.1);
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_primes() {
        let primes: Vec<usize> = dijkstra_primes(1000000);
        assert!(!primes.is_empty());
        assert_eq!(primes.len(), 78498);
        assert_eq!(primes.first().unwrap(), &2);
        assert_eq!(primes.last().unwrap(), &999983);
    }
}
