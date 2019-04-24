[![Build Status](https://travis-ci.org/kristopherjohnson/risk_odds.svg?branch=master)](https://travis-ci.org/kristopherjohnson/risk_odds)

risk_odds
=========

This is a simple program that calculates the odds of the attacker winning in a
3-vs.-2 attack in the game Risk, using
[Monte Carlo simulation](https://en.wikipedia.org/wiki/Monte_Carlo_method).

(This is a Rust version of the very first program I wrote in BASIC, back around
1979).

To run the Monte Carlo simulation, do this:

    cargo run --release --bin risk_odds_monte_carlo

A faster and more-accurate way to find the odds is to simply try every possible
combination of five dice.  To run that program, do this:

    cargo run --release --bin risk_odds_simple

For more about this topic, see [RISK® Analysis](http://www.datagenetics.com/blog/november22011/index.html).
