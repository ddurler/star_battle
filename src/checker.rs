//! Vérifie la validité d'une grille parsée

use super::LineColumn;
use super::{ParsedCell, Parser};

pub struct Checker {
    /// Grille parsée
    parser: Parser,
}

impl Checker {
    /// Constructeur d'un 'checker' d'une grille parsée
    pub const fn new(parser: Parser) -> Self {
        Self { parser }
    }

    /// Vérifie la validité d'une grille parsée
    pub fn check(&self) -> Result<(), String> {
        for region in &self.parser.regions() {
            if !self.region_ok(*region) {
                return Err(format!(
                    "La region '{region}' n'est pas un bloc consistant dans cette grille",
                ));
            }
        }

        Ok(())
    }

    /// Vérifie la validité d'une région de la grille
    fn region_ok(&self, region: char) -> bool {
        // Liste des cases de la région
        let all_region_cells = self.parser.region_cells(region);
        if all_region_cells.is_empty() {
            return false;
        }

        // Première case de la region
        let first_cell = all_region_cells[0].clone();

        // On construit la liste de toutes les cases adjacentes à cette 'first_cell'
        // Pour cela, on a une liste des cases à parcourir qu'on initialise avec first_cell et qu'on
        // enrichit des cases adjacentes qui sont dans la zone.
        let mut cells_to_check = vec![first_cell];
        let mut cells_checked = vec![];

        while let Some(current_cell) = cells_to_check.pop() {
            // Traitement d'une case à vérifier de la région
            if !cells_checked.contains(&current_cell) {
                // Pas déjà vérifiée...
                cells_checked.push(current_cell.clone());

                // Liste des cases adjacentes à cette case dans la région...
                let adjacent_region_cells = self.adjacent_region_cells(&current_cell);

                // ... qu'on ajoute à la liste des cases à traiter si pas déjà traitées
                for adjacent_region_cell in &adjacent_region_cells {
                    if !cells_checked.contains(adjacent_region_cell) {
                        cells_to_check.push(adjacent_region_cell.clone());
                    }
                }
            }
        }

        // Ici, 'cells_checked' contient toutes les cases de la region.
        // On doit en avoir le même nombre que celles de la grille
        cells_checked.len() == all_region_cells.len()
    }

    // Liste des case adjacentes à une case
    fn adjacent_cells(&self, cell: &ParsedCell) -> Vec<ParsedCell> {
        let mut cells = vec![];
        let (line, column) = (cell.line_column.line, cell.line_column.column);

        // North ?
        if line > 0 {
            cells.push(
                self.parser
                    .cell(&LineColumn::new(line - 1, column))
                    .unwrap(),
            );
        }

        // South ?
        if line < self.parser.nb_lines() - 1 {
            cells.push(
                self.parser
                    .cell(&LineColumn::new(line + 1, column))
                    .unwrap(),
            );
        }

        // West ?
        if column > 0 {
            cells.push(
                self.parser
                    .cell(&LineColumn::new(line, column - 1))
                    .unwrap(),
            );
        }

        // East ?
        if column < self.parser.nb_columns() - 1 {
            cells.push(
                self.parser
                    .cell(&LineColumn::new(line, column + 1))
                    .unwrap(),
            );
        }

        cells
    }

    /// Liste des cases adjacentes à la case (line, column) de la même région
    fn adjacent_region_cells(&self, cell: &ParsedCell) -> Vec<ParsedCell> {
        self.adjacent_cells(cell)
            .iter()
            .filter(|c| c.region == cell.region)
            .cloned()
            .collect()
    }
}
