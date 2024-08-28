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

/// Énumération des différentes zones examinées
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ZoneToExamine {
    Region,
    LineAndColumn,
    MultipleLinesAndColumns(usize),
}

/// Cherche toutes les combinaisons possibles dans les différentes régions.
/// Version simplifiée de `rule_zone_recursive_possible_stars` qui se limite au contenu des différentes
/// régions pour une compréhension plus aisées pour un humain
pub fn rule_region_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_possible_stars(handler, grid, ZoneToExamine::Region)
}

/// Cherche toutes les combinaisons possibles qui positionnent le nombre attendu d'étoiles
/// dans les différentes ligne ou colonne.
/// Pour chaque zone, examine ensuite l'ensemble des grilles après avoir placer toutes les étoiles pour
/// déterminer si le contenu d'une case est commun à toutes ces combinaisons possibles.
pub fn rule_line_column_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_possible_stars(handler, grid, ZoneToExamine::LineAndColumn)
}

/// Cherche toutes les combinaisons possibles qui positionnent le nombre attendu d'étoiles
/// dans différentes groupes de lignes consécutives ou groupes de colonnes consécutive.
/// Pour chaque zone, examine ensuite l'ensemble des grilles après avoir placer toutes les étoiles pour
/// déterminer si le contenu d'une case est commun à toutes ces combinaisons possibles.
pub fn rule_multi_lines_columns_recursive_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
) -> Option<GoodRule> {
    rule_possible_stars(handler, grid, ZoneToExamine::MultipleLinesAndColumns(2))
}

/// Méthode générique qui cherche toutes les combinaisons possibles dans les différentes zones ou régions
fn rule_possible_stars(
    handler: &GridHandler,
    grid: &Grid,
    zone_to_examine: ZoneToExamine,
) -> Option<GoodRule> {
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

    // Examine toutes les zones prévues
    for (zone, nb_stars, _) in zones {
        let invariant_actions = try_star_complete(handler, grid, &zone, nb_stars);
        if !invariant_actions.is_empty() {
            return Some(GoodRule::InvariantWithZone(zone, invariant_actions));
        }
    }
    None
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

/// Vérifie si la règle est applicable sur une zone définie
fn try_star_complete(
    handler: &GridHandler,
    grid: &Grid,
    grid_surfer: &GridSurfer,
    nb_stars: usize,
) -> Vec<GridAction> {
    let surfer = handler.surfer(grid, grid_surfer);
    let mut collector = Collector::new(handler, grid, &surfer, nb_stars);
    collector.collect_recursive_possible_grids();
    Variant::check_for_invariants(handler, grid, &collector.possible_grids)
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

    #[test]
    fn test_complete_start_number() {
        // La grille de test peut être complètement résolue avec cette seule règle sur les zones
        let (grid_handler, mut grid) = get_test_grid();

        println!("Grille initiale :\n{}", grid_handler.display(&grid, true));

        loop {
            let option_good_rule = rule_line_column_recursive_possible_stars(&grid_handler, &grid);
            if option_good_rule.is_some() {
                let good_rule = option_good_rule.unwrap();
                println!("{good_rule}");
                grid.apply_good_rule(&good_rule);

                println!("\n{}", grid_handler.display(&grid, true));
            } else {
                break;
            }
        }

        assert!(grid_handler.is_done(&grid));
    }

    #[test]
    fn test_moyen01_2() {
        // Test extrait d'une étape de résolution de la grille "./test_grid/moyen01_2.txt"
        let grid_text = "
# Exemple de grille 2★
# Bataille d'étoiles sur Android
AABBBCCCC
AAABBCCCC
AAABBCCCC
ADDEEEDCF
ADDDDDDFF
DDDDDGGGF
HDHHDFGGF
HHHHIFFFF
HHHIIIIIF
        ";

        let grid_parser = GridParser::try_from(grid_text).unwrap();
        let grid_handler = GridHandler::new(&grid_parser, 2);
        let mut grid = Grid::from(&grid_handler);

        // Etape particulière de la résolution
        grid.cell_mut(LineColumn::new(0, 1)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(0, 2)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(0, 3)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(0, 5)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(1, 1)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(1, 2)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(1, 3)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(1, 5)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(2, 2)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(2, 3)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(2, 4)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(2, 5)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(2, 6)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(3, 2)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(3, 3)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(3, 4)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(3, 5)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(3, 6)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(4, 2)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(4, 3)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(4, 4)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(4, 5)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(4, 6)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(5, 4)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(5, 5)).value = CellValue::Star;
        grid.cell_mut(LineColumn::new(5, 6)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(5, 8)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(6, 4)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(6, 5)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(6, 6)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(6, 8)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(7, 3)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(7, 6)).value = CellValue::NoStar;
        grid.cell_mut(LineColumn::new(7, 7)).value = CellValue::NoStar;

        grid.cell_mut(LineColumn::new(8, 5)).value = CellValue::NoStar;

        println!("Grille:\n{}", grid_handler.display(&grid, true));

        // En étudiant les possibilités de la première ligne, LineColumn(0, 7) ne peut être que NoStar
        // (Si on met une étoile dans cette case, on ne peut pas placer les 2 étoiles dans la colonne 6)
        let grid_surfer = GridSurfer::Line(0);
        let vec_actions = try_star_complete(&grid_handler, &grid, &grid_surfer, 2);
        assert!(!vec_actions.is_empty());
    }
}
