//! Parser des lignes 'textuelles' de définition d'une grille.
//!
//! Un fichier au format 'texte' est utilisé pour définir la grille initiale.
//!
//! Dans ce fichier, chaque ligne de texte correspond à une ligne de la grille. Les différentes régions sont identifiées par une 'lettre' distincte dans la case correspondante.
//!
//! Les lignes 'vides' ou qui commencent par l'un des caractères suivants sont ignorées : '*', '#', '/' (considérées comme d'éventuels commentaires dans le fichier).
//!
//! Chaque ligne 'utile' de ce fichier doit définir le même nombre de cases. Elles doivent donc toutes avoir la même longueur.
//!
//! Par exemple :
//!
//! ```text
//! # Exemple de grille 1★
//! ABBBB
//! ABBBB
//! CCBBB
//! DDDDD
//! DEEED
//! ```

use std::collections::HashSet;

use super::Checker;

/// Caractères de commentaire au début d'une ligne du fichier pour une grille à résoudre
const COMMENT_CHARS: [char; 3] = ['#', ';', '@'];

/// Caractères non admissibles comme symboles d'une région
const ILLEGAL_REGION_CHARS: [char; 4] = [' ', '\t', '\n', '\r'];

/// Case de la grille
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ParsedCell {
    /// Ligne de la case dans la grille
    pub line: usize,

    /// Colonne de la case dans la grille
    pub column: usize,

    /// Région de la case
    pub region: char,
}

/// Ligne de la grille
#[derive(Clone, Debug, Default)]
struct ParsedLine(Vec<ParsedCell>);

/// Grille
#[derive(Clone, Debug, Default)]
struct ParsedGrid(Vec<ParsedLine>);

/// Grid parser
#[derive(Clone, Debug, Default)]
pub struct Parser {
    /// Symboles identifiés comme 'région' dans la grille
    pub regions: HashSet<char>,

    /// Grille parsée
    parsed_grid: ParsedGrid,
}

impl Parser {
    /// Constructeur d'une grille à résoudre selon une liste de `Strings` où chaque `String` est la définition
    /// textuelle du contenu d'une ligne de la grille à résoudre
    pub fn try_from_text_lines(lines: &[String]) -> Result<Self, String> {
        let mut grid_parsed = Parser::default();
        // Parsing des lignes de la définition de la grille
        for (num_line, text_line) in lines.iter().enumerate() {
            let text_line = text_line.trim();
            if !text_line.is_empty() && !text_line.starts_with(COMMENT_CHARS) {
                if let Err(e) = grid_parsed.parse_text_line(text_line) {
                    return Err(format!(
                        "Erreur à la ligne #{} '{}': {}",
                        num_line + 1,
                        text_line,
                        e
                    ));
                }
            }
        }

        // Des régions identifiées ?
        if grid_parsed.regions.is_empty() || grid_parsed.parsed_grid.0.is_empty() {
            return Err("La grille n'a aucune région définie".to_string());
        }

        // Contrôle de la grille parsée
        let checker = Checker::new(grid_parsed.clone());
        checker.check()?;

        Ok(grid_parsed)
    }

    /// Nombre de lignes dans la grille parsée
    pub fn nb_lines(&self) -> usize {
        self.parsed_grid.0.len()
    }

    /// Nombre de colonnes dans la grille parsée
    pub fn nb_columns(&self) -> usize {
        self.parsed_grid.0[0].0.len()
    }

    /// Retourne la case de la grille en (line, column) (si existe)
    pub fn cell(&self, line: usize, column: usize) -> Option<ParsedCell> {
        if line < self.nb_lines() && column < self.nb_columns() {
            Some(self.parsed_grid.0[line].0[column].clone())
        } else {
            None
        }
    }

    /// région de la case (line, column)
    pub fn region(&self, line: usize, column: usize) -> char {
        self.parsed_grid.0[line].0[column].region
    }

    /// Liste des cases d'une grille parsée
    pub fn list_cells(&self) -> Vec<ParsedCell> {
        let mut cells = vec![];
        for line_parsed in &self.parsed_grid.0 {
            for cell in &line_parsed.0 {
                cells.push(cell.clone());
            }
        }
        cells
    }

    /// Liste des cases d'une région d'une grille parsée
    pub fn region_cells(&self, region: char) -> Vec<ParsedCell> {
        self.list_cells()
            .iter()
            .filter(|c| c.region == region)
            .cloned()
            .collect()
    }

    /// Analyse une ligne textuelle de définition d'une ligne la grille.
    /// Ici, la ligne textuelle n'est pas vide et n'est pas un commentaire.
    fn parse_text_line(&mut self, text_line: &str) -> Result<(), String> {
        let mut line_parsed = ParsedLine::default();
        let line = self.parsed_grid.0.len();

        // Parsing de la ligne
        for (column, region) in text_line.chars().enumerate() {
            if ILLEGAL_REGION_CHARS.contains(&region) {
                return Err(format!(
                    "Le symbole '{region}' n'est pas valide pour identifier une région"
                ));
            }
            self.regions.insert(region);
            let cur_cell = ParsedCell {
                line,
                column,
                region,
            };
            line_parsed.0.push(cur_cell);
        }

        // Nombre de colonnes correct ?
        if !self.parsed_grid.0.is_empty() && self.parsed_grid.0[0].0.len() != line_parsed.0.len() {
            return Err("La ligne de la grille n'est pas la même longueur".to_string());
        }

        // Ajout de la ligne à la grille
        self.parsed_grid.0.push(line_parsed);
        Ok(())
    }
}

impl TryFrom<&str> for Parser {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<String> = value.split('\n').map(|s: &str| s.to_string()).collect();
        Parser::try_from_text_lines(&lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_ok() {
        let result_grid = Parser::try_from(
            "
            # Exemple de grille 1★
            ABBBB
            ABBBB
            CCBBB
            DDDDD
            DEEED
        ",
        );
        assert!(result_grid.is_ok());

        let grid = result_grid.unwrap();
        assert_eq!(grid.nb_lines(), 5);
        assert_eq!(grid.nb_columns(), 5);

        // Région A
        assert_eq!(grid.region(0, 0), 'A');
        assert_eq!(grid.region(1, 0), 'A');

        // Région B
        assert_eq!(grid.region(0, 1), 'B');
        assert_eq!(grid.region(0, 2), 'B');
        assert_eq!(grid.region(0, 3), 'B');
        assert_eq!(grid.region(0, 4), 'B');

        assert_eq!(grid.region(1, 1), 'B');
        assert_eq!(grid.region(1, 2), 'B');
        assert_eq!(grid.region(1, 3), 'B');
        assert_eq!(grid.region(1, 4), 'B');

        assert_eq!(grid.region(2, 2), 'B');
        assert_eq!(grid.region(2, 3), 'B');
        assert_eq!(grid.region(2, 4), 'B');

        // Région C
        assert_eq!(grid.region(2, 0), 'C');
        assert_eq!(grid.region(2, 1), 'C');

        // Région D
        assert_eq!(grid.region(3, 0), 'D');
        assert_eq!(grid.region(3, 1), 'D');
        assert_eq!(grid.region(3, 2), 'D');
        assert_eq!(grid.region(3, 3), 'D');
        assert_eq!(grid.region(3, 4), 'D');

        assert_eq!(grid.region(4, 0), 'D');
        assert_eq!(grid.region(4, 4), 'D');

        // Région E
        assert_eq!(grid.region(4, 1), 'E');
        assert_eq!(grid.region(4, 2), 'E');
        assert_eq!(grid.region(4, 3), 'E');
    }

    // Toutes les grilles suivantes sont invalides
    const INVALID_GRIDS: [&str; 4] = [
        "
            # Grille invalide: Vide de toute définition
            # Manque des définitions de symboles
        ",
        "
            # Grille invalide: Symboles non admissibles
            A\tA
            BBB
        ",
        "
            # Grille invalide: Nombre inconsistant de colonnes
            AAA
            BB
        ",
        "
            # Grille invalide: Zone inconsistante
            AAA
            BBA
            AAB
        ",
    ];

    #[test]
    fn test_try_from_nok() {
        for s in INVALID_GRIDS {
            let grid = Parser::try_from(s);
            assert!(grid.is_err());
        }
    }
}
