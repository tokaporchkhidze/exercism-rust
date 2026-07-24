pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut n = n;
    let mut step = 0;
    loop {
        n = match n {
            1 => {
                return Some(step);
            }
            m if m % 2 == 0 => m / 2,
            _ => n * 3 + 1,
        };
        step += 1;
    }
}
