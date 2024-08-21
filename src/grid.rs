//! Contenu des case de la grille.

use crate::CellValue;
use crate::GridCell;
use crate::GridHandler;
use crate::LineColumn;

/// Cases de la grille
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Grid {
    /// Dimensions de la grille
    size: LineColumn,

    /// Cases de la grille
    cells: Vec<Vec<GridCell>>,
}

impl From<&GridHandler> for Grid {
    fn from(value: &GridHandler) -> Self {
        let nb_lines = value.nb_lines();
        let nb_columns = value.nb_columns();
        let mut cells = Vec::with_capacity(nb_lines);
        for line in 0..nb_lines {
            let mut cells_line = Vec::with_capacity(nb_columns);
            for column in 0..nb_columns {
                let line_column = LineColumn::new(line, column);
                let grid_cell = GridCell {
                    line_column,
                    region: value.cell_region(line_column),
                    value: CellValue::Unknown,
                };
                cells_line.push(grid_cell);
            }
            cells.push(cells_line);
        }
        Self {
            size: LineColumn::new(nb_lines, nb_columns),
            cells,
        }
    }
}

impl Grid {
    /// Nombre de lignes de la grille
    #[must_use]
    pub const fn nb_lines(&self) -> usize {
        self.size.line
    }

    /// Nombre de colonnes de la grille
    #[must_use]
    pub const fn nb_columns(&self) -> usize {
        self.size.column
    }

    /// Retourne la case (non mutable) de la grille en (line, column)
    #[must_use]
    pub fn cell(&self, line_column: LineColumn) -> &GridCell {
        &self.cells[line_column.line][line_column.column]
    }

    /// Retourne la case (mutable) de la grille en (line, column)
    #[must_use]
    pub fn cell_mut(&mut self, line_column: LineColumn) -> &mut GridCell {
        &mut self.cells[line_column.line][line_column.column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GridParser;

    #[test]
    fn test_from_grid_handler() {
        let parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let handler = GridHandler::new(&parser, 1);
        let grid = Grid::from(&handler);

        assert_eq!(grid.nb_lines(), 5);
        assert_eq!(grid.nb_columns(), 5);

        for line in 0..grid.nb_lines() {
            for column in 0..grid.nb_columns() {
                let line_column = LineColumn::new(line, column);
                assert_eq!(grid.cell(line_column).value, CellValue::Unknown);
            }
        }
    }

    #[test]
    fn test_clone_cell_mut() {
        let parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let handler = GridHandler::new(&parser, 1);
        let grid = Grid::from(&handler);

        let mut grid_cloned = grid.clone();
        let line_column = LineColumn::new(0, 0);
        grid_cloned.cell_mut(line_column).value = CellValue::Star;
        assert_eq!(grid.cell(line_column).value, CellValue::Unknown);
        assert_eq!(grid_cloned.cell(line_column).value, CellValue::Star);
    }
}
