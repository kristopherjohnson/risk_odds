extern crate risk_odds;

use risk_odds::{percentage, Attack, Die, Score};
use std::env;

fn main() {
    let mut attack_count = 100_000_000;

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        attack_count = args[1].parse().unwrap();
    } else if args.len() != 1 {
        eprintln!("usage: {} [COUNT]", args[0]);
        return;
    }

    let (wins, losses, ties) = simulate_attacks(attack_count);

    report_results(wins, losses, ties);
}

/// Simulate the specified number of attacks.
///
/// Returns a tuple `(wins, losses, ties)`.
fn simulate_attacks(count: i32) -> (i32, i32, i32) {
    let mut die = Die::new();

    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;

    for _ in 0..count {
        match Attack::with_die(&mut die).attacker_score() {
            Score::Win => wins += 1,
            Score::Loss => losses += 1,
            Score::Tie => ties += 1,
        }
    }

    (wins, losses, ties)
}

/// Print the results of a series of attacks.
fn report_results(wins: i32, losses: i32, ties: i32) {
    let total = wins + losses + ties;

    println!(
        "Wins   {:12} {:6.2}%\nLosses {:12} {:6.2}%\nTies   {:12} {:6.2}%\nTotal  {:12} {:6.2}%",
        wins,
        percentage(wins, total),
        losses,
        percentage(losses, total),
        ties,
        percentage(ties, total),
        total,
        percentage(total, total)
    );
}
