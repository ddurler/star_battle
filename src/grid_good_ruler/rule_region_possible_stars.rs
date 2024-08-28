//! Règle de construction/résolution d'une grille.
//!
//! Recherche les combinaisons d'étoiles possibles dans une région.
//! Plus simplement que `rule_region_star_complete`, on n'examine ici que le contenu des différentes
//! combinaisons dans une région sans examiner l'impact sur l'ensemble de la grille grille.
//! On intègre également dans cette recherche, toutes les cases environnant une région qui sont
//! forcément pas des étoles puisque toujours à proximité d'une étoile dans la région.
//! Les règles qui apparaissent ainsi sont plus compréhensible pour un humain.

use crate::GoodRule;
use crate::Grid;
use crate::GridHandler;

use super::rule_generic_possible_stars;
use super::ZoneToExamine;

/// Cherche toutes les combinaisons d'étoiles possibles dans les différentes régions.
/// Version simplifiée de `rule_region_recursive_possible_stars` qui se limite au contenu des
/// différentes régions pour une compréhension plus aisées pour un humain
pub fn rule_region_possible_stars(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_generic_possible_stars(handler, grid, ZoneToExamine::Region, false)
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
    fn test_region_stars() {
        let (grid_handler, mut grid) = get_test_grid();

        println!("Grille initiale :\n{}", grid_handler.display(&grid, true));

        // Cette règle s'applique sur la région 'CC' dans la 3eme ligne : Les cases adjacentes ne peuvent
        // pas être une étoile...
        let option_good_rule = rule_region_possible_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());

        // Cette règle s'applique sur l'avant dernière ligne de 'DDDDD' : On doit mettre une étoile
        // sur cette ligne donc les D sur la ligne suivante ne peuvent pas être une étoile...
        let option_good_rule = rule_region_possible_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());
    }
}
