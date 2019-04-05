extern crate risk_odds;

use risk_odds::{percentage, Attack, Die, Score};

use std::env;
use std::thread;

/// Program entry point
///
/// Takes two optional command-line parameters:
///
/// - number of attack rolls to simulate (default 100 million)
/// - number of threads (default 1)
fn main() {
    let mut attack_count = 100_000_000;
    let mut thread_count = 1;

    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        attack_count = args[1].parse().unwrap();
    }
    if args.len() >= 3 {
        thread_count = args[2].parse().unwrap();
    }

    // Spawn threads
    let mut threads = Vec::new();
    for _ in 0..thread_count {
        let count = attack_count;
        threads.push(thread::spawn(move || simulate_attacks(count)));
    }

    // Gather thread results
    let mut total_wins = 0;
    let mut total_losses = 0;
    let mut total_ties = 0;
    for thread in threads {
        let (wins, losses, ties) = thread.join().unwrap();
        total_wins += wins;
        total_losses += losses;
        total_ties += ties;
    }

    report_results(total_wins, total_losses, total_ties);
}

/// Simulate the specified number of attacks.
///
/// Returns a tuple `(wins, losses, ties)`.
fn simulate_attacks(count: i32) -> (i32, i32, i32) {
    let mut die = Die::default();

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
