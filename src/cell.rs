//! Case de la grille

use crate::LineColumn;
use crate::Value;

/// Case de la grille
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cell {
    /// Coordonnées de la case dans la grille
    pub line_column: LineColumn,

    /// Région de la case
    pub region: char,

    /// Valeur de la case
    pub value: Value,
}
