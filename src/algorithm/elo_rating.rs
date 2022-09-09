//! # ELO rating
//!
//! This module contains all of the standard methods that would be used to calculate elo.
//! The module provides the constants WIN, LOSE and DRAW.
#![allow(dead_code)]

use crate::algorithm::k_factor::{fide_k, icc_k, uscf_k};

/// The EloScore type is i32 since a rating can be negative.
pub type EloScore = i64;

pub type EloCompeteResult = f64;

/// The score for a won game
pub const WIN: EloCompeteResult = 1_f64;
/// The score for a drawn game
pub const DRAW: EloCompeteResult = 0.5;
/// The score for a lost game
pub const LOSS: EloCompeteResult = 0_f64;

fn rating_change(k: u64, score: EloCompeteResult, exp_score: EloCompeteResult) -> EloScore {
    (k as f64 * (score - exp_score)) as i64
}

/// Calculates the expected outcome of a match between two players.
/// This will always be a number between 0 and 1.
/// The closer to 1 the more favored the match is for player a.
pub fn expected_score(r_a: EloScore, r_b: EloScore) -> f64 {
    1_f64 / (1_f64 + 10_f64.powf(((r_b - r_a) as f64) / 400_f64))
}

/// Convenience function for a game played with FIDE k_factor.
pub fn compete_fide(
    r_a: EloScore,
    game_count_a: u64,
    r_b: EloScore,
    game_count_b: u64,
    s_a: EloCompeteResult,
) -> (EloScore, EloScore) {
    let k_a = fide_k(r_a, game_count_a);
    let k_b = fide_k(r_b, game_count_b);

    compete(r_a, r_b, s_a, k_a, k_b)
}

/// Convenience function for a game played with USCF k_factor.
pub fn compete_uscf(r_a: EloScore, r_b: EloScore, s_a: EloCompeteResult) -> (EloScore, EloScore) {
    let k_a = uscf_k(r_a);
    let k_b = uscf_k(r_b);

    compete(r_a, r_b, s_a, k_a, k_b)
}

/// Convenience function for a game played with ICC k_factor.
pub fn compete_icc(r_a: EloScore, r_b: EloScore, s_a: EloCompeteResult) -> (EloScore, EloScore) {
    let k_a = icc_k();
    let k_b = icc_k();

    compete(r_a, r_b, s_a, k_a, k_b)
}

/// Calculates the updated elo ratings of both players after a match.
/// The k_a and k_b are the K factors used to determine the updated rating,
/// If you just want a default behavior set these to 32, or use game_icc() instead.
pub fn compete(
    r_a: EloScore,
    r_b: EloScore,
    s_a: EloCompeteResult,
    k_a: u64,
    k_b: u64,
) -> (EloScore, EloScore) {
    let s_b = 1_f64 - s_a;

    let e_a = expected_score(r_a, r_b);
    let e_b = 1_f64 - e_a;

    let new_a = r_a + rating_change(k_a, s_a, e_a);
    let new_b = r_b + rating_change(k_b, s_b, e_b);

    (new_a, new_b)
}

/// Calculates the updated elo of a player, after a series of games.
/// This might be used to calculate the rating of a player after a tournament.
pub fn serial_compete(
    r_a: EloScore,
    games: &[(EloScore, EloCompeteResult)],
    k_factor: u64,
) -> EloScore {
    let mut score = 0_f64;
    let mut exp_score = 0_f64;

    for game in games {
        score += game.1;
        exp_score = expected_score(r_a, game.0);
    }

    r_a + rating_change(k_factor, score, exp_score)
}

#[cfg(test)]
mod tests {
    use crate::algorithm::k_factor::*;

    use super::*;

    #[test]
    fn test_expected_score() {
        let john = 1700;
        let paul = 1800;

        // calculate johns chance to win against paul
        let chance = expected_score(john, paul);
        assert!(chance >= 0.0 && chance <= 1.0);
        println!("johns chance to win against paul: {}", chance)
    }

    #[test]
    fn test_compete() {
        let john = 1700;
        let paul = 1800;

        println!("before compete: john: {}, paul: {}", john, paul);
        let (john, paul) = compete(john, paul, LOSS, 32, 32);
        println!("after compete(paul win): john: {}, paul: {}", john, paul);
    }

    #[test]
    fn test_serial_compete() {
        let john = 1700;
        println!("before serial competes: john: {}", john);

        // An array containing the results of johns games in the tournament
        let games = [(1600, WIN), (1800, DRAW), (2000, LOSS)];

        let john = serial_compete(john, &games, 32);
        println!("after serial competes: john: {}", john);
    }

    #[test]
    fn test_compete_uscf() {
        let john = 1400;
        let paul = 1800;

        println!("before compete uscf: john: {}, paul: {}", john, paul);
        let (john, paul) = compete_uscf(john, paul, WIN);
        println!(
            "after compete uscf(paul win): john: {}, paul: {}",
            john, paul
        );
    }
}
