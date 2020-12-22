use itertools::izip;
use itertools::Itertools;
use queue::Queue;
use std::str::FromStr;

pub(crate) fn solve_day22() -> u32 {
    let input_file = include_str!("../resources/day22.txt");
    let cards = input_file.split("\n\n").collect_vec();

    let player1 = cards[0]
        .lines()
        .skip(1)
        .map(|s| u32::from_str(s).unwrap())
        .collect_vec();
    let player2 = cards[1]
        .lines()
        .skip(1)
        .map(|s| u32::from_str(s).unwrap())
        .collect_vec();

    let mut player1q = Queue::new();
    let mut player2q = Queue::new();

    for (p1, p2) in izip!(player1, player2) {
        player1q.queue(p1).unwrap();
        player2q.queue(p2).unwrap();
    }

    while !player1q.is_empty() && !player2q.is_empty() {
        let card1 = player1q.dequeue().unwrap();
        let card2 = player2q.dequeue().unwrap();
        if card1 > card2 {
            player1q.queue(card1).unwrap();
            player1q.queue(card2).unwrap();
        } else if card1 < card2 {
            player2q.queue(card2).unwrap();
            player2q.queue(card1).unwrap();
        } else {
            unreachable!()
        }
    }

    let mut result = 0;
    if player1q.is_empty() {
        for (card, mul) in izip!(player2q.vec(), (1..=player2q.len()).rev()) {
            result += card * (mul as u32)
        }
    } else if player2q.is_empty() {
        for (card, mul) in izip!(player1q.vec(), (1..=player1q.len()).rev()) {
            result += card * (mul as u32)
        }
    } else {
        unreachable!()
    }

    result
}
