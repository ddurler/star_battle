//! Règle de construction/résolution d'une grille.
//!
//! Recherche les cases invariantes pour toutes les combinaisons possibles d'une zone

use crate::GoodRule;
use crate::Grid;
use crate::GridHandler;

use super::rule_generic_possible_stars;
use super::ZoneToExamine;

/// Cherche toutes les combinaisons possibles dans les différentes régions.
/// Version simplifiée de `rule_zone_recursive_possible_stars` qui se limite au contenu des différentes
/// régions pour une compréhension plus aisées pour un humain
pub fn rule_region_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_generic_possible_stars(handler, grid, ZoneToExamine::Region, true)
}

/// Cherche toutes les combinaisons possibles qui positionnent le nombre attendu d'étoiles
/// dans les différentes ligne ou colonne.
/// Pour chaque zone, examine ensuite l'ensemble des grilles après avoir placer toutes les étoiles pour
/// déterminer si le contenu d'une case est commun à toutes ces combinaisons possibles.
pub fn rule_line_column_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_generic_possible_stars(handler, grid, ZoneToExamine::LineAndColumn, true)
}

/// Cherche toutes les combinaisons possibles qui positionnent le nombre attendu d'étoiles
/// dans différentes groupes de lignes consécutives ou groupes de colonnes consécutive.
/// Pour chaque zone, examine ensuite l'ensemble des grilles après avoir placer toutes les étoiles pour
/// déterminer si le contenu d'une case est commun à toutes ces combinaisons possibles.
pub fn rule_multi_lines_columns_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_generic_possible_stars(
        handler,
        grid,
        ZoneToExamine::MultipleLinesAndColumns(2),
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::GridParser;

    // Construction d'un objet GridHandler et d'un Grid à partir d'une grille de test
    fn get_test_grid() -> (GridHandler, Grid) {
        let grid_parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid_handler = GridHandler::new(&grid_parser, 1);
        let grid = Grid::from(&grid_handler);
        (grid_handler, grid)
    }

    #[test]
    fn test_complete_start_number() {
        // La grille de test peut être complètement résolue avec cette seule règle sur les zones
        let (grid_handler, mut grid) = get_test_grid();

        println!("Grille initiale :\n{}", grid_handler.display(&grid, true));

        loop {
            let option_good_rule = rule_line_column_recursive_possible_stars(&grid_handler, &grid);
            if option_good_rule.is_some() {
                let good_rule = option_good_rule.unwrap();
                println!("{good_rule}");
                grid.apply_good_rule(&good_rule);

                println!("\n{}", grid_handler.display(&grid, true));
            } else {
                break;
            }
        }

        assert!(grid_handler.is_done(&grid));
    }
}
