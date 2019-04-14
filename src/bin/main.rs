extern crate risk_odds;

use risk_odds::{percentage, Attack, Die, Score};

use std::env;
use std::process;
use std::thread;

const DEFAULT_ATTACK_COUNT: i64 = 100_000_000;
const DEFAULT_THREAD_COUNT: i32 = 1;

/// Program entry point
///
/// Takes two optional command-line parameters:
///
/// - number of attack rolls to simulate (default 100 million)
/// - number of threads (default 1)
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 3 {
        print_help_and_exit(&args[0]);
    }

    let attack_count = if args.len() >= 2 {
        match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error: invalid attack-count argument \"{}\"", args[1]);
                print_help_and_exit(&args[0]);
            }
        }
    } else {
        DEFAULT_ATTACK_COUNT
    };

    let thread_count = if args.len() >= 3 {
        match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error: invalid thread-count argument \"{}\"", args[2]);
                print_help_and_exit(&args[0]);
            }
        }
    } else {
        DEFAULT_THREAD_COUNT
    };

    simulate_and_report(attack_count, thread_count);
}

/// Print usage message and exit with failure code.
fn print_help_and_exit(program_name: &str) -> ! {
    eprintln!("usage: {} [[ATTACKS] THREADS]", program_name);
    eprintln!("  Default attack count is {}.", DEFAULT_ATTACK_COUNT);
    eprintln!("  Default thread count is {}.", DEFAULT_THREAD_COUNT);
    process::exit(-1)
}

/// Simulate the attacks and report the results.
fn simulate_and_report(attack_count: i64, thread_count: i32) {
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
fn simulate_attacks(count: i64) -> (i64, i64, i64) {
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
