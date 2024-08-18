//! Case de la grille

use crate::CellValue;
use crate::LineColumn;
use crate::Region;

/// Case de la grille
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridCell {
    /// Coordonnées de la case dans la grille
    pub line_column: LineColumn,

    /// Région de la case
    pub region: Region,

    /// Valeur de la case
    pub value: CellValue,
}
