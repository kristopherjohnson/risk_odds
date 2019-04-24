extern crate risk_odds;

use risk_odds::{percentage, simulate_in_threads};

use std::env;
use std::process;
use std::str;

const DEFAULT_ATTACK_COUNT: i64 = 25_000_000;
const DEFAULT_THREAD_COUNT: i32 = 4;

/// Program entry point
///
/// Takes two optional command-line parameters:
///
/// - number of attack rolls to simulate per thread (default 25 million)
/// - number of threads (default 4)
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
fn arg_value<T>(name: &str, args: &[String], index: usize, default: T) -> T
where
    T: str::FromStr,
{
    if args.len() > index {
        args[index].parse().unwrap_or_else(|_| {
            eprintln!("error: invalid {} argument value \"{}\"", name, args[index]);
            print_help_and_exit(&args[0]);
        })
    } else {
        default
    }
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
