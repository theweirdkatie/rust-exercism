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

#[derive(Debug)]
pub struct Hand<'a> {
    cards: &'a str,
    rank: u32,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut all_hands = vec![];
    let mut winners = vec![];
    for entry in hands {
        all_hands.push(Hand::new(entry));
    }

    let mut high_rank = 0;
    for hand in all_hands {
        dbg!(&hand);
        if hand.rank > high_rank {
            winners.clear();
            winners.push(hand.cards);
            high_rank = hand.rank;
        } else if hand.rank == high_rank {
            // check 0
            if high_rank == 0 {
                winners.push(hand.cards);

            // check high cards
            } else if let Some(result) = hand.compare(winners[0]) {
                if result {
                    winners.clear();
                    winners.push(hand.cards);
                }
            } else {
                winners.push(hand.cards);
            }
        }
    }
    winners
}

impl Hand<'_> {
    pub fn new<'a>(cards: &'a str) -> Hand<'_> {
        Hand {
            cards,
            rank: Hand::rank_hand(cards),
        }
    }

    fn rank_hand(cards: &str) -> u32 {
        let (suits, faces) = (Hand::get_suits(cards), Hand::get_faces(cards));
        println!("Suits: {:?}, Faces: {:?}", suits, faces);
        // get initial rank
        let enum_faces: u64 = faces.iter().enumerate().fold(0, |acc: u64, (i, &val)| {
            acc + (2_u64.pow(val) - 1) * 16_u64.pow(i as u32)
        });
        println!("{enum_faces}");
        let mut rank = Hand::check_pairs(enum_faces);
        println!("Rank: {rank}");
        // check for straight
        if Hand::check_straight(enum_faces, faces) {
            rank = 6;
        } else if Hand::check_ace_low_straight(faces) {
            rank = 5;
        }
        println!("Rank: {rank}");

        // check for flush
        if Hand::check_flush(suits) {
            // check for straight flush
            if rank == 6 || rank == 5 {
                rank += 5;
            } else {
                rank = 7
            };
        }
        println!("Rank: {rank}");

        rank
    }

    fn get_suits(cards: &str) -> Vec<char> {
        let mut suits = vec![];
        cards
            .split(" ")
            .for_each(|s| suits.push(s.chars().last().unwrap()));
        suits
    }

    fn get_faces(cards: &str) -> [u32; 15] {
        let mut faces: [u32; 15] = [0; 15];
        for card in cards.split(' ') {
            let index = match card.chars().nth(0).unwrap() {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                '1' => 10,
                n => n.to_digit(10).unwrap(),
            };
            faces[index as usize] += 1;
        }
        faces
    }

    fn check_pairs(_enum: u64) -> u32 {
        match _enum % 15 {
            1 => 9,
            5..=7 => (_enum % 15) as u32 - 4,
            9 => 4,
            10 => 8,
            _ => 0,
        }
    }

    fn check_straight(_enum: u64, faces: [u32; 15]) -> bool {
        let first_card = faces.iter().position(|&x| x != 0).unwrap();
        _enum / (16_u64.pow(first_card as u32)) == 69905
    }

    fn check_ace_low_straight(faces: [u32; 15]) -> bool {
        if *faces.last().unwrap() == 0 {
            return false;
        }
        let mut new_faces = faces.clone();
        new_faces[1] = faces[faces.len() - 1];
        new_faces[faces.len() - 1] = 0;
        let new_enum: u64 = new_faces.iter().enumerate().fold(0, |acc: u64, (i, &val)| {
            acc + (2_u64.pow(val) - 1) * 16_u64.pow(i as u32)
        });
        Hand::check_straight(new_enum, new_faces)
    }

    fn check_flush(s: Vec<char>) -> bool {
        s.windows(2).all(|p| p[0] == p[1])
    }

    pub fn compare(&self, cards: &str) -> Option<bool> {
        let self_face = Hand::get_faces(self.cards);
        let other_face = Hand::get_faces(cards);

        match self.rank {
            8 | 9 => {
                let pairs: (u32, u32) = (self.rank - 5, 10 - self.rank);
                let self_pairs: (usize, usize) = (
                    self_face.iter().position(|&x| x == pairs.0).unwrap(),
                    self_face.iter().position(|&x| x == pairs.1).unwrap(),
                );
                let other_pairs: (usize, usize) = (
                    other_face.iter().position(|&x| x == pairs.0).unwrap(),
                    other_face.iter().position(|&x| x == pairs.1).unwrap(),
                );
                if self_pairs == other_pairs {
                    return None;
                } else {
                    if self_pairs.0 == other_pairs.0 {
                        return Some(self_pairs.1 > other_pairs.1);
                    } else {
                        return Some(self_pairs.0 > other_pairs.0);
                    }
                }
            }

            _ => {
                for i in (0..self_face.len()).rev() {
                    println!("{i}");
                    if self_face[i] != other_face[i] {
                        return Some(self_face[i] > other_face[i]);
                    }
                }
            }
        }
        None
    }
}
