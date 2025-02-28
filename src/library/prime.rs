pub struct PrimeFactorizationHelper {
    prime_memoization: Vec<usize>,
}

impl Default for PrimeFactorizationHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimeFactorizationHelper {
    pub fn new() -> PrimeFactorizationHelper {
        PrimeFactorizationHelper {
            prime_memoization: vec![],
        }
    }

    pub fn factorize(&mut self, num: usize) -> Vec<usize> {
        self.populize_memoization(num);

        let mut prime_factorization = Vec::new();
        let mut curr_reminder = num;
        while curr_reminder > 1 {
            for &prime in self.prime_memoization.iter() {
                if curr_reminder % prime == 0 {
                    prime_factorization.push(prime);
                    curr_reminder /= prime;
                    continue;
                }
            }

            if curr_reminder > 1 {
                // The remainder is a prime bigger than the current memoization done.
                self.memoize_primes_up_to(curr_reminder);
                prime_factorization.push(curr_reminder);
                break;
            }
        }
        prime_factorization
    }

    fn populize_memoization(&mut self, num: usize) {
        let sqrt = (num as f64).sqrt();
        let largest_val_to_check = sqrt.ceil() as usize;

        self.memoize_primes_up_to(largest_val_to_check);
    }

    fn memoize_primes_up_to(&mut self, value: usize) {
        let mut largest_prime_yet = 2;
        if let Some(val) = self.prime_memoization.last() {
            largest_prime_yet = *val;
        }

        if largest_prime_yet > value {
            return;
        }

        let mut curr = largest_prime_yet;
        while curr < value {
            if !self.prime_memoization.iter().any(|prime| curr % prime == 0) {
                self.prime_memoization.push(curr);
            }
            curr += 1;
        }
    }
}
