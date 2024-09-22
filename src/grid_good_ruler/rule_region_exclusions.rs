//! Règle de construction/résolution d'une grille.
//!
//! Recherche des combinaisons de 'n' lignes ou colonnes qui ne sont occupées que 'n' régions.<br>
//! Dans ce cas, toutes les autres cases dans ces 'n' régions qui ne sont pas dans les 'n' lignes ou colonnes
//! ne peuvent pas être des étoiles.
//!
//! S'il existe déjà des étoiles dans ces 'n' lignes ou colonnes, La règle n'est pas applicable.
//!
//! S'il existe des cases d'autres régions qui sont déjà définies comme pouvant pas être des étoiles,
//! on peut les ignorer.
//!
//! En effect, les 'n' lignes ou colonnes positionnent toutes les étoiles pour ces 'n' régions
//! et il ne peut donc pas y avoir d'autres étoiles dans ces 'n' régions.
//!
//! Par exemple, pour n = 1, si une seule région occupe toute une ligne (ou colonne) alors les étoiles de
//! cette région sont sur cette ligne (ou colonne) et les cases des cette région qui ne sont pas dans cette
//! ligne (ou colonne) ne peuvent pas être des étoiles.
//!
//! Cette règle est l'opposée de la règle [`rule_region_combinations`]

use crate::CellValue;
use crate::GoodRule;
use crate::Grid;
use crate::GridAction;
use crate::GridHandler;
use crate::GridSurfer;
use crate::LineColumn;
use crate::Region;

/// Recherche les régions de 1 ligne ou 1 colonne. Les autres cases de cette ligne ou colonne
/// ne peuvent pas être des étoiles
pub fn rule_region_1_exclusions(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_exclusions(handler, grid, 1)
}

/// Recherche les couples de régions sur 2 ligne ou 2 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_2_exclusions(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_exclusions(handler, grid, 2)
}

/// Recherche les triplets de régions sur 3 ligne ou 3 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_3_exclusions(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_exclusions(handler, grid, 3)
}

/// Recherche les quadruplets de régions sur 4 ligne ou 4 colonne. Les autres cases de ces lignes ou colonnes
/// ne peuvent pas être des étoiles
pub fn rule_region_4_exclusions(handler: &GridHandler, grid: &Grid) -> Option<GoodRule> {
    rule_region_generic_exclusions(handler, grid, 4)
}

/// Cherche les combinaisons de 'n' lignes ou colonnes qui contiennent exactement 'n' régions.<br>
/// S'il existe des cases appartement à ces régions dans d'autres lignes ou colonnes, elles ne peuvent
/// pas être des étoiles
#[allow(clippy::range_minus_one)]
fn rule_region_generic_exclusions(
    handler: &GridHandler,
    grid: &Grid,
    n: usize,
) -> Option<GoodRule> {
    for line in 0..=handler.nb_lines() - n {
        let grid_surfer = GridSurfer::Lines(line..=line + n - 1);
        if let Some((vec_regions, candidates)) =
            rule_region_more_generic_exclusions(handler, grid, n, &grid_surfer)
        {
            let mut actions = Vec::new();
            for line_column in candidates {
                actions.push(GridAction::SetNoStar(line_column));
            }
            return Some(GoodRule::ZoneExclusions(vec_regions, grid_surfer, actions));
        }
    }
    for column in 0..=handler.nb_columns() - n {
        let grid_surfer = GridSurfer::Columns(column..=column + n - 1);
        if let Some((vec_regions, candidates)) =
            rule_region_more_generic_exclusions(handler, grid, n, &grid_surfer)
        {
            let mut actions = Vec::new();
            for line_column in candidates {
                actions.push(GridAction::SetNoStar(line_column));
            }
            return Some(GoodRule::ZoneExclusions(vec_regions, grid_surfer, actions));
        }
    }
    None
}

/// Spécialisation de `rule_region_generic_exclusions` pour 'n' lignes ou 'n' colonnes.<br>
/// Compte combien de régions différentes sont présentes dans le `grid_surfer`. Si 'n' régions alors
/// recherche des cases candidates qui ne sont pas définies pour ces régions en dehors de `grid_surfer`
fn rule_region_more_generic_exclusions(
    handler: &GridHandler,
    grid: &Grid,
    n: usize,
    grid_surfer: &GridSurfer,
) -> Option<(Vec<Region>, Vec<LineColumn>)> {
    let surfer = handler.surfer(grid, grid_surfer);
    let mut vec_regions = Vec::new();
    for line_column in &surfer {
        match grid.cell(*line_column).value {
            // S'il existe déjà des étoiles dans les n lignes ou colonnes, on abandonne la recherche
            // (la règle n'est pas applicable)
            CellValue::Star => return None,
            // Si la case est déjà définie comme ne pouvant pas être une étoile, on l'ignore : Quelle que soit
            // sa région, la règle reste applicable
            CellValue::NoStar => continue,
            // Case non définie, on comptabilise sa région
            CellValue::Unknown => {
                let region = grid.cell(*line_column).region;
                if !vec_regions.contains(&region) {
                    vec_regions.push(region);
                    if vec_regions.len() > n {
                        return None;
                    }
                }
            }
        }
    }
    // vec_regions contient toutes les regions qui sont dans le 'grid_surfer' et il n'y a pas plus de 'n'.
    // On cherche des cases non définies de ces régions qui ne sont pas dans 'grid_surfer'
    let mut candidates = Vec::new();
    for line_column in handler.surfer(grid, &GridSurfer::AllCells) {
        if !surfer.contains(&line_column) {
            let cell = grid.cell(line_column);
            if cell.is_unknown() & vec_regions.contains(&cell.region) {
                candidates.push(line_column);
            }
        }
    }

    if candidates.is_empty() {
        None
    } else {
        Some((vec_regions, candidates))
    }
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
    fn test_region_exclusions() {
        let (grid_handler, mut grid) = get_test_grid();

        // Au moins la 4eme ligne 'DDDDD' déclenche cette règle
        let option_good_rule = rule_region_1_exclusions(&grid_handler, &grid);
        assert!(&option_good_rule.is_some());
        let good_rule = option_good_rule.unwrap();
        grid.apply_good_rule(&good_rule);

        // println!("Rule: {}", &good_rule);
        // println!("Grid :\n{}", grid_handler.display(&grid, true));
        // panic!("stop test")
    }
}
