//! Gestion des règles de construction/résolution d'une grille

mod good_rule;
mod rule_complete_start_number;
mod rule_no_star_adjacent_to_star;

use rule_complete_start_number::rule_complete_star_number;
use rule_no_star_adjacent_to_star::rule_no_star_adjacent_to_star;

pub use good_rule::{get_good_rule, GoodRule};
