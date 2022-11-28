pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug, Default, Clone)]

    pub struct Graph {
        pub nodes: Vec<char>,
        pub edges: Vec<char>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph{
                ..Default::default()
            }
        }
    }
}
