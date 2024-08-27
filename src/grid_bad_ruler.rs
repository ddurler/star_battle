//! Vérification de la validité d'une grille.
//!
//! Ce module déroule les règles de cohérence pour les cases d'un grille et signale les
//! éventuels problèmes détectés dans la construction d'une solution pour la grille.

use crate::CellValue;
use crate::Grid;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;

/// Erreur de cohérence de la grille
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum BadRuleError {
    /// Etoile adjacente à une autre étoile
    #[error("Etoile {0} adjacente à l'étoile {1}")]
    StarAdjacent(LineColumn, LineColumn),

    /// Trop d'étoiles dans une 'zone'
    #[error("Trop d'étoiles dans '{0}'")]
    TooManyStarsInZone(GridSurfer),

    /// Impossible de placer toutes les étoiles dans une 'zone'
    #[error("Impossible de placer toutes les étoiles dans '{0}'")]
    NotEnoughStarsInZone(GridSurfer),
}

/// Vérification de la validité d'une grille
///
/// ### Errors
/// Retourne un [`BadRuleError`] si la grille n'est pas valide
pub fn check_bad_rules(handler: &GridHandler, grid: &Grid) -> Result<(), BadRuleError> {
    check_no_star_adjacent(handler, grid)?;
    for region in handler.regions() {
        check_zone(handler, grid, &GridSurfer::Region(region))?;
    }
    for line in 0..handler.nb_lines() {
        check_zone(handler, grid, &GridSurfer::Line(line))?;
    }
    for column in 0..handler.nb_columns() {
        check_zone(handler, grid, &GridSurfer::Column(column))?;
    }
    Ok(())
}

/// Parcours les cases de la grille pour vérifier qu'aucune étoile n'est adjacent à une autre étoile
fn check_no_star_adjacent(handler: &GridHandler, grid: &Grid) -> Result<(), BadRuleError> {
    for line_column in handler.surfer(grid, &GridSurfer::AllCells) {
        let cell = grid.cell(line_column);
        if cell.value == CellValue::Star {
            for adjacent_line_column in handler.adjacent_cells(line_column) {
                let adjacent_cell = grid.cell(adjacent_line_column);
                if adjacent_cell.value == CellValue::Star {
                    return Err(BadRuleError::StarAdjacent(
                        line_column,
                        adjacent_line_column,
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Vérifie la validité du nombre d'étoile sur une zone (line, colonne ou région).<br>
fn check_zone(handler: &GridHandler, grid: &Grid, surfer: &GridSurfer) -> Result<(), BadRuleError> {
    let mut nb_stars = 0;
    let mut nb_possible_stars = 0;

    for line_column in handler.surfer(grid, surfer) {
        match grid.cell(line_column).value {
            CellValue::Star => nb_stars += 1,
            CellValue::Unknown => nb_possible_stars += 1,
            CellValue::NoStar => (),
        }
    }

    if nb_stars > handler.nb_stars() {
        return Err(BadRuleError::TooManyStarsInZone(surfer.clone()));
    } else if nb_stars + nb_possible_stars < handler.nb_stars() {
        return Err(BadRuleError::NotEnoughStarsInZone(surfer.clone()));
    }

    Ok(())
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
    fn test_no_star_adjacent() {
        let (grid_handler, mut grid) = get_test_grid();

        // On place volontairement 2 étoiles dans 2 cases adjacentes de la grille
        grid.cell_mut(LineColumn::new(0, 0)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(1, 1)).value = CellValue::Star;

        match check_bad_rules(&grid_handler, &grid) {
            Err(BadRuleError::StarAdjacent(_, _)) => (),
            _ => panic!("Échec détection de 2 étoiles adjacentes dans la grille"),
        }
    }

    #[test]
    fn test_too_many_stars_in_region() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement 2 étoiles non adjacentes dans la zone 'B' de la grille
        grid.cell_mut(LineColumn::new(0, 1)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(0, 4)).value = CellValue::Star;

        if let Err(BadRuleError::TooManyStarsInZone(GridSurfer::Region(region))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(
                region, 'B',
                "Échec détection trop d'étoiles dans la région 'B' (region '{region}' identifiée)"
            );
        } else {
            panic!("Échec détection trop d'étoiles dans une région");
        }
    }

    #[test]
    fn test_not_enough_stars_in_region() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement pas d'étoile dans les 2 case la zone 'A' de la grille
        grid.cell_mut(LineColumn::new(0, 0)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(1, 0)).value = CellValue::NoStar;

        if let Err(BadRuleError::NotEnoughStarsInZone(GridSurfer::Region(region))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(region, 'A',
                    "Échec détection impossible de placer une étoile dans la région 'A' (region '{region}' identifiée)");
        } else {
            panic!("Échec détection impossible de placer une étoile dans une région");
        }
    }

    #[test]
    fn test_too_many_stars_in_line() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement 2 étoiles non adjacentes dans 2eme ligne de la grille
        grid.cell_mut(LineColumn::new(1, 0)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(1, 4)).value = CellValue::Star;

        if let Err(BadRuleError::TooManyStarsInZone(GridSurfer::Line(line))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(
                line, 1,
                "Échec détection trop d'étoiles dans la ligne '1' (ligne '{line}' identifiée)"
            );
        } else {
            panic!("Échec détection trop d'étoiles dans une ligne");
        }
    }

    #[test]
    fn test_not_enough_stars_in_line() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement pas d'étoile dans les cases de la 2eme ligne de la grille
        for column in 0..grid_handler.nb_columns() {
            grid.cell_mut(LineColumn::new(1, column)).value = CellValue::NoStar;
        }

        if let Err(BadRuleError::NotEnoughStarsInZone(GridSurfer::Line(line))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(line, 1,
                    "Échec détection impossible de placer une étoile dans la ligne '1' (ligne '{line}' identifiée)");
        } else {
            panic!("Échec détection impossible de placer une étoile dans une ligne");
        }
    }

    #[test]
    fn test_too_many_stars_in_column() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement 2 étoiles non adjacentes dans 2eme colonne de la grille
        grid.cell_mut(LineColumn::new(0, 1)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(4, 1)).value = CellValue::Star;

        if let Err(BadRuleError::TooManyStarsInZone(GridSurfer::Column(column))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(
                column, 1,
                "Échec détection trop d'étoiles dans la colonne '1' (colonne '{column}' identifiée)"
            );
        } else {
            panic!("Échec détection trop d'étoiles dans une colonne");
        }
    }

    #[test]
    fn test_not_enough_stars_in_colonne() {
        let (grid_handler, mut grid) = get_test_grid();

        assert!(check_bad_rules(&grid_handler, &grid).is_ok());

        // On définit volontairement pas d'étoile dans les cases de la 2eme colonne de la grille
        for line in 0..grid_handler.nb_lines() {
            grid.cell_mut(LineColumn::new(line, 1)).value = CellValue::NoStar;
        }

        if let Err(BadRuleError::NotEnoughStarsInZone(GridSurfer::Column(column))) =
            check_bad_rules(&grid_handler, &grid)
        {
            assert_eq!(column, 1,
                    "Échec détection impossible de placer une étoile dans la colonne '1' (colonne '{column}' identifiée)");
        } else {
            panic!("Échec détection impossible de placer une étoile dans une colonne");
        }
    }
}
