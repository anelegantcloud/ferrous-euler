// This top-level module binds library exports

// Returns the sum of all multiples of any base less than limit
pub fn sum_of_multiples(bases: &Vec<usize>, limit: usize) -> usize {
    // Returns a vector of all multiples of base less than limit
    let multiples = |base: usize, limit: usize| -> Vec<usize> {
        let mut result = vec![];
        for n in (base..limit).step_by(base) {
            result.push(n);
        }
        return result;
    };
    let mut collected_multiples: Vec<usize> = vec![];
    for i in bases {
        collected_multiples.extend(multiples(*i, limit));
    }
    collected_multiples.sort();
    collected_multiples.dedup();
    return collected_multiples.iter().sum();
}

// Returns the vector of fibonacci integers less than limit evenly divisible by divisor
pub fn fibonacci(mut term1: usize, mut term2: usize, limit: usize, divisor: usize) -> Vec<usize> {
    let mut sequence = vec![];
    let mut push_divisible = |i: usize| -> () {
        if i % divisor == 0 {
            sequence.push(i)
        }
    };
    push_divisible(term1);
    push_divisible(term2);
    let mut new_term = term1 + term2;
    while new_term <= limit {
        push_divisible(new_term);
        term1 = term2;
        term2 = new_term;
        new_term = term1 + term2;
    }
    return sequence;
}

// Returns the vector of integers representing the prime factors of input
pub fn prime_factors(mut factorize: usize) -> Vec<usize> {
    if is_prime(factorize, 2) {
        return vec![factorize];
    } else {
        let mut results = vec![];
        for i in 2..(f64::sqrt(factorize as f64) as usize) {
            while factorize % i == 0 {
                results.push(i);
                factorize = factorize/i;
            }
        }
        return results;
    }
}

// Return whether a number is prime.
fn is_prime(n: usize, i: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    if n == i {
        return true;
    }

    if n % i == 0 {
        return false;
    }

    return is_prime(n, i+1);
}
