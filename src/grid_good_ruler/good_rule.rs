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

use super::rule_no_star_adjacent_to_star;
use super::{rule_region_star_complete, rule_zone_star_complete};

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
                    "Toutes les possibilités pour {surfer} impliquent la seule possibilité : {}",
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

    for f in [
        rule_no_star_adjacent_to_star,
        rule_region_star_complete,
        rule_zone_star_complete,
    ] {
        if let Some(rule) = f(handler, grid) {
            return Ok(Some(rule));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;

    use crate::GridParser;

    #[test]
    fn test_grids() {
        // Liste des grilles d'exemple
        let grid_filenames_and_nb_stars = vec![
            ("./test_grids/test01.txt", 1),
            ("./test_grids/facile01_2.txt", 2),
            ("./test_grids/moyen01_2.txt", 2),
        ];

        for (grid_file_name, nb_stars) in grid_filenames_and_nb_stars {
            // Ouverture du fichier
            println!("Fichier : {grid_file_name}");
            let mut file = File::open(grid_file_name).unwrap();
            // Lecture du fichier
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents).unwrap();
            // Conversion en Grid
            let grid_parser = GridParser::try_from(file_contents.as_str()).unwrap();
            let grid_handler = GridHandler::new(&grid_parser, nb_stars);
            let mut grid = Grid::from(&grid_handler);
            // Boucle de résolution
            loop {
                match get_good_rule(&grid_handler, &grid) {
                    Ok(option_good_rule) => {
                        if option_good_rule.is_some() {
                            let good_rule = option_good_rule.unwrap();
                            grid.apply_good_rule(&good_rule);
                        } else {
                            break;
                        }
                    }
                    Err(bad_rule) => {
                        panic!("{bad_rule} !!!");
                    }
                }
            }
            assert!(grid_handler.is_done(&grid));
        }
    }
}
