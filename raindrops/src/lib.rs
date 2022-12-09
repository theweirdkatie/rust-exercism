pub fn raindrops(n: u32) -> String {
    let result: String = [3, 5, 7].iter()
                        .zip(['i','a','o'].iter())
                        .map(|(num, c)| if n % num == 0 {format!("Pl{}ng", c)} else {"".to_string()})
                        .collect();
    if result.len() == 0 {n.to_string()} else {result}
}
