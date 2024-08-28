//! Examine toutes les possibilités pour poser les étoiles manquantes dans une zone et recherche
//! si des cases sont invariantes pour toutes ces possibilités.<br>

use crate::check_bad_rules;
use crate::CellValue;
use crate::Grid;
use crate::GridHandler;
use crate::LineColumn;

/// Structure pour la recherche des combinaisons possibles qui positionnent
/// le nombre attendu d'étoiles dans une zone.<br>
///
/// Une zone est ici une région, une ligne, une colonne ou un groupe de lignes ou de colonnes.
///
/// Cette structure propose 2 méthodes pour la recherche des grilles possibles :
///
/// * `collect_possible_grids` : Recherche les combinaisons possibles dans la zone uniquement
/// * `collect_recursive_possible_grids` : Recherche les combinaison de manière récursive en
///   examinant les autres cases des grilles possibles
///
/// Pour cela, cette structure `Collector` s'utilise comme suit :
///
/// - On détermine la zone à examiner pour cette règle. C'est un vecteur de `LineColumn` issu d'un `GridSurfer`
/// - On construit un `collector` pour cette zone `Collector::new(handler, grid, zone, nb_stars)`
/// - On appelle la méthode `collect_possible_grids` ou `collect_recursive_possible_grids` pour rechercher toutes
///   les grilles possibles pour cette zone
///
/// Ensuite, la fonction `Variant::check_for_invariants` permet examiner les différentes grilles possibles
/// pour en extraire d'éventuelles cases invariantes dans toutes les combinaisons
pub struct Collector<'a> {
    /// Handler de la grille à étudier
    handler: &'a GridHandler,

    /// Contenu de la grille à étudier
    grid: &'a Grid,

    /// Liste des cases de la zone à étudier
    zone: &'a Vec<LineColumn>,

    /// Nombre d'étoiles à placer dans la zone
    nb_stars: usize,

    /// Liste des combinaisons de grilles possibles pour placer le nombre d'étoiles demandés dans la zone
    pub possible_grids: Vec<Grid>,
}

impl<'a> Collector<'a> {
    /// Constructeur d'une zone à examiner
    pub const fn new(
        handler: &'a GridHandler,
        grid: &'a Grid,
        zone: &'a Vec<LineColumn>,
        nb_stars: usize,
    ) -> Self {
        Self {
            handler,
            grid,
            zone,
            nb_stars,
            possible_grids: Vec::new(),
        }
    }

    /// Cherche les combinaisons possibles qui positionnent le nombre attendu d'étoiles dans la zone.
    ///
    /// On utilise ici la 'force brute' pour tester toutes les façons de poser les étoiles manquantes
    /// dans la zone.
    ///
    /// S'il y a n étoiles à placer (n > 0) dans les m cases non définies d'une zone,
    /// on explore tous les nombres de 1 à 2**m -1 qui ont n bits à 1 et on positionne des étoiles
    /// dans tous les i-eme cases si me i-eme bit est 1.
    /// Si la grille obtenue est 'viable', on la retient comme combinaison possible.
    pub fn collect_possible_grids(&mut self) {
        let mut cur_nb_stars = 0; // Nombre d'étoiles déjà placées dans la région
        let mut cur_nb_unknown = 0; // Nombre de cases non définies dans la grille
        let mut cur_line_column_unknown = Vec::new(); // Coordonnées des cases non définies dans la région
        for line_column in self.zone {
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

    /// Cherche récursivement les combinaisons possibles qui positionnent le nombre attendu d'étoiles dans la zone.
    ///
    /// L'algorithme de recherche 'récursif' avec un cheminement comme suit :
    ///
    /// - On repère la première case possible de la zone qui peut contenir une étoile
    /// - On pose une étoile dans cette case et on recherche les grilles possibles avec cette combinaison.
    ///   Cette recherche se fait en appelant à nouveau le même algorithme de recherche
    /// - Puis, on définit qu'il n'y a pas d'étoile dans cette case et on recherche à nouveau les grilles possibles
    ///   avec cette combinaison. Cette recherche se fait en appelant à nouveau le même algorithme de recherche
    /// - En final, toutes les grilles possibles collectées 'récursivement' sont des grilles possibles pour la zone
    pub fn collect_recursive_possible_grids(&mut self) {
        // Décompte du nombre d'étoiles qui restent à placer dans la zone
        let nb_current_stars = self
            .zone
            .iter()
            .filter(|line_column| self.grid.cell(**line_column).value == CellValue::Star)
            .count();

        if nb_current_stars == self.nb_stars {
            // Toutes les étoiles sont placées dans la zone
            // La grille courante est la seule possibilité dans ce cas...
            // On complète les cases non définies de cette zone par des cases sans étoile
            let mut new_grid = self.grid.clone();
            for line_column in self.zone {
                if new_grid.cell(*line_column).value == CellValue::Unknown {
                    new_grid.cell_mut(*line_column).value = CellValue::NoStar;
                }
            }
            self.possible_grids.push(new_grid);
            // ...qu'on retourne
            return;
        }

        // Au moins une étoile est à placer. On cherche la première case possible dans la zone pour cela
        if let Some(line_column) = self.first_possible_line_column_for_a_star() {
            // On construit alors une nouvelle grille possible
            // Et on pose une étoile dans cette case dans une nouvelle grille possible
            // et on invalide la possibilité d'une étoile pour toutes les cases adjacentes
            let mut new_grid = self.grid.clone();
            self.set_star(&mut new_grid, line_column);
            // Si cette nouvelle grille est viable...
            if check_bad_rules(self.handler, &new_grid).is_ok() {
                // ...on recherche les grilles possibles pour cette nouvelle grille
                let mut new_collector =
                    Collector::new(self.handler, &new_grid, self.zone, self.nb_stars);
                new_collector.collect_recursive_possible_grids();
                // Toutes les grilles trouvées par ce nouveau collector sont des grilles possibles pour la grille courante
                self.possible_grids.extend(new_collector.possible_grids);
            }

            //  Puis on construit une autre grille possible pour la zone sans une étoile dans cette case
            let mut new_grid = self.grid.clone();
            new_grid.cell_mut(line_column).value = CellValue::NoStar;
            // On recherche les grilles possibles pour cette nouvelle grille
            let mut new_collector =
                Collector::new(self.handler, &new_grid, self.zone, self.nb_stars);
            new_collector.collect_recursive_possible_grids();
            // Toutes les grilles trouvées par ce nouveau collector sont des grilles possibles pour la grille courante
            self.possible_grids.extend(new_collector.possible_grids);
        }

        // On retourne les grilles trouvées jusqu'ici
    }

    /// Recherche la première case possible pour poser une étoile dans la zone
    fn first_possible_line_column_for_a_star(&self) -> Option<LineColumn> {
        for line_column in self.zone {
            // Case possible pour poser une étoile ?
            if self.grid.cell(*line_column).is_unknown() {
                // Il ne faut pas d'étoiles dans les cases adjacentes à cette case
                if self
                    .handler
                    .adjacent_cells(*line_column)
                    .iter()
                    .filter(|line_column| self.grid.cell(**line_column).value == CellValue::Star)
                    .count()
                    == 0
                {
                    return Some(*line_column);
                }
            }
        }
        None
    }

    /// Pose une étoile sur une grille possible et indique que toutes les cases autour de cette étoile
    /// ne peuvent pas être une étoile
    fn set_star(&self, new_grid: &mut Grid, line_column: LineColumn) {
        // Pose une étoile dans cette case dans une nouvelle grille possible
        new_grid.cell_mut(line_column).value = CellValue::Star;
        // On indique que toutes les cases autour de cette étoile ne peuvent pas être une étoile
        for adjacent_line_column in self.handler.adjacent_cells(line_column) {
            match self.grid.cell(adjacent_line_column).value {
                CellValue::Star => panic!("Bug dans l'algo !!! La case {adjacent_line_column} ne devrait pas être une étoile"),
                CellValue::NoStar => (),
                CellValue::Unknown => new_grid.cell_mut(adjacent_line_column).value = CellValue::NoStar,
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
