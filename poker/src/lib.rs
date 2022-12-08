/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
// Poker hand ranking (no wilds)
// 11 Straight Flush (5 in a row, same suit)
// 10 Straight Flush (5 in a row, same suit, ace low)
// 9 Four of a kind (4 same suit)               = 1
// 8 Full house (2 one suit, 3 another suit)    = 10
// 7 Flush (5 same suit)
// 6 Straight (5 in a row)
// 5 Straight (5 in a row, ace low)
// 4 Three of a Kind (3 same suit)              = 9
// 3 Two pair (2 one suit, 2 another suit)      = 7
// 2 One pair (2 same suit)                     = 6
// 1 High card                                  = 5

// Suits
// H Hearts
// S Spades
// C Clubs
// D Diamonds

use std::collections::HashMap;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let ts = hands.iter().map(|&s| (eval(s), s)).collect::<Vec<_>>();

    let (mx, _) = ts.iter().max_by_key(|(h, _)| h).unwrap();

    ts.iter()
        .filter(|(x, _)| x == mx)
        .map(|(_, s)| *s)
        .collect::<Vec<_>>()
}

fn eval(s: &str) -> (Vec<usize>, Vec<u8>) {
    let (rs, ss): (Vec<u8>, Vec<u8>) = s.split_whitespace().map(card).unzip();

    let mut hs: Vec<_> = rs
        .iter()
        .fold(HashMap::<u8, usize>::new(), |mut h, &v| {
            *h.entry(v).or_default() += 1;
            h
        })
        .into_iter()
        .map(|(v, c)| (c, v))
        .collect();

    hs.sort_by(|a, b| b.cmp(a));

    let (mut cs, mut rs): (Vec<_>, Vec<_>) = hs.iter().copied().unzip();

    if cs.len() == 5 {
        if rs == [14, 5, 4, 3, 2] {
            rs = vec![5, 4, 3, 2, 1];
        }

        let is_straight = rs[0] - rs[4] == 4;

        let is_flush = ss.windows(2).all(|w| w[0] == w[1]);

        match (is_straight, is_flush) {
            (true, true) => cs = vec![5],

            (false, true) => cs = vec![3, 1, 3],

            (true, false) => cs = vec![3, 1, 2],

            _ => {}
        }
    }

    (cs, rs)
}

fn card(c: &str) -> (u8, u8) {
    let (r, s) = c.split_at(c.len() - 1);

    (
        match r.parse::<u8>() {
            Ok(v) => v,

            Err(_) => "JQKA".find(r).unwrap() as u8 + 11,
        },
        s.as_bytes()[0],
    )
}
