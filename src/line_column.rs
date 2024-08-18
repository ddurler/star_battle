//! Help for grid line and column coordinates.

/// Coordonnées d'une case de la grille (`line`, `column`) base 0
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LineColumn {
    /// Numéro de la ligne (base 0). Ligne 0 correspond à la première ligne u haut.
    pub line: usize,

    /// Numéro de la colonne (base 0). Colonne 0 correspond à la première colonne de gauche
    pub column: usize,
}

impl From<(usize, usize)> for LineColumn {
    fn from((line, column): (usize, usize)) -> Self {
        Self { line, column }
    }
}

impl LineColumn {
    /// Constructeur
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Ligne de la case dans la grille (base 0)
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Colonne de la case dans la grille (base 0)
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let lc = LineColumn::from((1, 2));
        assert_eq!(lc.line(), 1);
        assert_eq!(lc.column(), 2);
    }

    #[test]
    fn test_new() {
        let lc = LineColumn::new(2, 1);
        assert_eq!(lc.line(), 2);
        assert_eq!(lc.column(), 1);
    }

    #[test]
    fn test_eq() {
        assert_eq!(LineColumn::new(1, 2), LineColumn::from((1, 2)));
    }
}
