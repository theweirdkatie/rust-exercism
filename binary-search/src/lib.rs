pub fn find(array: &[i32], key: i32) -> Option<usize> {
    match array.len() {
        0 => None,
        1 => Some(0),
        _ => {
            let mut low = 0;
            let mut high = array.len()-1;
            while low <= high {
                let mid = (low + high) / 2;
                println!("{}, {}, {}", low, mid, high);
                if array[mid] == key {
                    return Some(mid);
                } else if array[mid] < key {
                    low = mid + 1;
                } else if mid > 0 {
                    high = mid - 1;
                } else {
                    break;
                }
            }
            return None;
        }
    }
}
