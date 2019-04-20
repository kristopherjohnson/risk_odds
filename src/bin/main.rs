extern crate risk_odds;

use risk_odds::{percentage, Attack, Die, Score};

use std::env;
use std::process;
use std::str;
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

    let attack_count = arg_value("attack-count", &args, 1, DEFAULT_ATTACK_COUNT);
    let thread_count = arg_value("thread-count", &args, 2, DEFAULT_THREAD_COUNT);

    let (wins, losses, ties) = simulate_in_threads(attack_count, thread_count);

    report_results(wins, losses, ties);
}

/// Print usage message and exit with failure code.
fn print_help_and_exit(program_name: &str) -> ! {
    eprintln!("usage: {} [[ATTACKS] THREADS]", program_name);
    eprintln!("  Default attack count is {}.", DEFAULT_ATTACK_COUNT);
    eprintln!("  Default thread count is {}.", DEFAULT_THREAD_COUNT);
    process::exit(-1)
}

/// Get the specified argument value from the command-line arguments array.
///
/// If array is too short, return the specified default value.
///
/// If argument value cannot be parsed, print an error message and call
/// `print_help_and_exit()`.
fn arg_value<T>(arg_name: &str, args: &[String], arg_index: usize, default: T) -> T
where
    T: str::FromStr,
{
    if args.len() > arg_index {
        match args[arg_index].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!(
                    "error: invalid {} argument value \"{}\"",
                    arg_name, args[arg_index]
                );
                print_help_and_exit(&args[0]);
            }
        }
    } else {
        default
    }
}

/// Simulate the attacks by spawning threads and gathering results.
///
/// Returns a tuple `(wins, losses, ties)`.
fn simulate_in_threads(attack_count: i64, thread_count: i32) -> (i64, i64, i64) {
    // Spawn threads
    let mut threads = vec![];
    for _ in 0..thread_count {
        let count = attack_count;
        threads.push(thread::spawn(move || simulate_attacks(count)));
    }

    // Gather thread results
    threads.into_iter().map(|t| t.join().unwrap()).fold(
        (0, 0, 0),
        |(acc_wins, acc_losses, acc_ties), (wins, losses, ties)| {
            (acc_wins + wins, acc_losses + losses, acc_ties + ties)
        },
    )
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
