extern crate risk_odds;

use risk_odds::{percentage, Attack, Die, Score};

fn main() {
    let mut die = Die::new();

    let attack_count = 100_000_000;

    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;

    for _ in 1..=attack_count {
        match Attack::with_die(&mut die).attacker_score() {
            Score::Win => wins += 1,
            Score::Loss => losses += 1,
            Score::Tie => ties += 1,
        }
    }

    println!(
        "Wins\t{}\t{:.2}%\nLosses\t{}\t{:.2}%\nTies\t{}\t{:.2}%",
        wins,
        percentage(wins, attack_count),
        losses,
        percentage(losses, attack_count),
        ties,
        percentage(ties, attack_count)
    );
}
