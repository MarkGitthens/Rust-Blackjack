use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

//Choose an undrawn card and return it's face/value as a tuple
fn draw_card(deck: &mut [bool; 52]) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut num: i32 = rng.gen_range(0..51);

    while deck[num as usize] {
        num = rng.gen_range(0..51);
    }

    deck[num as usize] = true;

    ((num/13) as usize, (num%13) as usize)
}

fn calculate_hand_score(hand: &[(usize, usize);10], hand_size: usize) -> usize {
    let value_trans_table = [10,2,3,4,5,6,7,8,9,10,10,10,10];
    let mut num_aces: u8 = 0;
    let mut score: usize = 0;

    for i in 0..hand_size {
        if hand[i].1 == 0 {
            num_aces += 1;
        }

        score += value_trans_table[hand[i].1];
    }

    //Get largest legal value as long as we have aces
    while num_aces != 0 && score > 21 {
        score -= 9;
        num_aces -= 1;
    }

    score
}

fn reset_deck(deck: &mut [bool]) {
    for i in deck {
        *i = false;
    }
}

#[inline(always)]
fn draw_card_to_hand(deck: &mut [bool; 52], hand: &mut [(usize, usize); 10], hand_count: &mut usize) {
    hand[*hand_count] = draw_card(deck);   
    *hand_count += 1;
}

fn main() {
    let value_str_table = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    let face_str_table = ["D", "S", "H", "C"];

    let mut card_in_play = [false; 52];

    let mut player_hand: [(usize, usize); 10] = [(0, 0); 10];
    let mut dealer_hand: [(usize, usize); 10] = [(0, 0); 10];

    let mut p_hand_count = 0;
    let mut d_hand_count = 0;

    let mut user_response: String = String::new();

    loop {
        let mut p_score;

        draw_card_to_hand(&mut card_in_play, &mut dealer_hand, &mut d_hand_count);
        draw_card_to_hand(&mut card_in_play, &mut dealer_hand, &mut d_hand_count);

        let d_score = calculate_hand_score(&dealer_hand, d_hand_count);

        draw_card_to_hand(&mut card_in_play, &mut player_hand, &mut p_hand_count);
        draw_card_to_hand(&mut card_in_play, &mut player_hand, &mut p_hand_count);

        p_score = calculate_hand_score(&player_hand, p_hand_count);
        
        println!("Dealer: {}/{} {}/{} score: {}",
            face_str_table[dealer_hand[0].0], value_str_table[dealer_hand[0].1],
            face_str_table[dealer_hand[1].0], value_str_table[dealer_hand[1].1],
            d_score);

        println!("You: {}/{} {}/{} score: {}",
            face_str_table[player_hand[0].0], value_str_table[player_hand[0].1],
            face_str_table[player_hand[1].0], value_str_table[player_hand[1].1],
            p_score);

        
        loop {
            println!("(H)it or (S)tay: ");

            user_response.clear();
            io::stdin().read_line(&mut user_response).expect("Invalid input");

            let first_char = user_response.chars().next().unwrap();

            println!("{}", first_char);
            if first_char == 'h' {
                draw_card_to_hand(&mut card_in_play, &mut player_hand, &mut p_hand_count);

                p_score = calculate_hand_score(&player_hand, p_hand_count);

                println!("Dealer: {}/{} {}/{} score: {}",
                    face_str_table[dealer_hand[0].0], value_str_table[dealer_hand[0].1],
                    face_str_table[dealer_hand[1].0], value_str_table[dealer_hand[1].1],
                    d_score);

                println!("You: {}/{} {}/{} score: {}",
                    face_str_table[player_hand[0].0], value_str_table[player_hand[0].1],
                    face_str_table[player_hand[1].0], value_str_table[player_hand[1].1],
                    p_score);
            } else {
                break;
            }
        }

        match p_score.cmp(&d_score) {
            Ordering::Greater => println!("You win!"),
            Ordering::Less => println!("Dealer wins!"),
            Ordering::Equal =>  println!("Push"),
        };       

        reset_deck(&mut card_in_play);
        p_hand_count = 0;
        d_hand_count = 0;
    }
}