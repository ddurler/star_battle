//! Recherche des cases toujours adjacentes à une étoile dans une collection de grilles

use crate::CellValue;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

/// Énumération de la situation pour les cases possiblement toujours adjacentes à une étoile
/// dans toutes les combinaisons possibles de grilles
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StarAdjacent {
    // Case initialement inconnue
    Init,

    // Case vue adjacente à une une étoile dans une des grilles possibles
    Always,

    // Case vue avec différentes possibilités dans les grilles possibles
    Variable,
}

impl StarAdjacent {
    /// Examine un ensemble des grilles possibles collectées à partir d'une grille initiale à la recherche
    /// de cases toujours adjacentes à une étoile pour toutes les possibilités de grilles
    pub fn check_for_star_adjacents(
        handler: &GridHandler,
        grid: &Grid,
        possible_grids: &Vec<Grid>,
    ) -> Vec<GridAction> {
        // Liste des cases non déterminées dans la grille initiale
        let mut cells = Vec::new();
        // Liste des 'Variant' de ces cases
        let mut star_adjacents = Vec::new();
        for line_column in handler.surfer(grid, &GridSurfer::AllCells) {
            if grid.cell(line_column).is_unknown() {
                cells.push(line_column);
                star_adjacents.push(Self::Init);
            }
        }

        // Parcours de toutes les grilles possibles collectées
        for grid in possible_grids {
            // On combine toutes les cases à examiner avec ce qu'on a déjà observé
            for (line_column, variant) in cells.iter().zip(star_adjacents.iter_mut()) {
                // Seules les cases avec une situation différente de `CellValue::Star` peuvent prétendre
                // à être toujours adjacentes à une étoile
                if grid.cell(*line_column).value == CellValue::Star {
                    *variant = Self::Variable;
                } else {
                    // Et qu'elles n'ont pas été déjà identifiées comme StarAdjacent::Variable
                    if *variant != Self::Variable {
                        // Liste des cases adjacentes
                        let adjacents = handler.adjacent_cells(*line_column);
                        if adjacents
                            .iter()
                            .any(|line_column| grid.cell(*line_column).value == CellValue::Star)
                        {
                            *variant = Self::Always;
                        } else {
                            *variant = Self::Variable;
                        }
                    }
                }
            }
        }

        // Liste des cases toujours adjacentes à une étoile dans toutes les grilles examinées
        let mut adjacent_star_actions = Vec::new();
        for (line_column, star_adjacent) in cells.iter().zip(star_adjacents.iter()) {
            if star_adjacent == &Self::Always {
                /* Cette case est toujours adjacent à une étoile dans toutes les grilles possibles */
                adjacent_star_actions.push(GridAction::SetNoStar(*line_column));
            }
        }

        adjacent_star_actions
    }
}
