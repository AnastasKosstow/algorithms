pub fn dijkstra_primes(up_to: usize) -> Vec<i32> {

    if up_to < 2 {
        return vec![];
    }

    let mut primes: Vec<i32> = Vec::new();
    let mut pool = Pool::new();
    
    primes.push(2);
    pool.insert(2);
    
    for number in 3..(up_to as i32) {
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
    pairs: Vec<(i32, i32)>
}

impl Pool {
    fn new() -> Self {
        Pool { pairs: vec![] }
    }

    fn insert(&mut self, v: i32) {
        self.pairs.push((v, v * v));
    }

    fn update(&mut self, v: i32) {
        let mut index = 0;
        while self.pairs[index].1 == v {
            self.pairs[index].1 += self.pairs[index].0;
            index += 1;
        }

        index -= 1;
        let current = self.pairs[index];
        loop {
            let mut sorted = true;
            while current.1 > self.pairs[index + 1].1 {
                let temp = self.pairs[index + 1];
                self.pairs[index + 1] = current;
                self.pairs[index] = temp;
                sorted = false;

                if index == 0 {
                    break;
                }
                index -= 1;
            }

            if sorted {
                break;
            }
        }
    }

    fn get_smallest_multiplier(&self) -> Option<i32> {
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
        let primes: Vec<i32> = dijkstra_primes(30);
        assert!(!primes.is_empty());
        assert_eq!(primes.len(), 10);
        assert_eq!(primes.first().unwrap(), &2);
        assert_eq!(primes.last().unwrap(), &29);
    }
}
