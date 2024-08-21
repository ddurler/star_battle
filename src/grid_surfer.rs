//! [`Surfer`] permet de déterminer une façon de naviguer à travers la grille.<br>
//!
//! Applicable sur un objet [`GridHandler`] associé à une grille définie par un [`Grid`].

use std::fmt::Display;

use crate::Grid;
use crate::GridCell;
use crate::GridHandler;
use crate::LineColumn;
use crate::Region;

/// Navigation dans la grille
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

impl Display for GridSurfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllCells => write!(f, "Toute la grille"),
            Self::Region(region) => write!(f, "Region '{region}'"),
            Self::Adjacent(line_column) => write!(f, "Cases adjacentes à '{line_column}'"),
            Self::Line(line) => write!(f, "Ligne {line}"),
            Self::Column(column) => write!(f, "Colonne {column}"),
        }
    }
}

impl GridHandler {
    /// Retourne la liste des cases d'une grille qui satisfont à un certain critère.<br>
    /// Le critère est défini par l'énumération `GridSurfer`
    #[must_use]
    pub fn surfer(&self, grid: &Grid, surfer: GridSurfer) -> Vec<LineColumn> {
        let mut cells = Vec::new();
        for line in 0..self.nb_lines() {
            for column in 0..self.nb_columns() {
                let line_column = LineColumn::new(line, column);
                let cell: &GridCell = grid.cell(line_column);
                let cell_is_matching = match surfer {
                    // Toutes les case de la grille
                    GridSurfer::AllCells => true,
                    // Toutes les cases d'une région
                    GridSurfer::Region(region) => cell.region == region,
                    // Toutes les cases adjacentes à une case donnée (y compris les diagonales)
                    GridSurfer::Adjacent(line_column) => {
                        let adjacent_cells = self.adjacent_cells(line_column);
                        adjacent_cells
                            .iter()
                            .any(|cell| cell.line == line && cell.column == column)
                    }
                    // Toutes les cases d'une ligne
                    GridSurfer::Line(select_line) => select_line == line,
                    // Toutes les cases d'une colonne
                    GridSurfer::Column(select_column) => select_column == column,
                };
                if cell_is_matching {
                    cells.push(line_column);
                }
            }
        }

        cells
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
        let surfer = grid_handler.surfer(&grid, GridSurfer::AllCells);
        assert_eq!(
            surfer.len(),
            grid_handler.nb_lines() * grid_handler.nb_columns()
        );
    }

    #[test]
    fn test_region() {
        let (grid_handler, grid) = get_test_grid();
        let surfer = grid_handler.surfer(&grid, GridSurfer::Region('A'));
        assert_eq!(surfer, vec![LineColumn::new(0, 0), LineColumn::new(1, 0)]);
    }

    #[test]
    fn test_adjacent() {
        let (grid_handler, grid) = get_test_grid();
        // 8 cases adjacentes à la case (2, 2) au milieu de la grille
        let surfer = grid_handler.surfer(&grid, GridSurfer::Adjacent(LineColumn::new(2, 2)));
        assert_eq!(surfer.len(), 8);
    }

    #[test]
    fn test_line() {
        let (grid_handler, grid) = get_test_grid();
        // 5 cases de la 2eme ligne
        let surfer = grid_handler.surfer(&grid, GridSurfer::Line(1));
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
        let surfer = grid_handler.surfer(&grid, GridSurfer::Column(1));
        assert_eq!(
            surfer
                .iter()
                .filter(|line_column| line_column.column == 1)
                .count(),
            5
        );
    }
}
