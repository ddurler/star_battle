//! Règle de construction/résolution d'une grille.
//!
//! Recherche les cases adjacentes à une étoile qui ne peuvent pas contenir une étoile.

use crate::CellValue;
use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

/// Cherche dans les régions, les lignes et les colonnes s'il y a des contenus de cases 'évidents :
/// * Pas d'étoile si toutes les étoiles sont déjà placées dans la zone
/// * Une étoile si une seule possibilité pour la zone
pub fn rule_value_completed(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    let mut zones = Vec::new();

    // Parcours de toutes les régions
    for region in handler.regions() {
        zones.push(GridSurfer::Region(region));
    }

    // Parcours de toutes les lignes
    for line in 0..handler.nb_lines() {
        zones.push(GridSurfer::Line(line));
    }

    // Parcours de toutes les colonnes
    for column in 0..handler.nb_columns() {
        zones.push(GridSurfer::Column(column));
    }

    // Examine toutes les zones prévues
    for zone in zones {
        if let Some(good_rule) = try_value_completed(handler, grid, &zone, handler.nb_stars()) {
            return Some(good_rule);
        }
    }
    None
}

/// Détermine s'il y a des contenus de cases 'évidents' pour une zone
fn try_value_completed(
    handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
) -> Option<GoodRule> {
    let surfer = handler.surfer(grid, grid_surfer);

    // Décompte de cases inconnue/avec étoile/sans étoile dans la zone
    let mut cur_nb_stars = 0;
    let mut _cur_nb_no_stars = 0;
    // Nombre et cases restantes à placer dans la zone
    let mut cur_nb_unknown = 0;
    let mut line_column_unknown = Vec::new();
    // On pourrait compter les types de valeurs avec `handler.surfer_cells_with_value_count` mais
    // nécessiterait de créer à chaque fois un nouveau surfer (coûteux...)
    for line_column in surfer {
        match grid.cell(line_column).value {
            CellValue::Star => cur_nb_stars += 1,
            CellValue::NoStar => _cur_nb_no_stars += 1,
            CellValue::Unknown => {
                cur_nb_unknown += 1;
                line_column_unknown.push(line_column);
            }
        }
    }

    if cur_nb_unknown > 0 {
        // Il reste des cases non définies dans la zone...
        if cur_nb_stars == nb_stars {
            // ...et la zone possède déjà toutes les étoiles attendues
            // => les cases inconnues sont forcement sans étoile
            let mut actions = Vec::new();
            for line_column in line_column_unknown {
                actions.push(GridAction::SetNoStar(line_column));
            }
            return Some(GoodRule::ZoneNoStarCompleted(*grid_surfer, actions));
        }
        if cur_nb_unknown == nb_stars - cur_nb_stars {
            // ... et il reste dans la zone autant de cases indéfinies qu'il reste d'étoiles à placer
            // => les cases inconnues sont forcement avec une étoile
            let mut actions = Vec::new();
            for line_column in line_column_unknown {
                actions.push(GridAction::SetStar(line_column));
            }
            return Some(GoodRule::ZoneStarCompleted(*grid_surfer, actions));
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
    fn test_zone_no_star_completed() {
        let (grid_handler, grid) = get_test_grid();

        for line in 0..grid_handler.nb_lines() {
            for column in 0..grid_handler.nb_columns() {
                let mut test_grid = grid.clone();

                // On place volontairement 1 étoile dans la grille
                test_grid.apply_action(&GridAction::SetStar(LineColumn::new(line, column)));

                // La règle doit détecter une région qui doit être complétée avec des cases sans étoile
                let good_rule = rule_value_completed(&grid_handler, &test_grid);
                match good_rule {
                    Some(GoodRule::ZoneNoStarCompleted(_, _)) => (),
                    _ => panic!("La règle n'est pas détectée"),
                }
            }
        }
    }

    #[test]
    fn test_zone_star_completed() {
        let (grid_handler, mut grid) = get_test_grid();

        // On place volontairement 1 case sans étoile dans la zone A de 2 cases
        grid.apply_action(&GridAction::SetNoStar(LineColumn::new(1, 0)));

        // La règle doit détecter une région qui doit être complétée avec des cases avec étoile
        let good_rule = rule_value_completed(&grid_handler, &grid);
        match good_rule {
            Some(GoodRule::ZoneStarCompleted(_, _)) => (),
            _ => panic!("La règle n'est pas détectée"),
        }
    }
}
