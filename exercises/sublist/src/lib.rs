#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        return if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    }
    if _first_list.len() > _second_list.len() {
        return if (0..=_first_list.len() - _second_list.len()).any(|i| {
            _first_list[i..].starts_with(_second_list)
        }) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        };
    }
    if _second_list.len() > _first_list.len() {
        return if (0..=_second_list.len() - _first_list.len()).any(|i| {
            _second_list[i..].starts_with(_first_list)
        }) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        };
    }
    Comparison::Unequal
}
