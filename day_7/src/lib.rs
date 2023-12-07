mod p1 {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[repr(u8)]
    pub enum Card {
        Ace = 14,
        King = 13,
        Queen = 12,
        Jack = 11,
        Ten = 10,
        Nine = 9,
        Eight = 8,
        Seven = 7,
        Six = 6,
        Five = 5,
        Four = 4,
        Three = 3,
        Two = 2,
    }

    impl TryFrom<u8> for Card {
        type Error = &'static str;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                14 => Ok(Self::Ace),
                13 => Ok(Self::King),
                12 => Ok(Self::Queen),
                11 => Ok(Self::Jack),
                10 => Ok(Self::Ten),
                9 => Ok(Self::Nine),
                8 => Ok(Self::Eight),
                7 => Ok(Self::Seven),
                6 => Ok(Self::Six),
                5 => Ok(Self::Five),
                4 => Ok(Self::Four),
                3 => Ok(Self::Three),
                2 => Ok(Self::Two),
                _ => Err("Invalid card value"),
            }
        }
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Self::Ace),
                'K' => Ok(Self::King),
                'Q' => Ok(Self::Queen),
                'J' => Ok(Self::Jack),
                'T' => Ok(Self::Ten),
                '9' => Ok(Self::Nine),
                '8' => Ok(Self::Eight),
                '7' => Ok(Self::Seven),
                '6' => Ok(Self::Six),
                '5' => Ok(Self::Five),
                '4' => Ok(Self::Four),
                '3' => Ok(Self::Three),
                '2' => Ok(Self::Two),
                _ => Err("Invalid card value"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Hand {
        cards: [Card; 5],
        pub bid: usize,

        map: HashMap<Card, usize>,
        len: usize,
    }

    impl Hand {
        #[must_use]
        pub fn new(cards: [Card; 5], bid: usize) -> Self {
            let mut map = HashMap::new();
            for card in cards {
                *map.entry(card).or_insert(0) += 1;
            }

            let len = map.len();

            Self {
                cards,
                bid,
                map,
                len,
            }
        }

        #[must_use]
        const fn five_of_a_kind(&self) -> bool {
            self.len == 1
        }

        #[must_use]
        fn four_of_a_kind(&self) -> bool {
            self.len == 2 && self.map.values().any(|&v| v == 4)
        }

        #[must_use]
        fn full_house(&self) -> bool {
            self.len == 2 && self.map.values().any(|&v| v == 2 || v == 3)
        }

        #[must_use]
        fn three_of_a_kind(&self) -> bool {
            self.len == 3 && self.map.values().any(|&v| v == 3)
        }

        #[must_use]
        fn two_pair(&self) -> bool {
            self.len == 3 && self.map.values().any(|&v| v == 2)
        }

        #[must_use]
        fn one_pair(&self) -> bool {
            self.len == 4 && self.map.values().any(|&v| v == 2)
        }

        #[must_use]
        const fn high_card(&self) -> bool {
            self.len == 5
        }

        #[must_use]
        fn hand_value(&self) -> u8 {
            if self.five_of_a_kind() {
                6
            } else if self.four_of_a_kind() {
                5
            } else if self.full_house() {
                4
            } else if self.three_of_a_kind() {
                3
            } else if self.two_pair() {
                2
            } else if self.one_pair() {
                1
            } else {
                0
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_value = self.hand_value();
            let other_value = other.hand_value();

            if self_value == other_value {
                for i in 0..5 {
                    if self.cards[i] == other.cards[i] {
                        continue;
                    }

                    return self.cards[i].cmp(&other.cards[i]);
                }
            }

            self_value.cmp(&other_value)
        }
    }
}

#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    use p1::Hand;

    let mut hands = input
        .lines()
        .map(|line| {
            let bid = line[6..].parse::<usize>().unwrap();
            let chars = line[..6].chars().collect::<Vec<_>>();

            Hand::new(
                [
                    chars[0].try_into().unwrap(),
                    chars[1].try_into().unwrap(),
                    chars[2].try_into().unwrap(),
                    chars[3].try_into().unwrap(),
                    chars[4].try_into().unwrap(),
                ],
                bid,
            )
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}

mod p2 {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[repr(u8)]
    pub enum Card {
        Ace = 14,
        King = 13,
        Queen = 12,
        Ten = 10,
        Nine = 9,
        Eight = 8,
        Seven = 7,
        Six = 6,
        Five = 5,
        Four = 4,
        Three = 3,
        Two = 2,
        Joker = 1,
    }

    impl TryFrom<u8> for Card {
        type Error = &'static str;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                14 => Ok(Self::Ace),
                13 => Ok(Self::King),
                12 => Ok(Self::Queen),
                10 => Ok(Self::Ten),
                9 => Ok(Self::Nine),
                8 => Ok(Self::Eight),
                7 => Ok(Self::Seven),
                6 => Ok(Self::Six),
                5 => Ok(Self::Five),
                4 => Ok(Self::Four),
                3 => Ok(Self::Three),
                2 => Ok(Self::Two),
                1 => Ok(Self::Joker),
                _ => Err("Invalid card value"),
            }
        }
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Self::Ace),
                'K' => Ok(Self::King),
                'Q' => Ok(Self::Queen),
                'T' => Ok(Self::Ten),
                '9' => Ok(Self::Nine),
                '8' => Ok(Self::Eight),
                '7' => Ok(Self::Seven),
                '6' => Ok(Self::Six),
                '5' => Ok(Self::Five),
                '4' => Ok(Self::Four),
                '3' => Ok(Self::Three),
                '2' => Ok(Self::Two),
                'J' => Ok(Self::Joker),
                _ => Err("Invalid card value"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Hand {
        cards: [Card; 5],
        pub bid: usize,

        map: HashMap<Card, usize>,
        len: usize,
    }

    impl Hand {
        #[must_use]
        pub fn new(cards: [Card; 5], bid: usize) -> Self {
            let mut map = HashMap::new();
            for card in cards {
                *map.entry(card).or_insert(0) += 1;
            }

            let len = map.len();

            Self {
                cards,
                bid,
                map,
                len,
            }
        }

        #[must_use]
        fn five_of_a_kind(&self) -> bool {
            self.len == 1 || (self.len == 2 && self.map.contains_key(&Card::Joker))
        }

        #[must_use]
        fn four_of_a_kind(&self) -> bool {
            (self.len == 2
                && !self.map.contains_key(&Card::Joker)
                && self.map.values().any(|&v| v == 4))
                || (self.len == 3
                    && (self.map.get(&Card::Joker) == Some(&1)
                        && self.map.values().any(|&v| v == 3))
                    || (self.map.get(&Card::Joker) == Some(&2)
                        && self.map.values().filter(|&&v| v == 2).count() == 2)
                    || self.map.get(&Card::Joker) == Some(&3))
        }

        #[must_use]
        fn full_house(&self) -> bool {
            (self.len == 2
                && self.map.values().any(|&v| v == 2 || v == 3)
                && !self.map.contains_key(&Card::Joker))
                || (self.len == 3
                    && self.map.get(&Card::Joker) == Some(&1)
                    && self.map.values().all(|&v| (v == 1 || v == 2) && v != 3))
        }

        #[must_use]
        fn three_of_a_kind(&self) -> bool {
            (self.len == 3
                && self.map.values().any(|&v| v == 3)
                && !self.map.contains_key(&Card::Joker))
                || (self.len == 4
                    && self.map.contains_key(&Card::Joker)
                    && self.map.values().any(|&v| v == 2))
        }

        #[must_use]
        fn two_pair(&self) -> bool {
            self.len == 3 && !self.map.contains_key(&Card::Joker)
        }

        #[must_use]
        fn one_pair(&self) -> bool {
            (self.len == 4
                && !self.map.contains_key(&Card::Joker)
                && self.map.values().any(|&v| v == 2))
                || (self.len == 5 && self.map.contains_key(&Card::Joker))
        }

        #[must_use]
        fn high_card(&self) -> bool {
            self.len == 5 && !self.map.contains_key(&Card::Joker)
        }

        #[must_use]
        fn hand_value(&self) -> u8 {
            if self.five_of_a_kind() {
                6
            } else if self.four_of_a_kind() {
                5
            } else if self.full_house() {
                4
            } else if self.three_of_a_kind() {
                3
            } else if self.two_pair() {
                2
            } else if self.one_pair() {
                1
            } else {
                0
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_value = self.hand_value();
            let other_value = other.hand_value();

            if self_value == other_value {
                for i in 0..5 {
                    if self.cards[i] == other.cards[i] {
                        continue;
                    }

                    return self.cards[i].cmp(&other.cards[i]);
                }
            }

            self_value.cmp(&other_value)
        }
    }
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    use p2::Hand;

    let mut hands = input
        .lines()
        .map(|line| {
            let bid = line[6..].parse::<usize>().unwrap();
            let chars = line[..6].chars().collect::<Vec<_>>();

            Hand::new(
                [
                    chars[0].try_into().unwrap(),
                    chars[1].try_into().unwrap(),
                    chars[2].try_into().unwrap(),
                    chars[3].try_into().unwrap(),
                    chars[4].try_into().unwrap(),
                ],
                bid,
            )
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}
