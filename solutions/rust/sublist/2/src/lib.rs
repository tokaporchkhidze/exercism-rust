#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m > n => {
            if first_list
                .windows(second_list.len())
                .any(|slice| slice == second_list)
            {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m < n => {
            if second_list
                .windows(first_list.len())
                .any(|slice| slice == first_list)
            {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
