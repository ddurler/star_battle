//! Action possible sur une grille.

use std::fmt::Display;

use crate::CellValue;
use crate::Grid;
use crate::LineColumn;

/// Énumération des actions possibles sur le contenu d'une grille
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GridAction {
    /// L'action d'indiquer le contenu indéfini d'une case
    SetUnknown(LineColumn),

    /// L'action d'ajouter une étoile à une case
    SetStar(LineColumn),

    /// L'action de supprimer la possibilité d'une étoile à une case
    SetNoStar(LineColumn),
}

impl Display for GridAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetUnknown(line_column) => write!(f, "{line_column}-> Inconnu"),
            Self::SetStar(line_column) => write!(f, "{line_column}->Etoile"),
            Self::SetNoStar(line_column) => write!(f, "{line_column}->Pas d'étoile"),
        }
    }
}

/// Affichage d'une liste d'actions
pub fn display_vec_actions(actions: &Vec<GridAction>) -> String {
    let mut str_actions = String::new();
    for action in actions {
        if !str_actions.is_empty() {
            str_actions.push_str(", ");
        }
        str_actions.push_str(&action.to_string());
    }
    str_actions
}

impl GridAction {
    /// Retourne la `LineColumn` correspondant à l'action
    #[must_use]
    pub const fn line_column(&self) -> LineColumn {
        match self {
            Self::SetUnknown(line_column)
            | Self::SetStar(line_column)
            | Self::SetNoStar(line_column) => *line_column,
        }
    }

    /// Retourne la `CellValue` correspondant à l'action
    #[must_use]
    pub const fn value(&self) -> CellValue {
        match self {
            Self::SetUnknown(_) => CellValue::Unknown,
            Self::SetStar(_) => CellValue::Star,
            Self::SetNoStar(_) => CellValue::NoStar,
        }
    }

    /// Applique une action à la grille
    pub fn apply_action(&self, grid: &mut Grid) {
        match self {
            Self::SetUnknown(line_column) => {
                grid.cell_mut(*line_column).value = CellValue::Unknown;
            }
            Self::SetStar(line_column) => {
                grid.cell_mut(*line_column).value = CellValue::Star;
            }
            Self::SetNoStar(line_column) => {
                grid.cell_mut(*line_column).value = CellValue::NoStar;
            }
        }
    }
}

impl Grid {
    /// Applique une action à la grille
    pub fn apply_action(&mut self, action: &GridAction) {
        match action {
            GridAction::SetUnknown(line_column) => {
                self.cell_mut(*line_column).value = CellValue::Unknown;
            }
            GridAction::SetStar(line_column) => {
                self.cell_mut(*line_column).value = CellValue::Star;
            }
            GridAction::SetNoStar(line_column) => {
                self.cell_mut(*line_column).value = CellValue::NoStar;
            }
        }
    }
}
