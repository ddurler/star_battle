//! Gestion des règles de construction/résolution d'une grille

mod good_rule;
mod rule_no_star_adjacent_to_star;
mod rule_star_complete;

use rule_no_star_adjacent_to_star::rule_no_star_adjacent_to_star;
use rule_star_complete::{rule_region_star_complete, rule_zone_star_complete};

pub use good_rule::{get_good_rule, GoodRule};
