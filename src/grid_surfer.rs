//! [`Surfer`] permet de déterminer une façon de naviguer à travers la grille.<br>
//!
//! Applicable sur un objet [`GridHandler`] associé à une grille définie par un [`Grid`].

use std::fmt::Display;
use std::ops::RangeInclusive;

use crate::line_column::{display_column, display_line};
use crate::CellValue;
use crate::Grid;
use crate::GridCell;
use crate::GridHandler;
use crate::LineColumn;
use crate::Region;

/// Navigation dans la grille
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GridSurfer {
    /// Navigation sur toutes les case de la grille
    AllCells,

    /// Navigation sur toutes les cases d'une région
    Region(Region),

    /// Navigation sur toutes les cases adjacentes à une case donnée (y compris les diagonales)
    Adjacent(LineColumn),

    /// Navigation sur toutes les cases d'un ligne
    Line(usize),

    /// Navigation sur toutes les cases d'une colonne
    Column(usize),

    /// Navigation sur plusieurs lignes
    Lines(RangeInclusive<usize>),

    /// Navigation sur plusieurs colonnes
    Columns(RangeInclusive<usize>),
}

impl Display for GridSurfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllCells => write!(f, "Toute la grille"),
            Self::Region(region) => write!(f, "Region '{region}'"),
            Self::Adjacent(line_column) => write!(f, "Cases adjacentes à '{line_column}'"),
            Self::Line(line) => write!(f, "Ligne {}", display_line(*line)),
            Self::Column(column) => write!(f, "Colonne {}", display_column(*column)),
            Self::Lines(range) => write!(
                f,
                "Lignes {}-{}",
                display_line(*range.start()),
                display_line(*range.end())
            ),
            Self::Columns(range) => write!(
                f,
                "Colonnes {}-{}",
                display_column(*range.start()),
                display_column(*range.end())
            ),
        }
    }
}

impl GridHandler {
    /// Retourne la liste des cases d'une grille qui satisfont à un certain critère.<br>
    /// Le critère est défini par l'énumération `GridSurfer`
    #[must_use]
    pub fn surfer(&self, grid: &Grid, surfer: &GridSurfer) -> Vec<LineColumn> {
        let mut cells = Vec::new();
        for line in 0..self.nb_lines() {
            for column in 0..self.nb_columns() {
                let line_column = LineColumn::new(line, column);
                let cell: &GridCell = grid.cell(line_column);
                let cell_is_matching = match surfer {
                    // Toutes les case de la grille
                    GridSurfer::AllCells => true,
                    // Toutes les cases d'une région
                    GridSurfer::Region(region) => cell.region == *region,
                    // Toutes les cases adjacentes à une case donnée (y compris les diagonales)
                    GridSurfer::Adjacent(line_column) => {
                        let adjacent_cells = self.adjacent_cells(*line_column);
                        adjacent_cells
                            .iter()
                            .any(|cell| cell.line == line && cell.column == column)
                    }
                    // Toutes les cases d'une ligne
                    GridSurfer::Line(select_line) => *select_line == line,
                    // Toutes les cases d'une colonne
                    GridSurfer::Column(select_column) => *select_column == column,
                    // Toutes les cases de plusieurs lignes
                    GridSurfer::Lines(line_range) => line_range.contains(&line),
                    // Toutes les cases de plusieurs colonnes
                    GridSurfer::Columns(column_range) => column_range.contains(&column),
                };
                if cell_is_matching {
                    cells.push(line_column);
                }
            }
        }

        cells
    }

    /// Retourne le nombre de cases sans la zone définie par le `GridSurfer`
    #[must_use]
    pub fn surfer_cells_count(&self, grid: &Grid, surfer: &GridSurfer) -> usize {
        self.surfer(grid, surfer).len()
    }

    /// Retourne le nombre de cases contenant une valeur particulière dans la zone définie par le `GridSurfer`
    #[must_use]
    pub fn surfer_cells_with_value_count(
        &self,
        grid: &Grid,
        surfer: &GridSurfer,
        value: &CellValue,
    ) -> usize {
        self.surfer(grid, surfer)
            .iter()
            .filter(|line_column| grid.cell(**line_column).value == *value)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::GridParser;

    // Construction d'un objet GridHandler et d'un Grid à partir d'une grille de test
    fn get_test_grid() -> (GridHandler, Grid) {
        let parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid_handler = GridHandler::new(&parser, 1);
        let grid = Grid::from(&grid_handler);
        (grid_handler, grid)
    }

    #[test]
    fn test_all_cells() {
        let (grid_handler, grid) = get_test_grid();
        let surfer = grid_handler.surfer(&grid, &GridSurfer::AllCells);
        assert_eq!(
            surfer.len(),
            grid_handler.nb_lines() * grid_handler.nb_columns()
        );
    }

    #[test]
    fn test_region() {
        let (grid_handler, grid) = get_test_grid();
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Region('A'));
        assert_eq!(surfer, vec![LineColumn::new(0, 0), LineColumn::new(1, 0)]);
    }

    #[test]
    fn test_adjacent() {
        let (grid_handler, grid) = get_test_grid();
        // 8 cases adjacentes à la case (2, 2) au milieu de la grille
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Adjacent(LineColumn::new(2, 2)));
        assert_eq!(surfer.len(), 8);
    }

    #[test]
    fn test_line() {
        let (grid_handler, grid) = get_test_grid();
        // 5 cases de la 2eme ligne
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Line(1));
        assert_eq!(surfer.len(), 5);
        assert_eq!(
            surfer
                .iter()
                .filter(|line_column| line_column.line == 1)
                .count(),
            5
        );
    }

    #[test]
    fn test_column() {
        let (grid_handler, grid) = get_test_grid();
        // 5 cases de la 2eme colonne
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Column(1));
        assert_eq!(surfer.len(), 5);
        assert_eq!(
            surfer
                .iter()
                .filter(|line_column| line_column.column == 1)
                .count(),
            5
        );
    }

    #[test]
    fn test_multi_lines() {
        let (grid_handler, grid) = get_test_grid();
        // 15 cases de la 2eme, 3eme et 4eme lignes
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Lines(1..=3));
        assert_eq!(surfer.len(), 15);
        assert_eq!(
            surfer
                .iter()
                .filter(|line_column| (1..=3).contains(&line_column.line))
                .count(),
            15
        );
    }

    #[test]
    fn test_multi_columns() {
        let (grid_handler, grid) = get_test_grid();
        // 10 cases de la 4eme et dernière colonnes
        let surfer = grid_handler.surfer(&grid, &GridSurfer::Columns(3..=4));
        assert_eq!(surfer.len(), 10);
        assert_eq!(
            surfer
                .iter()
                .filter(|line_column| (3..=4).contains(&line_column.column))
                .count(),
            10
        );
    }

    #[test]
    fn test_surfer_cells_count() {
        let (grid_handler, grid) = get_test_grid();
        assert_eq!(
            grid_handler.surfer_cells_count(&grid, &GridSurfer::Region('A')),
            2
        );
    }

    #[test]
    fn test_surfer_cells_with_value_count() {
        let (grid_handler, mut grid) = get_test_grid();

        // Par défaut, toutes les cases sont à la valeur `CellValue::Unknown`
        // On place une étoile et une case qui ne peut pas contenir d'étoile sur la 1ere ligne
        grid.cell_mut(LineColumn::new(0, 1)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(0, 3)).value = CellValue::NoStar;

        assert_eq!(
            grid_handler.surfer_cells_with_value_count(
                &grid,
                &GridSurfer::Line(0),
                &CellValue::Star
            ),
            1
        );
        assert_eq!(
            grid_handler.surfer_cells_with_value_count(
                &grid,
                &GridSurfer::Line(0),
                &CellValue::NoStar
            ),
            1
        );
        assert_eq!(
            grid_handler.surfer_cells_with_value_count(
                &grid,
                &GridSurfer::Line(0),
                &CellValue::Unknown
            ),
            3
        );
    }
}
