//! Règle de construction/résolution d'une grille.
//!
//! Recherche des combinaisons de 'n' régions occupent 'n' lignes ou 'n' colonnes.<br>
//! Dans ce cas, toutes les cases dans ces 'n' lignes ou colonnes qui n'appartiennent pas aux
//! régions ne peuvent pas être des étoiles.
//!
//! En effect, les 'n' régions sur 'n' lignes positionnent toutes les étoiles sur ces 'n' lignes
//! et il ne peut donc pas y avoir d'autres étoiles sur ce 'n' lignes.<br>
//! Idem pour les colonnes.
//!
//! //! Cette règle est l'opposée de la règle [`rule_region_exclusions`]

/// Crate qui recherche n combinaisons possibles dans un vecteur d'elements
use combination::combine;

use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;

/// Recherche les régions de 1 ligne ou 1 colonne. Les autres cases de cette ligne ou colonne
/// ne peuvent pas être des étoiles
pub fn rule_region_1_combinations(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_combinations(handler, grid, 1)
}

/// Recherche les couples de régions sur 2 ligne ou 2 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_2_combinations(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_combinations(handler, grid, 2)
}

/// Recherche les triplets de régions sur 3 ligne ou 3 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_3_combinations(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_combinations(handler, grid, 3)
}

/// Recherche les quadruplets de régions sur 4 ligne ou 4 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_4_combinations(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_combinations(handler, grid, 4)
}

/// Cherche les combinaisons de 'n' régions occupent exactement 'n' lignes ou 'n' colonnes.<br>
/// Si des cases appartement à d'autres régions sont dans ces lignes ou colonnes, elles ne peuvent
/// pas être des étoiles
fn rule_region_generic_combinations(
    handler: &GridHandler,
    grid: &Grid,
    n: usize,
) -> Option<GoodRule> {
    // On utilise le crate 'combination' pour trouver toutes les combinaisons possibles
    for vec_regions in combine::from_vec_at(&handler.regions(), n) {
        // On cherche les cases qui sont dans la combinaison et on détermine les lignes/colonnes minimales/maximales
        let all_cells = handler.surfer(grid, &GridSurfer::AllCells);
        let mut min_line = usize::MAX;
        let mut max_line = 0;
        let mut min_column = usize::MAX;
        let mut max_column = 0;
        for line_column in all_cells {
            let cell = grid.cell(line_column);
            if vec_regions.contains(&cell.region) {
                // Cette case de la grille est dans une des régions de la combinaison
                let (line, column) = (cell.line_column.line, cell.line_column.column);
                if line < min_line {
                    min_line = line;
                }
                if line > max_line {
                    max_line = line;
                }
                if column < min_column {
                    min_column = column;
                }
                if column > max_column {
                    max_column = column;
                }
            }
        }

        if (max_line - min_line + 1) == n {
            // Les 'n' régions occupent exactement 'n' lignes
            // Existe-t-il des cases dans ces lignes qui n'appartiennent pas à ces régions et qui sont indéfinies ?
            let grid_surfer = GridSurfer::Lines(min_line..=max_line);
            let surfer = handler.surfer(grid, &grid_surfer);
            let candidates: Vec<LineColumn> = surfer
                .iter()
                .filter(|line_column| grid.cell(**line_column).is_unknown())
                .filter(|line_column| !vec_regions.contains(&grid.cell(**line_column).region))
                .copied()
                .collect();

            if !candidates.is_empty() {
                let mut actions = Vec::new();
                for line_column in candidates {
                    actions.push(GridAction::SetNoStar(line_column));
                }

                return Some(GoodRule::ZoneCombinations(
                    vec_regions,
                    grid_surfer,
                    actions,
                ));
            }
        }

        if (max_column - min_column + 1) == n {
            // Les 'n' regions occupent exactement 'n'
            // Existe-t-il des cases dans ces colonnes qui n'appartiennent pas à ces régions et qui sont indéfinies ?
            let grid_surfer = GridSurfer::Columns(min_column..=max_column);
            let surfer = handler.surfer(grid, &grid_surfer);
            let candidates: Vec<LineColumn> = surfer
                .iter()
                .filter(|line_column| grid.cell(**line_column).is_unknown())
                .filter(|line_column| !vec_regions.contains(&grid.cell(**line_column).region))
                .copied()
                .collect();

            if !candidates.is_empty() {
                let mut actions = Vec::new();
                for line_column in candidates {
                    actions.push(GridAction::SetNoStar(line_column));
                }

                return Some(GoodRule::ZoneCombinations(
                    vec_regions,
                    grid_surfer,
                    actions,
                ));
            }
        }
    }

    None
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
    fn test_region_combinations() {
        let (grid_handler, mut grid) = get_test_grid();

        // Au moins la région 'A' ou 'C' déclenche cette règle
        let option_good_rule = rule_region_1_combinations(&grid_handler, &grid);
        assert!(&option_good_rule.is_some());
        let good_rule = option_good_rule.unwrap();
        grid.apply_good_rule(&good_rule);

        // println!("Rule: {}", &good_rule);
        // println!("Grid :\n{}", grid_handler.display(&grid, true));
        // panic!("stop test")
    }
}
