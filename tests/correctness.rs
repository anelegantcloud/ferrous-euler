#[cfg(test)]
mod tests_correctness {
    // This module tests numerical output of problem statements for correctness. Solutions:
    // https://github.com/luckytoilet/projecteuler-solutions/blob/master/Solutions.md
    
    use ferrous_euler;

    #[test]
    fn solution_1() {
        // Find the sum of natural numbers below threshold X that are multiples of 3 or 5.
        let bases = vec![3,5];
        // The sum of these multiples is 23 when X = 10.
        assert_eq!(ferrous_euler::sum_of_multiples(&bases, 10), 23);
        // Find the sum of all the multiples of 3 or 5 when X = 1000.
        assert_eq!(ferrous_euler::sum_of_multiples(&bases, 1000), 233168);
    }
}