use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        //  List of 'suits' -  'hearts' , 'spades'
        //  List of 'values' -  'ace' , 'two', 'three'
        // Double nested for loop
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);

                cards.push(card);
            }
        }

        // Vec::new()
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Heres your deck: {:#?}", deck);
    // Probably need to add error handling !!
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
