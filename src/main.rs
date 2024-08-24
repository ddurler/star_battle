//! Star Battle Solver

use std::env;
use std::fs::File;
use std::io::Read;

use star_battle::get_good_rule;
use star_battle::Grid;
use star_battle::GridHandler;
use star_battle::GridParser;

/// Message d'aide pour l'utilisateur
const HELP_MESSAGE: &str = "
STAR BATTLE Usage: ./star-battle <grille>

<grille> est le nom d'un fichier contenant une grille à résoudre.

Un fichier contenant une grille à résoudre est, par exemple :

# Ligne de commentaire
ABBBB
ABBBB
CCBBB
DDDDD
DEEED
";

fn main() {
    // Nom du fichier contenant la grille à résoudre en paramètre
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() == 2 {
        &args[1]
    } else {
        println!("{HELP_MESSAGE}");
        return;
    };

    // Demande d'aide ?
    if ["-h", "--help", "aide"].contains(&file_name.to_lowercase().as_str()) {
        println!("{HELP_MESSAGE}");
        return;
    }

    // Traitement du contenu du fichier
    match read_lines(file_name) {
        Ok(lines) => match GridParser::try_from(&lines) {
            Ok(grid_parsed) => solve(&grid_parsed, 1),

            Err(e) => {
                println!("Erreur dans le fichier {file_name}: {e}");
            }
        },
        Err(e) => println!("Erreur dans le fichier {file_name}: {e}"),
    }
}

fn solve(grid_parsed: &GridParser, nb_stars: usize) {
    let grid_handler = GridHandler::new(grid_parsed, nb_stars);
    let mut grid = Grid::from(&grid_handler);

    println!("Grid:\n{}", grid_handler.display(&grid, true));
    loop {
        match get_good_rule(&grid_handler, &grid) {
            Ok(option_good_rule) => {
                if option_good_rule.is_some() {
                    let good_rule = option_good_rule.unwrap();
                    println!("{good_rule}");
                    grid.apply_good_rule(&good_rule);
                    println!("\n{}", grid_handler.display(&grid, true));
                } else {
                    break;
                }
            }
            Err(bad_rule) => {
                println!("{bad_rule} !!!");
                break;
            }
        }
    }

    if grid_handler.is_done(&grid) {
        println!("Grille résolue !\n");
    } else {
        println!("Grille non résolue :(\n");
    }
}

fn read_lines(filename: &str) -> Result<Vec<String>, String> {
    // Ouverture du fichier
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(format!("Erreur ouverture du fichier {filename}: {e}")),
    };
    // Lecture du fichier
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => {}
        Err(e) => return Err(format!("Erreur lecture du fichier {filename}: {e}")),
    }

    // Extraction des lignes du fichier
    let lines: Vec<String> = file_contents
        .split('\n')
        .map(|s: &str| s.to_string())
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // Liste de fichiers de tests avec des grilles à résoudre
        let test_files = vec!["./test_grids/test01.txt"];

        for test_file in test_files {
            let lines = read_lines(test_file).unwrap();
            let grid_parsed = GridParser::try_from(&lines).unwrap();
            let grid_handler = GridHandler::new(&grid_parsed, 1);
            let grid = Grid::from(&grid_handler);
            println!("Grid: \n{grid}");
        }
    }
}
