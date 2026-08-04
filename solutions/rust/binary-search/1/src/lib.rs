pub fn find<C, T>(array: C, key: T) -> Option<usize>
where T: Ord,
    C: AsRef<[T]>
{
    let array = array.as_ref();
    let mut low = 0usize;
    let mut high = array.len();
    while low < high {
        let mid = (low + high) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    None
}
