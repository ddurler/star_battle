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

        let mut cells_region = Vec::with_capacity(nb_lines);
        let mut regions = parser.regions();
        regions.sort_unstable();

        // Pour mettre nb_stars sans qu'elles se touchent, il faut au moins (2 * nb_stars) - 1 cases...
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_ok() {
        let parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid = GridHandler::new(&parser, 1);

        assert_eq!(grid.nb_lines(), 5);
        assert_eq!(grid.nb_columns(), 5);
        assert_eq!(grid.nb_stars(), 1);
        assert_eq!(grid.regions(), vec!['A', 'B', 'C', 'D', 'E']);

        // Région A
        assert_eq!(grid.cell_region(&LineColumn::new(0, 0)), 'A');
        assert_eq!(grid.cell_region(&LineColumn::new(1, 0)), 'A');

        // Région B
        assert_eq!(grid.cell_region(&LineColumn::new(0, 1)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(0, 2)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(0, 3)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(0, 4)), 'B');

        assert_eq!(grid.cell_region(&LineColumn::new(1, 1)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(1, 2)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(1, 3)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(1, 4)), 'B');

        assert_eq!(grid.cell_region(&LineColumn::new(2, 2)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(2, 3)), 'B');
        assert_eq!(grid.cell_region(&LineColumn::new(2, 4)), 'B');

        // Région C
        assert_eq!(grid.cell_region(&LineColumn::new(2, 0)), 'C');
        assert_eq!(grid.cell_region(&LineColumn::new(2, 1)), 'C');

        // Région D
        assert_eq!(grid.cell_region(&LineColumn::new(3, 0)), 'D');
        assert_eq!(grid.cell_region(&LineColumn::new(3, 1)), 'D');
        assert_eq!(grid.cell_region(&LineColumn::new(3, 2)), 'D');
        assert_eq!(grid.cell_region(&LineColumn::new(3, 3)), 'D');
        assert_eq!(grid.cell_region(&LineColumn::new(3, 4)), 'D');

        assert_eq!(grid.cell_region(&LineColumn::new(4, 0)), 'D');
        assert_eq!(grid.cell_region(&LineColumn::new(4, 4)), 'D');

        // Région E
        assert_eq!(grid.cell_region(&LineColumn::new(4, 1)), 'E');
        assert_eq!(grid.cell_region(&LineColumn::new(4, 2)), 'E');
        assert_eq!(grid.cell_region(&LineColumn::new(4, 3)), 'E');
    }
}
