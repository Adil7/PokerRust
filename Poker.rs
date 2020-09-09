// Adil Mahmood 500865290

use std::collections::HashMap;

// fn main(){
//     let perm:[u32;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let winner:[String;5] = deal(perm);

//     println!("{:?}", deal([9, 8, 7, 6, 5, 4, 3, 2, 1]));
//     println!("{:?}", deal([40, 41, 42, 43, 48, 49, 50, 51, 52]));
//     println!("{:?}", deal([40, 41, 27, 28, 1, 14, 15, 42, 29]));
//     println!("{:?}", deal([30, 13, 27, 44, 12, 17, 33, 41, 43]));
//     println!("{:?}", deal([27, 45, 3, 48, 44, 43, 41, 33, 12]));
//     println!("{:?}", deal([17, 31, 30, 51, 44, 43, 41, 33, 12]));
//     println!("{:?}", deal([17, 39, 30, 52, 44, 25, 41, 51, 12]));
//     println!("{:?}", deal([11, 25, 9, 39, 50, 48, 3, 49, 45]));
//     println!("{:?}", deal([50, 26, 39, 3, 11, 27, 20, 48, 52]));
//     println!("{:?}", deal([40, 52, 46, 11, 48, 27, 29, 32, 37]));

//     println!("{:?}", deal([23, 32, 11, 38, 44, 49, 50, 22, 9])); //p1 wins
// }

pub fn deal(perm:[u32;9])->[String;5] {

    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();
    hand1.push(perm[0]);
    hand1.push(perm[2]);
    hand2.push(perm[1]);
    hand2.push(perm[3]);
    for n in 4..9 {
        hand1.push(perm[n]);
        hand2.push(perm[n]);
    }

    let p1 = get_player_hand(&hand1);
    let p2 = get_player_hand(&hand2);

    let mut res = Vec::new();

    if get_hand_rank(&p1) < get_hand_rank(&p2) {
        res = p1.clone();
    } else if get_hand_rank(&p1) > get_hand_rank(&p2) {
        res = p2.clone();
    }

    if get_hand_rank(&p1) == get_hand_rank(&p2) {
        
        let x = get_high_card(&p1);
        let y = get_high_card(&p2);
        if x > y {
            res = p1.clone();
        } else if x < y {
            res = p2.clone();
        } else {
            let x = get_second_high_card(&p1);
            let y = get_second_high_card(&p2);
            if x > y {
                res = p1.clone();
            } else if x < y {
                res = p2.clone();
            } else {
                let x = get_third_high_card(&p1);
                let y = get_third_high_card(&p2);
                if x > y {
                    res = p1.clone();
                } else if x < y {
                    res = p2.clone();
                } else {
                    let x = get_fourth_high_card(&p1);
                    let y = get_fourth_high_card(&p2);
                    if x > y {
                        res = p1.clone();
                    } else if x < y {
                        res = p2.clone();
                    } else {
                        let x = get_fifth_high_card(&p1);
                        let y = get_fifth_high_card(&p2);
                        if x > y {
                            res = p1.clone();
                        } else if x < y {
                            res = p2.clone();
                        } else {
                            res = p1.clone();
                        }
                    }
                }
            }
        }



    }

    return hand_to_string(&res);
}

fn hand_to_string(hand: &Vec<u32>) -> [String;5]{
    let mut result: [String; 5] = Default::default();
    let clubs = String::from("C");
    let diamonds = String::from("D");
    let hearts = String::from("H");
    let spades = String::from("S");   
    let mut j = 0;
    for i in result.iter_mut(){
        let mut rank = hand[j] % 13;
        if rank == 0 { rank = 13; };
        if get_suit(hand[j]) == 1{
            *i = format!("{}{}",rank,clubs);
        } else if get_suit(hand[j]) == 2{
            *i = format!("{}{}",rank,diamonds);
        } else if get_suit(hand[j]) == 3{
            *i = format!("{}{}",rank,hearts);
        } else if get_suit(hand[j]) == 4{
            *i = format!("{}{}",rank,spades);
        }
        j+=1;
    }
    result.sort();
    return result
}

fn get_player_hand(cards: &Vec<u32>) -> Vec<u32> {
    let mut cur_hand = Vec::<u32>::new();
    let mut res = Vec::<u32>::new();
    let mut cur_rank = 0;
    let mut max_rank = 99;

    for first in 0..7  {        
        for x in (first+1)..7 {
            for i in 0..7 {
                if first != i && x != i {
                    cur_hand.push(cards[i]);
                }
            }

            cur_rank = get_hand_rank(&cur_hand);

            if cur_rank == max_rank {
                let x = get_high_card(&cur_hand);
                let y = get_high_card(&res);
                if x > y {
                    res = cur_hand.clone();
                } else if x == y {
                    let x = get_second_high_card(&cur_hand);
                    let y = get_second_high_card(&res);
                    if x > y {
                        res = cur_hand.clone();
                    } else if x == y {
                        let x = get_third_high_card(&cur_hand);
                        let y = get_third_high_card(&res);
                        if x > y {
                            res = cur_hand.clone();
                        } else if x == y {
                            let x = get_fourth_high_card(&cur_hand);
                            let y = get_fourth_high_card(&res);
                            if x > y {
                                res = cur_hand.clone();
                            } else if x == y {
                                let x = get_fifth_high_card(&cur_hand);
                                let y = get_fifth_high_card(&res);
                                if x > y {
                                    res = cur_hand.clone();
                                } 
                            }
                        }
                    }
                }
            }
            if cur_rank < max_rank {
                res = cur_hand.clone();
                max_rank = cur_rank;
            }
            cur_hand = Vec::<u32>::new();
        }
    }

    return res;
}

fn get_high_card(cards: &Vec<u32>) -> u32 {
    let rank = get_hand_rank(cards);
    let mut hand = norm_hand(cards);
    hand.sort();

    let mut map = HashMap::new();

    let first = hand[0];
    let last = hand[4];
    
    if rank == 1 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[4]; 
    }

    if rank == 2 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 4 {
                return *key;
            }
        }
    }

    if rank == 3 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 3 {
                return *key;
            }
        }
    }

    if rank == 4 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
        }
        hand.sort();
        return hand[4];
    }

    if rank == 5 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[4]; 
    }

    if rank == 6 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 3 {
                return *key;
            }
        }
    }

    if rank == 7 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in map.iter() {
            if *val == 2 {
                hand_short.push(*key)
            }
        }
        hand_short.sort();
        return hand_short[1];
    }

    if rank == 8 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 2 {
                return *key;
            }
        }
    }

    if rank == 9 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
        }
        hand.sort();
        return hand[4];
    }
    return 0;
}

fn get_second_high_card(cards: &Vec<u32>) -> u32 {
    let rank = get_hand_rank(cards);
    let mut hand = norm_hand(cards);
    hand.sort();

    let mut map = HashMap::new();

    let first = hand[0];
    let last = hand[4];
    
    if rank == 1 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[3]; 
    }

    if rank == 2 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 1 {
                return *key;
            }
        }
    }

    if rank == 3 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 2 {
                return *key;
            }
        }
    }

    if rank == 4 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
        }
        hand.sort();
        return hand[3];
    }

    if rank == 5 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[3]; 
    }

    if rank == 6 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in &map {
            if *val == 1 {
                hand_short.push(*key);
            }
        }
        hand_short.sort();
        return hand_short[1];
    }

    if rank == 7 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in map.iter() {
            if *val == 2 {
                hand_short.push(*key);
            }
        }
        hand_short.sort();
        return hand_short[0];
    }

    if rank == 8 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in map.iter() {
            if *val == 1 {
                hand_short.push(*key);
            }
        }
        hand_short.sort();
        return hand_short[2];
    }

    if rank == 9 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
        }
        hand.sort();
        return hand[3];
    }

    return 0;
}

fn get_third_high_card(cards: &Vec<u32>) -> u32 {
    let rank = get_hand_rank(cards);
    let mut hand = norm_hand(cards);
    hand.sort();

    let mut map = HashMap::new();

    let first = hand[0];
    let last = hand[4];
    
    if rank == 1 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[2]; 
    }

    if rank == 4 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
        }
        hand.sort();
        return hand[2];
    }

    if rank == 5 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[2]; 
    }

    if rank == 6 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in &map {
            if *val == 1 {
                hand_short.push(*key);
            }
        }
        hand_short.sort();
        return hand_short[0];
    }

    if rank == 7 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        for (key, val) in map.iter() {
            if *val == 1 {
                return *key;
            }
        }
    }

    if rank == 8 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in map.iter() {
            if *val == 1 {
                hand_short.push(*key)
            }
        }
        hand_short.sort();
        return hand_short[1];
    }

    if rank == 9 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
        }
        hand.sort();
        return hand[2];
    }
    return 0;
}

fn get_fourth_high_card(cards: &Vec<u32>) -> u32 {
    let rank = get_hand_rank(cards);
    let mut hand = norm_hand(cards);
    hand.sort();

    let mut map = HashMap::new();

    let first = hand[0];
    let last = hand[4];
    
    if rank == 1 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[1]; 
    }

    if rank == 4 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
        }
        hand.sort();
        return hand[1];
    }

    if rank == 5 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[1]; 
    }

    if rank == 8 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
            *map.entry(hand[i]).or_insert(0) += 1;
        }
        let mut hand_short = Vec::<u32>::new();
        for (key, val) in map.iter() {
            if *val == 1 {
                hand_short.push(*key);
            }
        }
        hand_short.sort();
        return hand_short[0];
    }

    if rank == 9 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
        }
        hand.sort();
        return hand[1];
    }

    return 0;
}

fn get_fifth_high_card(cards: &Vec<u32>) -> u32 {
    let rank = get_hand_rank(cards);
    let mut hand = norm_hand(cards);
    hand.sort();

    let first = hand[0];
    let last = hand[4];
    
    if rank == 1 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[0]; 
    }

    if rank == 4 {
        for i in 0..5 {
            if hand[i] == 1 { hand[i] = 14 };
        }
        hand.sort();
        return hand[0];
    }

    if rank == 5 {
        if first == 1 && last == 13 {
            hand.remove(0);
            hand.push(14);
        }
        hand.sort();
        return hand[0]; 
    }

    if rank == 9 {
        for i in 0..5 {
            if hand[i] == 1 {hand[i] = 14};
        }
        hand.sort();
        return hand[0];
    }
    return 0;
}

fn get_hand_rank(hand: &Vec<u32>) -> u32 {
    if is_straight_flush(hand) {return 1;}
    else if is_four_of_a_kind(hand) {return 2;}
    else if is_full_house(hand) {return 3;}
    else if is_flush(hand) {return 4;}
    else if is_straight(hand) {return 5;}
    else if is_three_of_a_kind(hand) {return 6;}
    else if is_two_pair(hand) {return 7;}
    else if is_one_pair(hand) {return 8;}
    else {return 9;}
}

fn is_straight_flush(cards: &Vec<u32>) -> bool{
    is_straight(cards) && is_flush(cards)
}

fn is_four_of_a_kind(cards: &Vec<u32>) -> bool{
    let mut hand = norm_hand(cards);
    hand.sort();
    
    let mut map = HashMap::new();
    for i in 0..5 {
        *map.entry(hand[i]).or_insert(0) += 1;
    }
    
    if map.len() != 2 {return false;}

    let val_4_exists = map.values().any(|&x| x == 4);
    if val_4_exists {
        return true;
    } 

    return false;
}

fn is_full_house(cards: &Vec<u32>) -> bool{
    let hand = norm_hand(cards);    
    let mut map = HashMap::new();
    for i in 0..5 {
        *map.entry(hand[i]).or_insert(0) += 1;
    }
    // for (key, value) in map.iter() {
    //     println!("{} - {}", key, value);
    // }
    if map.len() != 2 {return false;}

    let val_3_exists = map.values().any(|&x| x == 3);
    let val_2_exists = map.values().any(|&x| x == 2);
    if val_3_exists && val_2_exists {
        return true;
    } 

    return false;
}

fn is_flush(cards: &Vec<u32>) -> bool{
    let cur_suit = get_suit(cards[0]);
    for i in 1..5 {
        if cur_suit != get_suit(cards[i]) {return false;}
    }
    return true;
}

fn is_straight(cards: &Vec<u32>) -> bool{
    let mut hand = norm_hand(cards);
    hand.sort();
    let first = hand[0];
    let last = hand[4]; 

    if first == 1 && last == 13 {
        hand.remove(0);
        hand.push(14);
    }

    for i in 0..4 {
        if (hand[i] + 1) != hand[i+1] {
            return false;
        }
    }
   
    return true;
}

fn is_three_of_a_kind(cards: &Vec<u32>) -> bool{
    let hand = norm_hand(cards);
    let mut map = HashMap::new();
    for i in 0..5 {
        *map.entry(hand[i]).or_insert(0) += 1;
    }

    if map.len() != 3 {
        return false;
    }

    let val_3_exists = map.values().any(|&x| x == 3);
    let val_1_exists = map.values().any(|&x| x == 1);
    //let num_of_val_1 = map.values().filter(|&x| x == &1).count();
    //println!("the value: {}", num_of_val_1);
    if val_3_exists && val_1_exists {
        return true;
    } 
    return false;
}

fn is_two_pair(cards: &Vec<u32>) -> bool{
    let hand = norm_hand(cards);
    let mut map = HashMap::new();
    for i in 0..5 {
        *map.entry(hand[i]).or_insert(0) += 1;
    }

    if map.len() != 3 {return false;}

    //let val_2_exists = map.values().any(|&x| x == 2);
    let num_of_pairs = map.values().filter(|&x| x == &2).count();

    if map.len() == 3 && num_of_pairs == 2 {
        return true;
    } 
    return false;
}

fn is_one_pair(cards: &Vec<u32>) -> bool{
    let hand = norm_hand(cards);
    let mut map = HashMap::new();
    for i in 0..5 {
        *map.entry(hand[i]).or_insert(0) += 1;
    }
    let val_2_exists = map.values().any(|&x| x == 2);
    if map.len() == 4 && val_2_exists {
        return true;
    } 
    return false;
}

fn norm_hand(hand: &Vec<u32>) -> Vec<u32>{
    let mut res = Vec::<u32>::new();
    for i in 0..5 {
        if hand[i] % 13 == 0 {
            res.push(13);
        } else {
            res.push(hand[i] % 13);
        }
    }
    res
}

fn get_suit(num: u32) -> u32 {
    if num <= 13 {return 1; }
    else if num <= 26 {return 2;}
    else if num <= 39 {return 3;}
    else {return 4;}
}