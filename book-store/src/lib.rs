pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        0
    } else {
        let mut current_price = 0;
        let mut basket = vec![];
        let mut remaining_books = books.to_vec();
        let mut i = 0;

        while i < remaining_books.len() {
            if !basket.contains(&remaining_books[i]) {
                basket.push(remaining_books[i]);
                remaining_books.remove(i);
                let partial_price = lowest_price(&remaining_books);
                let price = partial_price + match basket.len() {
                                            2 => 16 * 95,
                                            3 => 24 * 90,
                                            4 => 32 * 80,
                                            5 => 40 * 75,
                                            _ => 8 * 100,
                    };
                if current_price == 0 || current_price > price {
                    current_price = price
                };
            } else {
                i += 1;
            }
        }
        current_price
    }
}
