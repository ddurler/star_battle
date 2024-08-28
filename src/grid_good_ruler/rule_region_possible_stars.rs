//! Règle de construction/résolution d'une grille.
//!
//! Recherche les combinaisons d'étoiles possibles dans une région.
//! Plus simplement que `rule_region_star_complete`, on n'examine ici que le contenu des différentes
//! combinaisons dans une région sans examiner l'impact sur l'ensemble de la grille grille.
//! On intègre également dans cette recherche, toutes les cases environnant une région qui sont
//! forcément pas des étoles puisque toujours à proximité d'une étoile dans la région.
//! Les règles qui apparaissent ainsi sont plus compréhensible pour un humain.

use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;

use super::collector::Collector;
use super::invariant::Variant;
use super::star_adjacent::StarAdjacent;

/// Cherche toutes les combinaisons d'étoiles possibles dans les différentes régions.
/// Version simplifiée de `rule_region_recursive_possible_stars` qui se limite au contenu des
/// différentes régions pour une compréhension plus aisées pour un humain
pub fn rule_region_possible_stars(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    // Pour simplifier la règle présentée à un humain, on retient la région qui génère un minimum
    // de grilles pour placer toutes les étoiles
    #[derive(Debug, Default)]
    struct BestCollector {
        grid_surfer: Option<GridSurfer>,
        nb_possible_grids: usize,
        invariant_actions: Vec<GridAction>,
    }
    let mut best_collector = BestCollector::default();
    // Examine toutes les différentes régions
    for region in handler.regions() {
        let grid_surfer = GridSurfer::Region(region);
        let (invariant_actions, nb_possible_grids) =
            try_star_complete(handler, grid, &grid_surfer, handler.nb_stars());
        if !invariant_actions.is_empty()
        // La règle s'applique pour cette région...
            && (best_collector.grid_surfer.is_none()
            // ... et c'est la première région qui permet d'appliquer la règle...
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

/// Vérifie si la règle est applicable sur la région définie.<br>
/// Si applicable, retourne la liste des actions déduites par la règle et le nombre de grilles possibles
/// qui ont été examinées pour ces actions
fn try_star_complete(
    handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
) -> (Vec<GridAction>, usize) {
    let surfer = handler.surfer(grid, grid_surfer);
    let mut collector = Collector::new(handler, grid, &surfer, nb_stars);
    collector.collect_possible_grids();
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

    // Construction d'un objet GridHandler et d'un Grid à partir d'une grille de test
    fn get_test_grid() -> (GridHandler, Grid) {
        let grid_parser =
            GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
        let grid_handler = GridHandler::new(&grid_parser, 1);
        let grid = Grid::from(&grid_handler);
        (grid_handler, grid)
    }

    #[test]
    fn test_region_stars() {
        let (grid_handler, mut grid) = get_test_grid();

        println!("Grille initiale :\n{}", grid_handler.display(&grid, true));

        // Cette règle s'applique sur la région 'CC' dans la 3eme ligne : Les cases adjacentes ne peuvent
        // pas être une étoile...
        let option_good_rule = rule_region_possible_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());

        // Cette règle s'applique sur l'avant dernière ligne de 'DDDDD' : On doit mettre une étoile
        // sur cette ligne donc les D sur la ligne suivante ne peuvent pas être une étoile...
        let option_good_rule = rule_region_possible_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());
    }
}
