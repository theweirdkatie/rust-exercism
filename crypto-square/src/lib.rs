pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<Vec<_>>();
    let cols = (normalized.len() as f64).sqrt().ceil() as usize;
    let rows = (normalized.len() as f64).sqrt().floor() as usize;
    (0..cols)
        .map(|i| {
            (0..rows)
                .map(|j| normalized.get(i + j * cols).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}