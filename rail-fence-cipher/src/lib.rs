pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut code = vec![String::new(); self.rails as usize];
        let zig_zag = (0..self.rails - 1).chain((1..self.rails).rev()).cycle();
        for (ch, i) in text.chars().zip(zig_zag) {
            code[i as usize].push(ch);
        }
        code.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let zig_zag = (0..self.rails - 1).chain((1..self.rails).rev()).cycle();
        let mut indexes: Vec<_> = zig_zag.zip(1..).take(cipher.len()).collect();
        indexes.sort();

        let mut char_with_index: Vec<_> = cipher
            .chars()
            .zip(indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();

        char_with_index.sort();
        char_with_index.iter().map(|(_, c)| c).collect()
    }
}
