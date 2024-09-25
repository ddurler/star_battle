//! Règles de construction/résolution d'une grille.
//!
//! Ce module expose les différentes règles permettant d'avancer dans la résolution d'une grille.

use std::fmt::Display;

use crate::check_bad_rules;
use crate::grid_action::display_vec_actions;
use crate::BadRuleError;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;
use crate::Region;

use super::rule_no_star_adjacent_to_star::rule_no_star_adjacent_to_star;
use super::rule_region_combinations::{
    rule_region_1_combinations, rule_region_2_combinations, rule_region_3_combinations,
    rule_region_4_combinations,
};
use super::rule_region_exclusions::{
    rule_region_1_exclusions, rule_region_2_exclusions, rule_region_3_exclusions,
    rule_region_4_exclusions,
};
use super::rule_region_possible_stars::rule_region_possible_stars;
use super::rule_value_completed::rule_value_completed;
use super::rule_zone_possible_stars::{
    rule_line_column_recursive_possible_stars, rule_multi_2_lines_columns_recursive_possible_stars,
    rule_multi_3_lines_columns_recursive_possible_stars,
    rule_multi_4_lines_columns_recursive_possible_stars, rule_region_recursive_possible_stars,
};

/// Énumération des règles applicables à la construction/résolution d'une grille
#[derive(Clone, Debug)]
pub enum GoodRule {
    /// Indique les cases adjacentes à une étoile qui ne peuvent pas contenir une étoile
    NoStarAdjacentToStar(LineColumn, Vec<GridAction>),

    /// Indique les cases restantes dans une zone ne peuvent pas être des étoiles
    ZoneNoStarCompleted(GridSurfer, Vec<GridAction>),

    /// Indique que les cases restantes des régions en dehors d'une combinaison de lignes ou colonnes
    /// ne peuvent pas contenir des étoiles
    ZoneExclusions(Vec<Region>, GridSurfer, Vec<GridAction>),

    /// Indique que les cases restantes en dehors d'une combinaison de régions/lignes ou colonnes
    /// ne peuvent pas contenir des étoiles
    ZoneCombinations(Vec<Region>, GridSurfer, Vec<GridAction>),

    /// Indique les cases restantes dans une zone sont forcement des étoiles
    ZoneStarCompleted(GridSurfer, Vec<GridAction>),

    /// Indique que quelle que soit la façon de placer les étoiles dans une zone, des cases n'ont
    /// toujours qu'une seule et même possibilité
    InvariantWithZone(GridSurfer, Vec<GridAction>),
}

impl Display for GoodRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Texte pour une ligne de régions
        fn display_vec_regions(regions: &[Region]) -> String {
            let mut str_regions = String::new();
            for region in regions {
                if !str_regions.is_empty() {
                    str_regions.push('+');
                }
                str_regions.push(*region);
            }
            str_regions
        }

        match self {
            Self::NoStarAdjacentToStar(line_column, actions) => {
                write!(f, "Les cases adjacentes à l'étoile en {line_column} ne peuvent pas contenir une étoile : {}", display_vec_actions(actions))
            }
            Self::ZoneNoStarCompleted(grid_surfer, actions) => {
                write!(
                    f,
                    "Les cases restantes pour {grid_surfer} ne peuvent pas contenir une étoile : {}",
                    display_vec_actions(actions)
                )
            }
            Self::ZoneExclusions(regions, grid_surfer, actions) => {
                let str_regions = display_vec_regions(regions);
                write!(
                    f,
                    "Les cases restantes des regions {str_regions} qui ne sont pas dans {grid_surfer} ne peuvent être une étoile : {}",
                    display_vec_actions(actions)
                )
            }
            Self::ZoneCombinations(regions, grid_surfer, actions) => {
                let str_regions = display_vec_regions(regions);
                write!(
                    f,
                    "Les cases restantes sur {grid_surfer} qui ne sont pas dans les régions {str_regions} ne peuvent être une étoile : {}",
                    display_vec_actions(actions)
                )
            }
            Self::ZoneStarCompleted(grid_surfer, actions) => {
                write!(
                    f,
                    "Les cases restantes pour {grid_surfer} peuvent être qu'une étoile : {}",
                    display_vec_actions(actions)
                )
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
            | GoodRule::ZoneNoStarCompleted(_, actions)
            | GoodRule::ZoneExclusions(_, _, actions)
            | GoodRule::ZoneCombinations(_, _, actions)
            | GoodRule::ZoneStarCompleted(_, actions)
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
    // Grille viable ?
    check_bad_rules(handler, grid)?;

    // Grille terminée ?
    if handler.is_done(grid) {
        return Ok(None);
    }

    for f in [
        rule_no_star_adjacent_to_star,
        rule_value_completed,
        rule_region_1_exclusions,
        rule_region_1_combinations,
        rule_region_possible_stars,
        rule_region_2_exclusions,
        rule_region_2_combinations,
        rule_region_recursive_possible_stars,
        rule_region_3_exclusions,
        rule_region_3_combinations,
        rule_line_column_recursive_possible_stars,
        rule_region_4_exclusions,
        rule_region_4_combinations,
        rule_multi_2_lines_columns_recursive_possible_stars,
        rule_multi_3_lines_columns_recursive_possible_stars,
        rule_multi_4_lines_columns_recursive_possible_stars,
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

    // Liste des grilles d'exemple
    const TEST_GRIDS_FILENAME_AND_NB_STARS: &[(&str, usize)] = &[
        ("./test_grids/test01.txt", 1),
        ("./test_grids/facile01_2.txt", 2),
        ("./test_grids/moyen01_2.txt", 2),
        ("./test_grids/difficile01_2.txt", 2),
        ("./test_grids/expert01_2.txt", 2),
        ("./test_grids/facile02_2.txt", 2),
        ("./test_grids/moyen02_2.txt", 2),
        ("./test_grids/difficile02_2.txt", 2),
        ("./test_grids/expert02_2.txt", 2),
        ("./test_grids/facile03_2.txt", 2),
        ("./test_grids/moyen03_2.txt", 2),
        ("./test_grids/difficile03_2.txt", 2),
        ("./test_grids/expert03_2.txt", 2),
    ];

    // #[test]
    // fn test_grid_dd_debug() {
    //     test_all_test_grids("facile03");
    // }

    #[test]
    fn test_grid_test() {
        test_all_test_grids("test");
    }

    #[test]
    fn test_grid_facile() {
        test_all_test_grids("facile");
    }

    #[test]
    fn test_grid_moyen() {
        test_all_test_grids("moyen");
    }

    #[test]
    fn test_grid_difficile() {
        test_all_test_grids("difficile");
    }

    #[test]
    fn test_grid_expert() {
        test_all_test_grids("expert");
    }

    /// Primitive générique qui teste les grilles de tests dont le nom de leur fichier contient
    /// la chaîne `filename_part`
    /// (Evite de tout tester silencieusement car c'est un peu long...)
    fn test_all_test_grids(filename_part: &str) {
        for (grid_file_name, nb_stars) in TEST_GRIDS_FILENAME_AND_NB_STARS {
            if grid_file_name.contains(filename_part) {
                // Ouverture du fichier
                println!("Fichier : {grid_file_name}");
                let mut file = File::open(grid_file_name).unwrap();
                // Lecture du fichier
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents).unwrap();
                // Conversion en Grid
                let grid_parser = GridParser::try_from(file_contents.as_str()).unwrap();
                let grid_handler = GridHandler::new(&grid_parser, *nb_stars);
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

                println!(
                    "\nFILE {grid_file_name}\n{}",
                    grid_handler.display(&grid, true)
                );
                assert!(grid_handler.is_done(&grid));
            }
        }
    }
}
