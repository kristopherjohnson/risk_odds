extern crate risk_odds;

use risk_odds::{percentage, Attack, Score};

fn main() {
    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;

    // Try every combination of five dice.
    for a1 in 1..=6 {
        for a2 in 1..=6 {
            for a3 in 1..=6 {
                for d1 in 1..=6 {
                    for d2 in 1..=6 {
                        match Attack::with_die_rolls(a1, a2, a3, d1, d2).attacker_score() {
                            Score::Win => wins += 1,
                            Score::Loss => losses += 1,
                            Score::Tie => ties += 1,
                        }
                    }
                }
            }
        }
    }

    report_results(wins, losses, ties);
}

/// Print the results of a series of attacks.
fn report_results(wins: i64, losses: i64, ties: i64) {
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
