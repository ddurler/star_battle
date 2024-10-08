//! Règle de construction/résolution d'une grille.
//!
//! Recherche les cases adjacentes à une étoile qui ne peuvent pas contenir une étoile.

use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

/// Cherche si une étoile déjà placée à des cases adjacentes non définies.
/// Si oui, ces cases peuvent être définie comme `NoStar`
pub fn rule_no_star_adjacent_to_star(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    for line_column in handler.surfer(grid, &GridSurfer::AllCells) {
        if grid.cell(line_column).is_star() {
            let unknown_adjacent_cells: Vec<GridAction> = handler
                .adjacent_cells(line_column)
                .iter()
                .filter(|line_column| grid.cell(**line_column).is_unknown())
                .map(|line_column| GridAction::SetNoStar(*line_column))
                .collect();
            if !unknown_adjacent_cells.is_empty() {
                return Some(GoodRule::NoStarAdjacentToStar(
                    line_column,
                    unknown_adjacent_cells,
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
    use crate::LineColumn;

    // Construction d'un objet GridHandler et d'un Grid à partir d'une grille de test
    fn get_test_grid() -> (GridHandler, Grid) {
        let grid_parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid_handler = GridHandler::new(&grid_parser, 1);
        let grid = Grid::from(&grid_handler);
        (grid_handler, grid)
    }

    #[test]
    fn test_no_star_adjacent_to_star() {
        let (grid_handler, mut grid) = get_test_grid();

        // On place volontairement 1 étoile au centre de la grille
        let center_line_column = LineColumn::new(2, 2);
        grid.apply_action(&GridAction::SetStar(center_line_column));

        // Les 8 cases adjacentes ne peuvent pas contenir une étoile
        let good_rule = rule_no_star_adjacent_to_star(&grid_handler, &grid);
        match good_rule {
            Some(GoodRule::NoStarAdjacentToStar(line_column, actions)) => {
                assert_eq!(line_column, center_line_column);
                assert_eq!(actions.len(), 8);
                let adjacent_to_center_line_column =
                    grid_handler.surfer(&grid, &GridSurfer::Adjacent(center_line_column));
                for action in actions {
                    match action {
                        GridAction::SetNoStar(line_column) => {
                            assert!(adjacent_to_center_line_column.contains(&line_column));
                        }
                        _ => panic!("L'action n'est pas détectée"),
                    }
                }
            }
            _ => panic!("La règle n'est pas détectée"),
        }
    }
}
