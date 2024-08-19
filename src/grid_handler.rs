//! Structure d'une grille en cours de résolution.

use crate::GridParser;
use crate::LineColumn;
use crate::Region;

/// Description d'une grille en cours de résolution
#[derive(Debug)]
pub struct GridHandler {
    /// Taille de la grille
    size: LineColumn,

    /// Nombre d'étoiles à placer dans chaque ligne, colonne ou région dans la grille
    nb_stars: usize,

    /// Liste des régions de la grille
    regions: Vec<Region>,

    /// Liste des lignes avec la région correspondant à chaque case de la ligne
    cells_region: Vec<Vec<Region>>,
}

impl GridHandler {
    /// Constructeur selon un grid parser et le nombre d'étoiles à placer dans la grille
    /// # Panics
    /// Panic si la taille de la grille est <= 0 ou qu'il y a trop d'étoiles à placer selon la taille de la grille
    #[must_use]
    pub fn new(parser: &GridParser, nb_stars: usize) -> Self {
        let nb_lines = parser.nb_lines();
        let nb_columns = parser.nb_columns();
        assert!(nb_lines > 0, "nb_lines doit être > 0");
        assert!(nb_columns > 0, "nb_columns doit être > 0");

        // Liste des regions de la grille
        let mut regions: Vec<char> = parser.regions();
        // Tri par taille de la region (en nombre de cases)
        regions.sort_by(|a, b| {
            parser
                .region_cells(*a)
                .len()
                .cmp(&parser.region_cells(*b).len())
        });

        // Pour mettre nb_stars sans qu'elles se touchent, il faut au moins ((2 * nb_stars) - 1) cases...
        let min_nb_cells = (2 * nb_stars) - 1;
        assert!(nb_stars > 0, "nb_stars doit être > 0");
        assert!(
            nb_lines >= min_nb_cells,
            "Trop d'étoiles à placer ({nb_stars}) pour une grille de {nb_lines} lignes"
        );
        assert!(
            nb_columns >= min_nb_cells,
            "Trop d'étoiles à placer ({nb_stars}) pour une grille de {nb_columns} colonnes"
        );
        for region in parser.regions() {
            let nb_cells = parser.region_cells(region).len();
            assert!(nb_cells >= min_nb_cells,
                "Trop d'étoiles à placer ({nb_stars}) pour la region '{region}' de {nb_cells} cases dans la grille");
        }

        // Reconstruction de la région de chaque case
        let mut cells_region = Vec::with_capacity(nb_lines);
        for line in 0..nb_lines {
            let mut vec_line_regions = Vec::with_capacity(nb_columns);
            for column in 0..nb_columns {
                vec_line_regions.push(parser.cell(&LineColumn::new(line, column)).unwrap().region);
            }
            cells_region.push(vec_line_regions);
        }

        Self {
            size: LineColumn::new(nb_lines, nb_columns),
            regions,
            cells_region,
            nb_stars,
        }
    }

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

    /// Nombre d'étoiles à placer dans la grille
    #[must_use]
    pub const fn nb_stars(&self) -> usize {
        self.nb_stars
    }

    /// Liste des régions de la grille
    #[must_use]
    pub fn regions(&self) -> Vec<Region> {
        self.regions.clone()
    }

    /// Région d'une case de la grille
    #[must_use]
    pub fn cell_region(&self, line_column: &LineColumn) -> Region {
        self.cells_region[line_column.line][line_column.column]
    }

    /// Liste des cases adjacentes d'une case de la grille (y compris en diagonale)
    #[must_use]
    pub fn adjacent_cells(&self, line_column: &LineColumn) -> Vec<LineColumn> {
        let (line, column) = (line_column.line, line_column.column);
        let mut adjacent_cells = vec![];
        // North
        if line > 0 {
            adjacent_cells.push(LineColumn::new(line - 1, column));
            // North-West
            if column > 0 {
                adjacent_cells.push(LineColumn::new(line - 1, column - 1));
            }
            // North-East
            if column < (self.nb_columns() - 1) {
                adjacent_cells.push(LineColumn::new(line - 1, column + 1));
            }
        }
        // West
        if column > 0 {
            adjacent_cells.push(LineColumn::new(line, column - 1));
            // South-West
            if line < (self.nb_lines() - 1) {
                adjacent_cells.push(LineColumn::new(line + 1, column - 1));
            }
        }
        // East
        if line < (self.nb_lines() - 1) {
            adjacent_cells.push(LineColumn::new(line + 1, column));
            // South-East
            if column < (self.nb_columns() - 1) {
                adjacent_cells.push(LineColumn::new(line + 1, column + 1));
            }
        }
        // South
        if column < (self.nb_columns() - 1) {
            adjacent_cells.push(LineColumn::new(line, column + 1));
        }
        adjacent_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_ok() {
        let parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let handler = GridHandler::new(&parser, 1);

        assert_eq!(handler.nb_lines(), 5);
        assert_eq!(handler.nb_columns(), 5);
        assert_eq!(handler.nb_stars(), 1);
        for region in ['A', 'B', 'C', 'D', 'E'] {
            assert!(handler.regions().contains(&region));
        }

        // Région A
        assert_eq!(handler.cell_region(&LineColumn::new(0, 0)), 'A');
        assert_eq!(handler.cell_region(&LineColumn::new(1, 0)), 'A');

        // Région B
        assert_eq!(handler.cell_region(&LineColumn::new(0, 1)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(0, 2)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(0, 3)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(0, 4)), 'B');

        assert_eq!(handler.cell_region(&LineColumn::new(1, 1)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(1, 2)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(1, 3)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(1, 4)), 'B');

        assert_eq!(handler.cell_region(&LineColumn::new(2, 2)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(2, 3)), 'B');
        assert_eq!(handler.cell_region(&LineColumn::new(2, 4)), 'B');

        // Région C
        assert_eq!(handler.cell_region(&LineColumn::new(2, 0)), 'C');
        assert_eq!(handler.cell_region(&LineColumn::new(2, 1)), 'C');

        // Région D
        assert_eq!(handler.cell_region(&LineColumn::new(3, 0)), 'D');
        assert_eq!(handler.cell_region(&LineColumn::new(3, 1)), 'D');
        assert_eq!(handler.cell_region(&LineColumn::new(3, 2)), 'D');
        assert_eq!(handler.cell_region(&LineColumn::new(3, 3)), 'D');
        assert_eq!(handler.cell_region(&LineColumn::new(3, 4)), 'D');

        assert_eq!(handler.cell_region(&LineColumn::new(4, 0)), 'D');
        assert_eq!(handler.cell_region(&LineColumn::new(4, 4)), 'D');

        // Région E
        assert_eq!(handler.cell_region(&LineColumn::new(4, 1)), 'E');
        assert_eq!(handler.cell_region(&LineColumn::new(4, 2)), 'E');
        assert_eq!(handler.cell_region(&LineColumn::new(4, 3)), 'E');
    }

    #[test]
    #[rustfmt::skip]
    fn test_adjacent() {
        fn assert_adjacents(handler: &GridHandler, (line, column):(usize, usize), expected: Vec<(usize, usize)>, ) {
            let adjacent_cells:HashSet<LineColumn> = handler.adjacent_cells(&LineColumn::new(line, column)).into_iter().collect();
            let expected_cells:HashSet<LineColumn> = expected.into_iter().map(|(line, column)| LineColumn::new(line, column)).collect();
            assert_eq!(adjacent_cells, expected_cells);
        }

        //  A A A
        //  B B B
        //  C C C
        let parser =
            GridParser::try_from(vec!["AAA", "BBB", "CCC"]).unwrap();
        let handler = GridHandler::new(&parser, 1);

        assert_adjacents(&handler, (0, 0), vec![(0, 1), (1, 0), (1, 1)]);
        assert_adjacents(&handler, (0, 1), vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]);
        assert_adjacents(&handler, (0, 2), vec![(0, 1), (1, 1), (1, 2)]);
        assert_adjacents(&handler, (1, 0), vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]);
        assert_adjacents(&handler, (1, 1), vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
        assert_adjacents(&handler, (1, 2), vec![(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)]);
        assert_adjacents(&handler, (2, 0), vec![(1, 0), (1, 1), (2, 1),]);
        assert_adjacents(&handler, (2, 1), vec![(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)]);
        assert_adjacents(&handler, (2, 2), vec![(1, 1), (1, 2), (2, 1)]);
    }
}
