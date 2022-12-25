
use std::cmp::Ordering;
use std::io::{self, Write};
use std::process;
use std_deck::deck::*;

fn calculate_score(hand: &Vec<Card>) -> usize {
    let value_trans_table = [10,2,3,4,5,6,7,8,9,10,10,10,10];
    let mut num_aces: u8 = 0;
    let mut score: usize = 0;

    for i in 0..hand.len() {
        match hand[i].rank {
            Rank::Ace => {
                num_aces += 1;
            }
            _ => {}
        }

        score += match hand[i].rank {
            Rank::Ace => { value_trans_table[0] },
            Rank::Two => { value_trans_table[1] },
            Rank::Three => { value_trans_table[2] },
            Rank::Four => { value_trans_table[3] },
            Rank::Five => { value_trans_table[4] },
            Rank::Six => { value_trans_table[5] },
            Rank::Seven => { value_trans_table[6] },
            Rank::Eight => { value_trans_table[7] },
            Rank::Nine => { value_trans_table[8] },
            Rank::Ten => { value_trans_table[9] },
            Rank::Jack => { value_trans_table[10] },
            Rank::Queen => { value_trans_table[11] },
            Rank::King => { value_trans_table[12] },
        }
    }

    //Get largest legal value as long as we have aces
    while num_aces != 0 && score > 21 {
        score -= 9;
        num_aces -= 1;
    }

    score
}

#[inline(always)]
fn draw_card_to_hand(deck: &mut Deck, hand: &mut Vec<Card>) {
    match deck.draw_shuffled() {
        Ok(c) => { hand.push(c); },
        Err(_e) => {} 
    }
}

#[inline(always)]
fn format_hand(hand: &Vec<Card>) -> String{

    let mut output = String::new();

    for i in hand {
        let temp = String::from(format!("{}/{} ", i.rank.to_char(), i.suit.to_char())).to_owned();
        output.push_str(&temp);
    }
    return String::from(format!("{} \nscore: {}\n",
        output.to_owned(),
        calculate_score(hand)));
}

fn main() {
    let mut deck = Deck::default();
    deck.reset();   

    let mut user_response: String = String::new();

    loop {
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();

        draw_card_to_hand(&mut deck, &mut dealer_hand);
        draw_card_to_hand(&mut deck, &mut dealer_hand);

        draw_card_to_hand(&mut deck, &mut player_hand);
        draw_card_to_hand(&mut deck, &mut player_hand);
       
        println!("Player: {}", format_hand(&player_hand));
        println!("Dealer: {}", format_hand(&dealer_hand));
        
        loop {
            print!("(H)it/(S)tay/(Q)uit: ");
            io::stdout().flush().unwrap();

            user_response.clear();
            io::stdin().read_line(&mut user_response).expect("Invalid input");

            let first_char = user_response.chars().next().unwrap().to_ascii_lowercase();

            match first_char {
                'h' => {
                    draw_card_to_hand(&mut deck, &mut player_hand);

                    println!("Player: {}", format_hand(&player_hand));
                    println!("Dealer: {}", format_hand(&dealer_hand));

                    if calculate_score(&player_hand) > 21 {
                        break;
                    }
                }
                'q' => {
                    process::exit(0);
                }
                _ => {
                    break;
                }
            }
        }

        if calculate_score(&player_hand) > 21 {
            println!("You Bust!");
            println!("\n");
            continue;
        }

        match calculate_score(&player_hand).cmp(&calculate_score(&dealer_hand)) {
            Ordering::Greater => println!("You win!"),
            Ordering::Less => println!("Dealer wins!"),
            Ordering::Equal =>  println!("Push"),
        };       

        println!("\n");
        deck.reset();

        player_hand.clear();
        dealer_hand.clear();
    }
}