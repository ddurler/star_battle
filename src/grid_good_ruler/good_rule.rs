//! Règles de construction/résolution d'une grille.
//!
//! Ce module expose les règles permettant d'avancer dans la résolution d'une grille.

use std::fmt::Display;

use crate::check_bad_rules;
use crate::grid_action::display_vec_actions;
use crate::BadRuleError;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;

use super::rule_complete_star_number;
use super::rule_no_star_adjacent_to_star;

/// Énumération des règles applicables à la construction/résolution d'une grille
#[derive(Clone, Debug)]
pub enum GoodRule {
    /// Indique les cases adjacentes à une étoile qui ne peuvent pas contenir une étoile
    NoStarAdjacentToStar(LineColumn, Vec<GridAction>),

    /// Indique que quelle que soit la façon de placer les étoiles dans une zone, des cases n'ont
    /// toujours qu'une seule et même possibilité
    InvariantWithZone(GridSurfer, Vec<GridAction>),
}

impl Display for GoodRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStarAdjacentToStar(line_column, actions) => {
                write!(f, "Les cases adjacentes à l'étoile en {line_column} ne peuvent pas contenir une étoile : {}", display_vec_actions(actions))
            }
            Self::InvariantWithZone(surfer, actions) => {
                write!(
                    f,
                    "Toutes les possibilités pour La zone {surfer} impliquent la seule possibilité : {}",
                    display_vec_actions(actions)
                )
            }
        }
    }
}

impl Grid {
    /// Application d'une règle de construction sur une grille
    pub fn apply_good_rule(&mut self, rule: &GoodRule) {
        match rule {
            GoodRule::NoStarAdjacentToStar(_, actions)
            | GoodRule::InvariantWithZone(_, actions) => {
                for action in actions {
                    self.apply_action(action);
                }
            }
        }
    }
}

/// Identification d'une règle de construction applicable à la grille.<br>
/// Retourne une règle applicable à la construction/résolution de la grille si trouvé. None sinon.
/// ### Errors
/// Retourne un [`BadRuleError`] si la grille n'est pas valide
#[allow(clippy::module_name_repetitions)]
pub fn get_good_rule(handler: &GridHandler, grid: &Grid) -> Result<Option<GoodRule>, BadRuleError> {
    check_bad_rules(handler, grid)?;

    for f in [rule_no_star_adjacent_to_star, rule_complete_star_number] {
        if let Some(rule) = f(handler, grid) {
            return Ok(Some(rule));
        }
    }

    Ok(None)
}
