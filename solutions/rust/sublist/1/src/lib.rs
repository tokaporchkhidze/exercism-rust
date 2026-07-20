#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of_first(first_list: &[i32], second_list: &[i32]) -> bool {
    let mut slice = first_list;
    while let Some(start_index) = slice.iter().position(|&num| num == second_list[0]) {
        if start_index + second_list.len() > slice.len() {
            return false;
        }

        if second_list == &slice[start_index..start_index + second_list.len()] {
            return true;
        }
        slice = &slice[start_index + 1..];
    }
    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.len() > second_list.len()
        && (second_list.is_empty() || sublist_of_first(first_list, second_list))
    {
        return Comparison::Superlist;
    } else if first_list.len() < second_list.len()
        && (first_list.is_empty() || sublist_of_first(second_list, first_list))
    {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
