extern crate rand;

use rand::distributions::Uniform;
use rand::prelude::*;

use std::thread;

/// A six-sided die
pub struct Die {
    dist: Uniform<i32>,
    rng: ThreadRng,
}

impl Die {
    /// Roll the die, providing a result between 1 and 6
    pub fn roll(&mut self) -> i32 {
        self.dist.sample(&mut self.rng)
    }
}

impl Default for Die {
    fn default() -> Die {
        Die {
            dist: Uniform::from(1..=6),
            rng: rand::thread_rng(),
        }
    }
}

/// Possible outcomes of an attack
#[derive(Debug, PartialEq)]
pub enum Score {
    /// Attacker destroys two defenders
    Win,
    /// Defender destroys two attackers
    Loss,
    /// Attacker and defender each lose one
    Tie,
}

/// Set of die rolls for an attack.
///
/// a1, a2, and a3 are the attacker's die rolls.
/// d1 and d2 are the defender's die rolls.
#[derive(Debug)]
pub struct Attack {
    a1: i32,
    a2: i32,
    a3: i32,

    d1: i32,
    d2: i32,
}

impl Attack {
    /// Construct a Roll by rolling a die five times.
    pub fn with_die(die: &mut Die) -> Attack {
        Attack {
            a1: die.roll(),
            a2: die.roll(),
            a3: die.roll(),

            d1: die.roll(),
            d2: die.roll(),
        }
    }

    /// Construct a Roll by providing five values.
    ///
    /// This method is used for tests of the scoring function.
    pub fn with_die_rolls(a1: i32, a2: i32, a3: i32, d1: i32, d2: i32) -> Attack {
        Attack { a1, a2, a3, d1, d2 }
    }

    /// Get the two largest attacker dice.
    ///
    /// The first element of the result is the largest
    /// die value, and the second element is the
    /// next largest.
    pub fn attacker_largest(&self) -> (i32, i32) {
        let a1 = self.a1;
        let a2 = self.a2;
        let a3 = self.a3;
        if a1 > a2 {
            if a3 > a1 {
                (a3, a1)
            } else if a3 > a2 {
                (a1, a3)
            } else {
                (a1, a2)
            }
        } else if a3 > a2 {
            (a3, a2)
        } else if a3 > a1 {
            (a2, a3)
        } else {
            (a2, a1)
        }
    }

    /// Get the defender's die rolls in (largest, smallest) order.
    pub fn defender_largest(&self) -> (i32, i32) {
        let d1 = self.d1;
        let d2 = self.d2;
        if d1 > d2 {
            (d1, d2)
        } else {
            (d2, d1)
        }
    }

    /// Determine the score for a roll, from the attacker's point-of-view.
    ///
    /// Returns `Win` if attacker destroys two defenders, `Loss` if the defender
    /// destroys two attackers, or `Tie` if each side destroys one of the other
    /// side.
    pub fn attacker_score(&self) -> Score {
        let mut count = 0;

        let (a_largest, a_next) = self.attacker_largest();
        let (d_largest, d_next) = self.defender_largest();

        // Note: defender wins ties

        if a_largest > d_largest {
            count += 1;
        } else {
            count -= 1;
        }

        if a_next > d_next {
            count += 1;
        } else {
            count -= 1;
        }

        if count > 0 {
            Score::Win
        } else if count < 0 {
            Score::Loss
        } else {
            Score::Tie
        }
    }
}

/// Simulate the attacks by spawning threads and gathering results.
///
/// The total number of simulated attacks is `attack_count * thread_count`.
///
/// Returns a tuple `(wins, losses, ties)`.
pub fn simulate_in_threads(attack_count: i64, thread_count: i32) -> (i64, i64, i64) {
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
pub fn simulate_attacks(count: i64) -> (i64, i64, i64) {
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

/// Given numerator and denominator, calculate percentage.
pub fn percentage(numerator: i64, denominator: i64) -> f64 {
    100.0 * numerator as f64 / denominator as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn die_roll() {
        let mut die = Die::default();

        for _ in 1..1000 {
            let roll = die.roll();
            assert!(roll >= 1);
            assert!(roll <= 6);
        }
    }

    #[test]
    fn attack_score() {
        let roll = Attack::with_die_rolls(1, 2, 3, 4, 5);
        assert_eq!(roll.attacker_largest(), (3, 2));
        assert_eq!(roll.defender_largest(), (5, 4));
        assert_eq!(roll.attacker_score(), Score::Loss);

        let roll = Attack::with_die_rolls(5, 4, 3, 2, 1);
        assert_eq!(roll.attacker_largest(), (5, 4));
        assert_eq!(roll.defender_largest(), (2, 1));
        assert_eq!(roll.attacker_score(), Score::Win);

        let roll = Attack::with_die_rolls(4, 5, 3, 3, 6);
        assert_eq!(roll.attacker_largest(), (5, 4));
        assert_eq!(roll.defender_largest(), (6, 3));
        assert_eq!(roll.attacker_score(), Score::Tie);
    }

    #[test]
    fn test_simulate_threads() {
        let attacks = 1000;
        let threads = 4;

        let (wins, losses, ties) = simulate_in_threads(attacks, threads);

        let expected_total = attacks * i64::from(threads);
        assert_eq!(expected_total, wins + losses + ties);

        // The following assertions aren't guaranteed to be true, but are
        // statistically likely.  If these fail, increasing the number of
        // attacks or threads should increase the likelihoods.
        //
        // The expected results are approximately:
        //
        // - wins: 37%
        // - losses: 29%
        // - ties: 34%

        assert!(wins > 0);
        assert!(losses > 0);
        assert!(ties > 0);

        assert!(wins > ties);
        assert!(ties > losses);
    }
}
