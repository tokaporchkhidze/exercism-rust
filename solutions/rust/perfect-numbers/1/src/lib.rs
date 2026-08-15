#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match (1..num).into_iter().filter(|&x| num.is_multiple_of(x)).sum::<u64>() {
        c if c == num => Some(Classification::Perfect),
        c if num > c => Some(Classification::Deficient),
        _ => Some(Classification::Abundant),
    }
}
