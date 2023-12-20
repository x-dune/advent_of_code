pub mod util {
    use std::collections::HashMap;

    fn get_prime_factors(n: i64) -> Vec<i64> {
        let mut n = n;
        let mut result = vec![];

        while n % 2 == 0 {
            result.push(2);
            n /= 2;
        }

        let mut i = 3;
        while i <= (n as f64).sqrt().floor() as i64 {
            i += 2;

            while n % i == 0 {
                result.push(i);
                n /= i;
            }
        }

        if n > 2 {
            result.push(n);
        }

        result
    }

    pub fn lcm(ns: Vec<i64>) -> i64 {
        let prime_factors_of_ns = ns.into_iter().map(get_prime_factors).collect::<Vec<_>>();

        let mut max_count_of_factor: HashMap<i64, i64> = HashMap::new();

        for factors in prime_factors_of_ns {
            let mut count_of_factor: HashMap<i64, i64> = HashMap::new();
            for factor in factors {
                *count_of_factor.entry(factor).or_insert(0) += 1;
            }

            for (factor, count) in count_of_factor.into_iter() {
                let max_count = max_count_of_factor.entry(factor).or_insert(0);

                if count > *max_count {
                    *max_count = count;
                }
            }
        }

        max_count_of_factor
            .iter()
            .fold(1, |acc, (factor, count)| acc * factor.pow(*count as u32))
    }
}
