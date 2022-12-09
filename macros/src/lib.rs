#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut temp_hash = ::std::collections::HashMap::new();
            $(
                let _ = temp_hash.insert($key, $value);
            )*
            temp_hash
        }
    };
}
