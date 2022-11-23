// This top-level module binds library exports

// Returns a vector of all multiples of base less than threshold
fn multiples(base: usize, threshold: usize) -> Vec<usize> {
    let mut result = vec![];
    for n in (base..threshold).step_by(base) {
        result.push(n);
    }
    return result;
}

// Returns the sum of all multiples of any base less than threshold
pub fn sum_of_multiples(bases: &Vec<usize>, threshold: usize) -> usize {
    let mut collected_multiples: Vec<usize> = vec![];
    for i in bases {
        collected_multiples.extend(multiples(*i, threshold));
    }
    collected_multiples.sort();
    collected_multiples.dedup();
    return collected_multiples.iter().sum();
}