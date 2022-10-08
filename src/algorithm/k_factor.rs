//! Convenience functions for various popular rating systems using elo.

use crate::algorithm::elo_rating::EloScore;

/// FIDE calculates their k_factor depending on how many games you have played,
/// and what you elo rating is.
///
/// They also sometimes use age. But this is left out.
pub fn fide_k(rating: EloScore, game_counts: u64) -> u64 {
    if game_counts < 30 {
        40
    } else if rating < 2400 {
        20
    } else {
        10
    }
}

/// The USCF uses three different k_factors depending on you rating.
pub fn uscf_k(rating: EloScore) -> u64 {
    if rating < 2100 {
        32
    } else if rating < 2400 {
        24
    } else {
        16
    }
}

/// The ICC uses a global k_factor.
pub fn icc_k() -> u64 {
    32
}
