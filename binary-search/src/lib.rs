use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len().checked_sub(1)?;

    while low <= high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid.checked_sub(1)?,
        }
    }
    None
}
