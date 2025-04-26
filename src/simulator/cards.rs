use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Debug)]
pub struct CardDeck {
    cards: Vec<Card>,
    last_drawn: Vec<Card>,
    rng: Xoshiro256PlusPlus,
}

impl Default for CardDeck {
    fn default() -> Self {
        Self::new()
    }
}

impl CardDeck {
    pub fn new() -> Self {
        use rand::SeedableRng as _; // for seed_from_u64()
        use rand::seq::SliceRandom as _; // for shuffle()

        let last_drawn = Vec::new();
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(rand::random());
        let mut cards: Vec<_> = Card::deck_iter().collect();
        cards.shuffle(&mut rng);
        Self {
            cards,
            last_drawn,
            rng,
        }
    }

    fn refill(&mut self) {
        use rand::seq::SliceRandom as _; // for shuffle()

        let mut cards: Vec<_> = Card::deck_iter()
            .filter(|card| !self.last_drawn.contains(card))
            .collect();
        cards.shuffle(&mut self.rng);
        self.cards = cards;
    }

    pub fn new_round(&mut self) {
        let refill = self.last_drawn.contains(&Card::JOKER);
        self.last_drawn.clear();
        if refill {
            self.refill();
        }
    }

    pub fn draw(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            return card;
        }
        self.refill();
        if let Some(card) = self.cards.pop() {
            return card;
        }
        self.last_drawn.clear();
        self.refill();
        self.cards.pop().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(u8);

impl Card {
    const JOKER: Self = Card(52);

    fn new(i: u8) -> Self {
        assert!(i < 53);
        Self(i)
    }

    fn deck_iter() -> impl Iterator<Item = Self> {
        (0..=52).map(Card::new).chain(std::iter::once(Card::JOKER))
    }

    pub fn is_joker(self) -> bool {
        self.0 == 52
    }

    pub fn suit(self) -> Suit {
        match self.0 {
            0..4 => Suit::Two,
            4..8 => Suit::Three,
            8..12 => Suit::Four,
            12..16 => Suit::Five,
            16..20 => Suit::Six,
            20..24 => Suit::Seven,
            24..28 => Suit::Eight,
            28..32 => Suit::Nine,
            32..36 => Suit::Ten,
            36..40 => Suit::Jack,
            40..44 => Suit::Queen,
            44..48 => Suit::King,
            48..52 => Suit::Ace,
            52 => Suit::Joker,
            53.. => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    Joker = 15,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_cards_are_sorted() {
        // this test is just needed to enable us to use is_sorted() later to ensure something is shuffled
        let cards: Vec<_> = Card::deck_iter().collect();
        assert!(cards.is_sorted());
    }

    #[test]
    fn test_carddeck_starts_shuffled() {
        let deck = CardDeck::new();
        assert!(!deck.cards.is_sorted());
    }

    #[test]
    fn test_carddeck_is_shuffled_on_refill() {
        let mut deck = CardDeck::new();
        deck.cards.clear();
        deck.refill();
        assert!(!deck.cards.is_empty());
        assert!(!deck.cards.is_sorted());
    }

    #[test]
    fn test_carddeck_contains_two_jokers() {
        let deck = CardDeck::new();
        let num_jokers = deck.cards.iter().filter(|card| card.is_joker()).count();
        assert_eq!(num_jokers, 2);
    }

    #[test]
    fn test_carddeck_containts_54_cards() {
        let deck = CardDeck::new();
        assert_eq!(deck.cards.len(), 54);
    }

    #[test]
    fn test_carddeck_refills_after_joker() {
        let mut deck = CardDeck::new();

        deck.cards.sort();
        assert_eq!(deck.draw(), Card::JOKER);
        assert_eq!(deck.draw(), Card::JOKER);

        deck.new_round();

        let num_cards = deck.cards.len();
        let num_jokers = deck.cards.iter().filter(|card| card.is_joker()).count();
        assert_eq!(num_cards, 52);
        assert_eq!(num_jokers, 0);
    }

    #[test]
    fn test_carddeck_refills_after_empty() {
        let mut deck = CardDeck::new();
        deck.cards.clear();
        deck.draw();
        let num_cards = deck.cards.len();
        assert_eq!(num_cards, 53);
    }

    #[test]
    fn test_carddeck_new_round_does_not_alter_initial_state() {
        let mut deck = CardDeck::new();
        let cards = deck.cards.clone();

        deck.new_round();
        assert_eq!(deck.cards, cards);
    }

    #[test]
    fn test_suit() {
        assert!(Suit::Three < Suit::Four);
        assert!(Suit::Four < Suit::Ten);
        assert!(Suit::Ten < Suit::Jack);
        assert!(Suit::Jack < Suit::Ace);
        assert!(Suit::King < Suit::Joker);
        assert_eq!(Card::JOKER.suit(), Suit::Joker);
    }
}
