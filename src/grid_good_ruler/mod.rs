//! Gestion des règles de construction/résolution d'une grille

mod collector;
mod good_rule;
mod invariant;
mod rule_generic_possible_stars;
mod rule_no_star_adjacent_to_star;
mod rule_region_possible_stars;
mod rule_value_completed;
mod rule_zone_possible_stars;
mod star_adjacent;

pub use good_rule::{get_good_rule, GoodRule};
use rule_generic_possible_stars::{rule_generic_possible_stars, ZoneToExamine};
