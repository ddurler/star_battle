//! Gestion des règles de construction/résolution d'une grille

mod good_rule;
mod invariant;
mod rule_no_star_adjacent_to_star;
mod rule_region_stars;
mod rule_star_complete;
mod rule_value_completed;

pub use good_rule::{get_good_rule, GoodRule};
