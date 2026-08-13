use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<Factor>,
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

#[derive(Debug, Clone, Copy)]
struct Factor(u64, u64);

impl PartialEq for Factor {
    fn eq(&self, other: &Self) -> bool {
        if (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0) {
            return true;
        }
        false
    }
}

impl Eq for Factor {}

impl Hash for Factor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (low, high) = if self.0 <= self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        };
        low.hash(state);
        high.hash(state);
    }
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors.into_iter().map(|f| (f.0, f.1)).collect()
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
        for j in min..=max {
            let product = i * j;
            if is_palindrome(product) {
                if palindrome_min.value > product {
                    palindrome_min = Palindrome {
                        value: product,
                        factors: HashSet::from([Factor(i, j)]),
                    };
                } else if palindrome_min.value == product {
                    palindrome_min.factors.insert(Factor(i, j));
                }
                if palindrome_max.value < product {
                    palindrome_max = Palindrome {
                        value: product,
                        factors: HashSet::from([Factor(i, j)]),
                    }
                } else if palindrome_max.value == product {
                    palindrome_max.factors.insert(Factor(i, j));
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
