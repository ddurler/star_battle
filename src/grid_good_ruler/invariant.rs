//! Recherche des cases invariantes dans une collection de grilles

use crate::CellValue;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

/// Énumération de la situation pour les cases possiblement variantes dans toutes les
/// combinaisons possibles de grilles
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    // Case initialement inconnue
    Init,

    // Case vue comme une étoile dans une des grilles possibles
    Star,

    // Case vue comme sans étoile dans une des grilles possibles
    NoStar,

    // Case vue comme inconnue dans une des grilles possibles
    Unknown,

    // Case vue avec différentes possibilités dans les grilles possibles
    Variable,
}

impl Variant {
    /// Retourne l'état `Variant` résultant de la combinaison de 2 `Variant`
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            /* Toutes les cases combinées avec un Variant::Init devient l'autre variant */
            /* Une Variant::Star ne peut se combiner qu'avec un autre Variant::Star sinon c'est Variant::Variant */
            /* Idem pour les Variant::NoStar */
            /* Idem pour les Variant::Unknown */
            /* Les Variant::Variant ne peuvent que rester Variant::Variant */
            (Self::Init, other) | (other, Self::Init) => other,
            (Self::Star, Self::Star) => Self::Star,
            (Self::NoStar, Self::NoStar) => Self::NoStar,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
            (Self::Star | Self::NoStar | Self::Unknown | Self::Variable, _) => Self::Variable,
        }
    }
}

impl Variant {
    /// Examine un ensemble des grilles possibles collectées à partir d'une grille initiale à la recherche
    /// de cases invariantes pour toutes les possibilités de grilles
    pub fn check_for_invariants(
        handler: &GridHandler,
        grid: &Grid,
        possible_grids: &Vec<Grid>,
    ) -> Vec<GridAction> {
        // Liste des cases non déterminées dans la grille initiale
        let mut cells = Vec::new();
        // Liste des 'Variant' de ces cases
        let mut variants = Vec::new();
        for line_column in handler.surfer(grid, &GridSurfer::AllCells) {
            if grid.cell(line_column).is_unknown() {
                cells.push(line_column);
                variants.push(Self::Init);
            }
        }

        // Parcours de toutes les grilles possibles collectées
        for grid in possible_grids {
            // On combine toutes les cases à examiner avec ce qu'on a déjà observé
            for (line_column, variant) in cells.iter().zip(variants.iter_mut()) {
                let prev_variant = *variant;
                let new_variant = prev_variant.combine(match grid.cell(*line_column).value {
                    CellValue::Star => Self::Star,
                    CellValue::NoStar => Self::NoStar,
                    CellValue::Unknown => Self::Unknown,
                });
                *variant = new_variant;
            }
        }

        // Liste des invariants dans toutes les grilles examinées
        let mut invariants_actions = Vec::new();
        for (line_column, variant) in cells.iter().zip(variants.iter()) {
            match variant {
                Self::Star => {
                    /* Cette case est toujours une étoile dans toutes les grilles possibles */
                    invariants_actions.push(GridAction::SetStar(*line_column));
                }
                Self::NoStar => {
                    /* Cette case n'est jamais une étoile dans toutes les grilles possibles */
                    invariants_actions.push(GridAction::SetNoStar(*line_column));
                }
                _ => (),
            }
        }

        invariants_actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_variants() {
        let expected = vec![
            /* (variant1, variant2), expected variant1.combine(variant2) */

            /* Toutes les cases combinées avec un Variant::Init devient l'autre variant */
            ((Variant::Init, Variant::Init), Variant::Init),
            ((Variant::Init, Variant::Star), Variant::Star),
            ((Variant::Init, Variant::NoStar), Variant::NoStar),
            ((Variant::Init, Variant::Unknown), Variant::Unknown),
            ((Variant::Init, Variant::Variable), Variant::Variable),
            ((Variant::Star, Variant::Init), Variant::Star),
            ((Variant::NoStar, Variant::Init), Variant::NoStar),
            ((Variant::Unknown, Variant::Init), Variant::Unknown),
            ((Variant::Variable, Variant::Init), Variant::Variable),
            /* Une Variant::Star ne peut se combiner qu'avec un autre Variant::Star sinon c'est Variant::Variant */
            ((Variant::Star, Variant::Star), Variant::Star),
            ((Variant::Star, Variant::NoStar), Variant::Variable),
            ((Variant::Star, Variant::Unknown), Variant::Variable),
            ((Variant::Star, Variant::Variable), Variant::Variable),
            /* Une Variant::NoStar ne peut se combiner qu'avec un autre Variant::NoStar sinon c'est Variant::Variant */
            ((Variant::NoStar, Variant::Star), Variant::Variable),
            ((Variant::NoStar, Variant::NoStar), Variant::NoStar),
            ((Variant::NoStar, Variant::Unknown), Variant::Variable),
            ((Variant::NoStar, Variant::Variable), Variant::Variable),
            /* Une Variant::Unknown ne peut se combiner qu'avec un autre Variant::Unknown sinon c'est Variant::Variant */
            ((Variant::Unknown, Variant::Star), Variant::Variable),
            ((Variant::Unknown, Variant::NoStar), Variant::Variable),
            ((Variant::Unknown, Variant::Unknown), Variant::Unknown),
            ((Variant::Unknown, Variant::Variable), Variant::Variable),
            /* Les Variant::Variant ne peuvent que rester Variant::Variant */
            ((Variant::Variable, Variant::Star), Variant::Variable),
            ((Variant::Variable, Variant::NoStar), Variant::Variable),
            ((Variant::Variable, Variant::Unknown), Variant::Variable),
            ((Variant::Variable, Variant::Variable), Variant::Variable),
        ];

        for ((v1, v2), expected) in expected {
            assert_eq!(v1.combine(v2), expected);
        }
    }
}
