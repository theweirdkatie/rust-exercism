pub fn map<F, T, U>(input: Vec<T>, mut _function: F) -> Vec<U> 
    where F: FnMut(T) -> U
{
    let mut result = vec![];
    for i in input {
        result.push(_function(i));
    }
    result
}
