/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

// Poker hand ranking (no wilds)
// 1 Straight Flush (5 in a row, same suit)
// 2 Four of a kind (4 same suit)
// 3 Full house (2 one suit, 3 another suit)
// 4 Flush (5 same suit)
// 5 Straight (5 in a row)
// 6 Three of a Kind (3 same suit)
// 7 Two pair (2 one suit, 2 another suit)
// 8 One pair (2 same suit)
// 9 High card

// Suits
// H Hearts
// S Spades
// C Clubs
// D Diamonds

use std::collections::HashMap;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_map: HashMap<u32, String> = HashMap::new();
    let mut rank: u32 = 0;
    for h in hands {
        let mut cards: Vec<&str> = h.split(" ").collect::<Vec<&str>>();
        for c in cards {
            let num = c.chars().nth(0).unwrap_or_default().to_digit(10).unwrap_or_default();
            let suit = c.chars().last().unwrap_or_default();

            rank +=
        }

    }
    
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
