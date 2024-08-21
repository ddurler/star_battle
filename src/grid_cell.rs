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

impl GridCell {
    /// Retourne `true` si la case n'est pas définie
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.value == CellValue::Unknown
    }

    /// Retourne `true` si la case ne peut pas être une étoile
    #[must_use]
    pub fn is_no_star(&self) -> bool {
        self.value == CellValue::NoStar
    }

    /// Retourne `true` si la case est une étoile
    #[must_use]
    pub fn is_star(&self) -> bool {
        self.value == CellValue::Star
    }
}
