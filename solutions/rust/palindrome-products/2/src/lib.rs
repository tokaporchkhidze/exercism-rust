use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl PartialEq for Palindrome {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Palindrome {}

impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Palindrome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindrome_min = Palindrome {
        value: u64::MAX,
        factors: HashSet::new(),
    };
    let mut palindrome_max = Palindrome {
        value: u64::MIN,
        factors: HashSet::new(),
    };
    for i in min..=max {
        for j in i..=max {
            let product = i * j;

            let is_viable_min = palindrome_min.value >= product;
            let is_viable_max = palindrome_max.value <= product;

            if !is_viable_min && !is_viable_max {
                continue;
            }

            if is_palindrome(product) {
                if palindrome_min.value > product {
                    palindrome_min = Palindrome {
                        value: product,
                        factors: HashSet::from([(i, j)]),
                    };
                } else if palindrome_min.value == product {
                    palindrome_min.factors.insert((i, j));
                }
                if palindrome_max.value < product {
                    palindrome_max = Palindrome {
                        value: product,
                        factors: HashSet::from([(i, j)]),
                    }
                } else if palindrome_max.value == product {
                    palindrome_max.factors.insert((i, j));
                }
            }
        }
    }
    match (palindrome_min.value, palindrome_max.value) {
        (u64::MAX, u64::MIN) => None,
        _ => Some((palindrome_min, palindrome_max)),
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut tmp = n;
    let mut sum = 0;
    while tmp > 0 {
        sum = sum * 10 + tmp % 10;
        tmp /= 10;
    }
    sum == n
}
