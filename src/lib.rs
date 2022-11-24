// This top-level module binds library exports

// Returns a vector of all multiples of base less than limit
fn multiples(base: usize, limit: usize) -> Vec<usize> {
    let mut result = vec![];
    for n in (base..limit).step_by(base) {
        result.push(n);
    }
    return result;
}

// Returns the sum of all multiples of any base less than limit
pub fn sum_of_multiples(bases: &Vec<usize>, limit: usize) -> usize {
    let mut collected_multiples: Vec<usize> = vec![];
    for i in bases {
        collected_multiples.extend(multiples(*i, limit));
    }
    collected_multiples.sort();
    collected_multiples.dedup();
    return collected_multiples.iter().sum();
}

// Returns the sequence of evenly divisible fibonacci numbers created less than limit
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
