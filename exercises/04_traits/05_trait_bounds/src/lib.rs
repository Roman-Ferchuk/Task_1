use std::cmp::PartialOrd;

pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
