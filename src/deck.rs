use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

impl Suit {
    pub fn to_char(&self) -> char {
        match *self {
            Suit::Club    => { 'C' },
            Suit::Spade   => { 'S' },
            Suit::Heart   => { 'H' },
            Suit::Diamond => { 'D' },
        }
    }

    pub fn from_char(input: char) -> Result<Suit, &'static str> {
        match input.to_ascii_lowercase() {
            'c' => { Ok(Suit::Club) },
            's' => { Ok(Suit::Spade) },
            'h' => { Ok(Suit::Heart) },
            'd' => { Ok(Suit::Diamond) },
            _   => { Err("Invalid Suit Char") }  
        }
    }

    pub fn index(&self) -> usize {
        match *self {
            Suit::Club    => { 0 },
            Suit::Spade   => { 1 },
            Suit::Heart   => { 2 },
            Suit::Diamond => { 3 },
        }
    }

    pub fn from_index(index: usize) -> Result<Suit, &'static str> {
        match index {
            0  => { Ok(Suit::Club) },
            1  => { Ok(Suit::Spade) },
            2  => { Ok(Suit::Heart) },
            3  => { Ok(Suit::Diamond) },
            _  => { Err("Invalid Suit Index") }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn to_char(&self) -> char {
        match *self {
            Rank::Ace   => { 'A' },
            Rank::Two   => { '2' },
            Rank::Three => { '3' },
            Rank::Four  => { '4' },
            Rank::Five  => { '5' },
            Rank::Six   => { '6' },
            Rank::Seven => { '7' },
            Rank::Eight => { '8' },
            Rank::Nine  => { '9' },
            Rank::Ten   => { 'T' },
            Rank::Jack  => { 'J' },
            Rank::Queen => { 'Q' },
            Rank::King  => { 'K' },
        }
    }

    pub fn from_char(input: char) -> Result<Rank, &'static str> {
        match input.to_ascii_lowercase() {
            'A' => { Ok(Rank::Ace  ) },
            '2' => { Ok(Rank::Two  ) },
            '3' => { Ok(Rank::Three) },
            '4' => { Ok(Rank::Four ) },
            '5' => { Ok(Rank::Five ) },
            '6' => { Ok(Rank::Six  ) },
            '7' => { Ok(Rank::Seven) },
            '8' => { Ok(Rank::Eight) },
            '9' => { Ok(Rank::Nine ) },
            'T' => { Ok(Rank::Ten  ) },
            'J' => { Ok(Rank::Jack ) },
            'Q' => { Ok(Rank::Queen) },
            'K' => { Ok(Rank::King ) },
            _   => { Err("Invalid Rank!") }
        }
    }

    pub fn index(&self) -> usize {
        match *self {
            Rank::Ace   => { 0 },
            Rank::Two   => { 1 },
            Rank::Three => { 2 },
            Rank::Four  => { 3 },
            Rank::Five  => { 4 },
            Rank::Six   => { 5 },
            Rank::Seven => { 6 },
            Rank::Eight => { 7 },
            Rank::Nine  => { 8 },
            Rank::Ten   => { 9 },
            Rank::Jack  => { 10 },
            Rank::Queen => { 11 },
            Rank::King  => { 12 },
        }
    }

    pub fn from_index(index: usize) -> Result<Rank, &'static str> {
        match index {
            0  => { Ok(Rank::Ace  ) },
            1  => { Ok(Rank::Two  ) },
            2  => { Ok(Rank::Three) },
            3  => { Ok(Rank::Four ) },
            4  => { Ok(Rank::Five ) },
            5  => { Ok(Rank::Six  ) },
            6  => { Ok(Rank::Seven) },
            7  => { Ok(Rank::Eight) },
            8  => { Ok(Rank::Nine ) },
            9  => { Ok(Rank::Ten  ) },
            10 => { Ok(Rank::Jack ) },
            11 => { Ok(Rank::Queen) },
            12 => { Ok(Rank::King ) },
            _  => { Err("Invalid Rank Index") }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Default for Card {
    fn default() -> Card {
        Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        }
    }
}

pub struct Deck {
    pub cards: [Card; 52],
    pub in_play: [bool; 52],
    pub drawn_cards: u8
}

impl Default for Deck {
    //Initialize deck to a default 52 size pack of cards
    fn default() -> Deck {
        let mut temp_cards = [Card::default(); 52];
        for i in 0..3 {
            for j in 0..12 {
                temp_cards[i*13 + j] = Card {
                        suit:  Suit::from_index(i).unwrap(),
                        rank:  Rank::from_index(j).unwrap(),
                };
            }
        }

        Deck{
            cards: temp_cards,
            in_play: [false; 52],
            drawn_cards: 0,
        }
    }
}

impl Deck {
    pub fn draw_shuffled(&mut self) -> Result<Card, &str> {
        if self.drawn_cards < 52 {
            let mut rng = rand::thread_rng();
            let mut num: i32 = rng.gen_range(0..51);
            
            while self.in_play[num as usize] {
                num = rng.gen_range(0..51);
            }
        
            self.in_play[num as usize] = true;
            return Ok(Card {
                suit: Suit::from_index((num / 13) as usize).unwrap(),
                rank: Rank::from_index((num % 13) as usize).unwrap(),
            });
        }
        Err("Deck Empty!")
    }
    //Draw from deck in a consistent order
    //Does not adhere to standard ordering from a freshly opened real life deck
    pub fn draw_unshuffled(&mut self) -> Result<Card, &str> {
        if self.drawn_cards < 52 {
            self.in_play[self.drawn_cards as usize];
            let current_index = self.drawn_cards;
            self.drawn_cards += 1;

            return Ok(Card {
                suit: Suit::from_index((current_index / 13) as usize).unwrap(),
                rank: Rank::from_index((current_index % 13) as usize).unwrap(),
            });
        }
        Err("Deck Empty")
    }
    
    pub fn return_card(&mut self, card: Card){
        let card_index = card.suit.index() * 13 + card.rank.index();
        self.in_play[card_index] = false;
        if self.drawn_cards > 0 {
           self. drawn_cards -= 1;
        }   
    }

    pub fn reset(&mut self) {
        for i in 0..51 {
            self.in_play[i] = false;
        }
    }
}