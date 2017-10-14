extern crate rand;

use std::env;
use rand::Rng;

fn main() {
    let mut args = env::args();
    let computer_hand = rand::thread_rng().gen_range(0, 2);
    if let Some(hand) = args.nth(1) {
        let hand_int = hand.parse::<i32>().unwrap();
        let result = judge(&hand_int, &computer_hand);
        println!(
            "Your hand is {}, computer hand is {}",
            convert(&hand_int),
            convert(&computer_hand)
        );
        println!("{}", result);
    }
}

/// 入力された引数の手と、乱数生成されたコンピュータ側の手を比較して勝ち負けを判定します。
/// 0 = グー、チョキ = 1、パー = 2 と解釈します。
/// 判定ロジックは「(自分の手 - 相手の手 + 3) % 3」で、
/// その結果が0なら引き分け、1なら負け、2なら勝ちと判定します。
fn judge(hand_int: &i32, computer_hand: &i32) -> String {
    let answer = (hand_int - computer_hand + 3) % 3;
    match answer {
        0 => String::from("draw"),
        1 => String::from("lose..."),
        2 => String::from("win!"),
        _ => String::from("unexpected value"),
    }
}

fn convert(hand_int: &i32) -> String {
    match *hand_int {
        0 => String::from("Rock"),
        1 => String::from("Scissors"),
        2 => String::from("Paper"),
        _ => String::from("unexpected value"),
    }
}

#[test]
fn test_rand() {
    let generated = rand::thread_rng().gen_range(0, 2);
    println!("Generated: {}", generated);
    assert!(generated >= 0 && generated <= 2);
}

#[test]
fn test_convert() {
    let rock = convert(&0);
    assert!(rock == String::from("Rock"));
    let sci = convert(&1);
    assert!(sci == String::from("Scissors"));
    let paper = convert(&2);
    assert!(paper == String::from("Paper"));
    let err = convert(&3);
    assert!(err == String::from("unexpected value"));
}

#[test]
fn test_judge_draw_logic() {
    {
        let result = judge(&0, &0);
        assert!(result == String::from("draw"));
    }
    {
        let result = judge(&1, &1);
        assert!(result == String::from("draw"));
    }
    {
        let result = judge(&2, &2);
        assert!(result == String::from("draw"));
    }    
}

#[test]
fn test_judge_lose_logic() {
    {
        let result = judge(&0, &2);
        assert!(result == String::from("lose..."));
    }
    {
        let result = judge(&1, &0);
        assert!(result == String::from("lose..."));
    }
    {
        let result = judge(&2, &1);
        assert!(result == String::from("lose..."));
    }
}

#[test]
fn test_judge_win_logic() {
    {
        let result = judge(&0, &1);
        assert!(result == String::from("win!"));
    }
    {
        let result = judge(&1, &2);
        assert!(result == String::from("win!"));
    }
    {
        let result = judge(&2, &0);
        assert!(result == String::from("win!"));
    }
}
