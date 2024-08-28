//! Règle de construction/résolution d'une grille.
//!
//! Recherche les cases invariantes pour toutes les combinaisons possibles d'une zone

use crate::CellValue;
use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

use super::collector::Collector;
use super::invariant::Variant;
use super::star_adjacent::StarAdjacent;

/// Énumération des différentes zones possibles pour être examinées
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ZoneToExamine {
    Region,
    LineAndColumn,
    MultipleLinesAndColumns(usize),
}

/// Méthode générique qui cherche toutes les combinaisons possibles dans les différentes zones ou régions
pub fn rule_generic_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
    zone_to_examine: ZoneToExamine,
    recursive: bool,
) -> Option<GoodRule> {
    // Pour simplifier la règle présentée à un humain, on retient la région qui génère un minimum
    // de grilles pour placer toutes les étoiles
    #[derive(Debug, Default)]
    struct BestCollector {
        grid_surfer: Option<GridSurfer>,
        nb_possible_grids: usize,
        invariant_actions: Vec<GridAction>,
    }

    // zones: [(GridSurfer, nb_stars, combinaisons_count)]
    let mut zones = Vec::new();

    // Closure pour compléter la liste des zones à examiner (évite les répétitions de paramètres)
    let mut add_zone = |grid_surfer: GridSurfer, nb_stars: usize| {
        let nb_combinaisons = combinaisons_count(handler, grid, &grid_surfer, nb_stars);
        zones.push((grid_surfer, nb_stars, nb_combinaisons));
    };

    match zone_to_examine {
        ZoneToExamine::Region => {
            // Parcours de toutes les régions
            for region in handler.regions() {
                add_zone(GridSurfer::Region(region), handler.nb_stars());
            }
        }
        ZoneToExamine::LineAndColumn => {
            // Parcours de toutes les lignes
            for line in 0..handler.nb_lines() {
                add_zone(GridSurfer::Line(line), handler.nb_stars());
            }
            // Parcours de toutes les colonnes
            for column in 0..handler.nb_columns() {
                add_zone(GridSurfer::Column(column), handler.nb_stars());
            }
        }
        ZoneToExamine::MultipleLinesAndColumns(2) => {
            // Double-lignes
            for line in 0..handler.nb_lines() - 1 {
                add_zone(GridSurfer::Lines(line..=line + 1), 2 * handler.nb_stars());
            }

            // Double-colonnes
            for column in 0..handler.nb_columns() - 1 {
                add_zone(
                    GridSurfer::Columns(column..=column + 1),
                    2 * handler.nb_stars(),
                );
            }
        }
        ZoneToExamine::MultipleLinesAndColumns(_) => {
            todo!(
                "rule_multi_lines_columns_recursive_possible_stars pour plus de 2 lignes/colonnes"
            )
        }
    }

    // Tri des différentes zones par ordre croissant de combinaisons possible
    zones.sort_by(|a, b| a.2.cmp(&b.2));

    let mut best_collector = BestCollector::default();
    // Examine les différentes zones
    for (grid_surfer, nb_stars, _) in zones {
        let (invariant_actions, nb_possible_grids) =
            try_star_complete(handler, grid, &grid_surfer, nb_stars, recursive);
        if !invariant_actions.is_empty()
        // La règle s'applique pour cette zone...
            && (best_collector.grid_surfer.is_none()
            // ... et c'est la première zone qui permet d'appliquer la règle...
                || nb_possible_grids < best_collector.nb_possible_grids)
        // ... ou le nombre de grilles possibles est moindre que ce qu'on a déjà vu
        {
            best_collector = BestCollector {
                grid_surfer: Some(grid_surfer),
                nb_possible_grids,
                invariant_actions,
            };
        }
    }
    // Règle trouvée ?
    if best_collector.grid_surfer.is_some() {
        Some(GoodRule::InvariantWithZone(
            best_collector.grid_surfer.unwrap(),
            best_collector.invariant_actions,
        ))
    } else {
        None
    }
}

/// Calcul le nombre de combinaisons possible pour placer toutes les étoiles dans une zone
fn combinaisons_count(
    grid_handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
) -> usize {
    // Nombre d'étoiles déjà placées dans la zone
    let cur_nb_stars =
        grid_handler.surfer_cells_with_value_count(grid, grid_surfer, &CellValue::Star);
    if cur_nb_stars >= nb_stars {
        return usize::MAX; // Pas de combinaison possible
    }
    // Nombre d'étoiles restant à placer dans la zone
    let nb_stars_left = nb_stars - cur_nb_stars;
    // Nombre de case non définies dans la zone
    let mut nb_cells =
        grid_handler.surfer_cells_with_value_count(grid, grid_surfer, &CellValue::Unknown);
    if nb_cells <= nb_stars_left {
        return 0; // Pas de combinaison possible
    }
    let mut nb_combinaisons = 1;
    for _ in 0..nb_stars_left {
        // Pour chaque étoile restant à placer, on ajoute le nombre de combinaisons possible
        nb_combinaisons *= nb_cells;
        nb_cells -= 1;
    }
    nb_combinaisons
}

/// Vérifie si la règle est applicable sur la région définie.<br>
/// Si applicable, retourne la liste des actions déduites par la règle et le nombre de grilles possibles
/// qui ont été examinées pour ces actions
fn try_star_complete(
    handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
    recursive: bool,
) -> (Vec<GridAction>, usize) {
    let surfer = handler.surfer(grid, grid_surfer);
    let mut collector = Collector::new(handler, grid, &surfer, nb_stars);
    if recursive {
        collector.collect_recursive_possible_grids();
    } else {
        collector.collect_possible_grids();
    }
    // Liste des invariants dans la région pour toutes les grilles possibles
    let mut invariants = Variant::check_for_invariants(handler, grid, &collector.possible_grids);
    // Qu'on complète avec les cases autour des régions qui sont toujours adjacentes à une étoile dans la
    // région pour toutes les grilles possibles (et qui ne sont pas déjà présentes dans les invariants)
    let star_adjacents =
        StarAdjacent::check_for_star_adjacents(handler, grid, &collector.possible_grids);
    for action in star_adjacents {
        if !invariants.contains(&action) {
            invariants.push(action);
        }
    }
    (invariants, collector.possible_grids.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::GridParser;
    use crate::LineColumn;

    // Construction d'un objet GridHandler et d'un Grid à partir d'une grille de test
    fn get_test_grid() -> (GridHandler, Grid) {
        let grid_parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid_handler = GridHandler::new(&grid_parser, 1);
        let grid = Grid::from(&grid_handler);
        (grid_handler, grid)
    }

    #[test]
    fn test_combinaisons_count() {
        let (grid_handler, mut grid) = get_test_grid();

        // La zone A contient 2 cases non définies => 2 combinaisons pour placer une étoile
        assert_eq!(
            combinaisons_count(&grid_handler, &grid, &GridSurfer::Region('A'), 1),
            2
        );

        // La ligne 0 contient 5 cases non définies => 5 x 4 = 20 combinaisons pour placer 2 étoiles
        assert_eq!(
            combinaisons_count(&grid_handler, &grid, &GridSurfer::Line(0), 2),
            20
        );

        // On place une étoile en (0, 0)
        grid.cell_mut(LineColumn::new(0, 0)).value = CellValue::Star;

        // La colonne 0 contient 1 étoiles et 4 cases non définies => 4 combinaisons pour placer 2 étoiles
        assert_eq!(
            combinaisons_count(&grid_handler, &grid, &GridSurfer::Column(0), 2),
            4
        );
    }
}
