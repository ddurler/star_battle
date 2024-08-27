//! Règle de construction/résolution d'une grille.
//!
//! Recherche les combinaisons d'étoiles possibles dans une région.
//! Plus simplement que `rule_region_star_complete`, on n'examine ici que le contenu des différentes
//! combinaisons dans une région sans examiner l'impact sur l'ensemble de la grille grille.
//! On intègre également dans cette recherche, toutes les cases environnant une région qui sont
//! forcément pas des étoles puisque toujours à proximité d'une étoile dans la région.
//! Les règles qui apparaissent ainsi sont plus compréhensible pour un humain.

use crate::check_bad_rules;
use crate::CellValue;
use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;

use super::invariant::Variant;
use super::star_adjacent::StarAdjacent;

/// Cherche toutes les combinaisons d'étoiles possibles dans les différentes régions.
/// Version simplifiée de `rule_complete_star_number` qui se limite au contenu des différentes
/// régions pour une compréhension plus aisées pour un humain
pub fn rule_region_stars(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    // Examine toutes les différentes régions
    for region in handler.regions() {
        let zone = GridSurfer::Region(region);
        let invariant_actions = try_star_complete(handler, grid, &zone, handler.nb_stars());
        if !invariant_actions.is_empty() {
            return Some(GoodRule::InvariantWithZone(zone, invariant_actions));
        }
    }
    None
}

/// Vérifie si la règle est applicable sur la région définie
fn try_star_complete(
    handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
) -> Vec<GridAction> {
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
    invariants
}

/// Structure pour la recherche des combinaisons possibles qui positionnent
/// le nombre attendu d'étoiles dans une region.<br>
///
/// On utilise ici la 'force brute' pour tester toutes les façons de poser les étoiles manquantes
/// dans la région.
///
/// S'il y a n étoiles à placer (n > 0) dans les m cases non définies d'une région,
/// on explore tous les nombres de 1 à 2**m -1 qui ont n bits à 1 et on positionne des étoiles
/// dans tous les i-eme cases si me i-eme bit est 1.
/// Si la grille obtenue est 'viable', on la retient comme combinaison possible.
/// On examine ensuite les grilles retenues à la recherche de cases invariantes.
struct Collector<'a> {
    /// Handler de la grille à étudier
    handler: &'a GridHandler,

    /// Contenu de la grille à étudier
    grid: &'a Grid,

    /// Liste des cases de la région à étudier
    region: &'a Vec<LineColumn>,

    /// Nombre d'étoiles à placer dans la zone
    nb_stars: usize,

    /// Liste des combinaisons de grilles possibles pour placer le nombre d'étoiles demandés dans la zone
    possible_grids: Vec<Grid>,
}

impl<'a> Collector<'a> {
    /// Constructeur d'une zone à examiner
    pub const fn new(
        handler: &'a GridHandler,
        grid: &'a Grid,
        region: &'a Vec<LineColumn>,
        nb_stars: usize,
    ) -> Self {
        Self {
            handler,
            grid,
            region,
            nb_stars,
            possible_grids: Vec::new(),
        }
    }

    /// Cherche les combinaisons possibles qui positionnent le nombre attendu d'étoiles dans la région
    pub fn collect_possible_grids(&mut self) {
        let mut cur_nb_stars = 0; // Nombre d'étoiles déjà placées dans la région
        let mut cur_nb_unknown = 0; // Nombre de cases non définies dans la grille
        let mut cur_line_column_unknown = Vec::new(); // Coordonnées des cases non définies dans la région
        for line_column in self.region {
            match self.grid.cell(*line_column).value {
                CellValue::Star => cur_nb_stars += 1,
                CellValue::NoStar => (),
                CellValue::Unknown => {
                    cur_nb_unknown += 1;
                    cur_line_column_unknown.push(*line_column);
                }
            }
        }

        if cur_nb_stars >= self.nb_stars {
            // Toutes les étoiles sont placées dans la région.
            // Rien à explorer dans cette région
            return;
        }

        // Nombre d'étoiles qui restent à placer dans la région
        let nb_to_do_star = self.nb_stars - cur_nb_stars;

        assert!(
            nb_to_do_star <= cur_nb_unknown,
            "Situation inattendue lors de l'examen de la région !"
        );

        // Boucle sur toutes les façons de poser `nb_to_do_star` étoiles dans les
        // `cur_nb_unknown` cases non définies.
        for combinaison in 1..usize::pow(
            2,
            u32::try_from(cur_nb_unknown).expect("Région trop grande (32 cases inconnues max) !"),
        ) {
            // On a besoin d'autant de bits à 1 dans combinaison qu'on d'étoiles à placer
            if count_ones(combinaison) == nb_to_do_star {
                // On crée un nouvelle grille possible avec toutes les étoiles positionnées dans la région
                let mut new_grid = self.grid.clone();
                for (i, line_column) in cur_line_column_unknown.iter().enumerate() {
                    new_grid.cell_mut(*line_column).value = {
                        if combinaison & (1 << i) == 0 {
                            CellValue::NoStar
                        } else {
                            CellValue::Star
                        }
                    }
                }

                // Si cette nouvelle grille est viable... on l'ajoute à la liste des grilles possibles
                if check_bad_rules(self.handler, &new_grid).is_ok() {
                    self.possible_grids.push(new_grid);
                }
            }
        }
    }
}

/// Compte le nombre de bits à 1 dans un usize
const fn count_ones(n: usize) -> usize {
    let mut count = 0;
    let mut num = n;

    while num > 0 {
        count += num & 1; // Ajoute 1 si le bit de poids faible est 1
        num >>= 1; // Décale num vers la droite
    }

    count
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
        let option_good_rule = rule_region_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());

        // Cette règle s'applique sur l'avant dernière ligne de 'DDDDD' : On doit mettre une étoile
        // sur cette ligne donc les D sur la ligne suivante ne peuvent pas être une étoile...
        let option_good_rule = rule_region_stars(&grid_handler, &grid);
        assert!(option_good_rule.is_some());
        grid.apply_good_rule(&option_good_rule.unwrap());
    }
}
