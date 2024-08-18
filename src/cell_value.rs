//! Énumération des valeurs possibles d'une case de la grille

/// Valeur possible d'une case de la grille
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum CellValue {
    /// Case dont le contenu est inconnu
    #[default]
    Unknown,

    /// Case dont le contenu est une étoile
    Star,

    /// Case dont le contenu n'est pas une étoile
    NoStar,
}
