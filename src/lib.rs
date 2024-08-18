/*!
Star Battle Solver

Star Battle est un puzzle logique avec des règles simples et des solutions stimulantes.

Les règles de Star Battle sont simples :
Vous devez placer des étoiles sur la grille selon ces règles :

* 2 étoiles ne peuvent pas être adjacentes horizontalement, verticalement ou en diagonale.
* Pour les puzzles 1★, vous devez placer 1 étoile sur chaque ligne, colonne et bloc.
* Pour les puzzles 2★, les étoiles par ligne, colonne et bloc doivent être 2, etc.
* Il existe également des puzzles 3★.

## Sites internet

* [Site en français](https://fr.puzzle-star-battle.com/)
* [Vidéo en français](https://www.youtube.com/watch?v=dG-xkOYYkwY)
* [Site en anglais](https://starbattle.puzzlebaron.com/)

## [`Region`]

[`Region`] est la zone dans laquelle se trouve une étoile. C'est un `char`.

## [`GridParser`]

[`GridParser`] construit une grille depuis une formalisation textuelle d'une grille à résoudre'.

Le constructeur est une forme de `TryFrom` pour l'un des types suivants :

* `TryFrom<&Vec<String>> for Parser`
* `TryFrom<Vec<String>> for Parser`
* `TryFrom<&[String]> for Parser`
* `TryFrom<Vec<&str>> for `
* `TryFrom<&str> for Parser`

Chaque ligne du texte (ou chaque élément du vecteur) correspond à une ligne de la grille.<br>
Les différentes zones sont identifiées par des caractères distincts dans les cases correspondantes.<br>
Les espaces ou séparateurs équivalents (e.g. TAB) sont ignorés.<br>
Les lignes 'vides' ou qui débutent par l'un des caractères suivants sont ignorées : '*', '#', '/'
(considérés comme d'éventuels commentaires).<br>

```rust
use star_battle::GridParser;
assert!(GridParser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").is_ok());
```

## [`LineColumn`]

[`LineColumn`] repère une case dans la grille par ses coordonnées (`line`, `column`) base 0.

```rust
use star_battle::LineColumn;
let lc = LineColumn::new(0, 0);
assert_eq!(lc.line(), 0);
assert_eq!(lc.column(), 0);
```

## [`CellValue`]

[`CellValue`] définit une valeur possible d'une case de la grille parmi:

* `Unknown` : Contenu inconnu de la case (valeur par défaut)
* `Star` : La case contient une étoile
* `NoStar` : La case ne contient pas d'étoile

```rust
use star_battle::CellValue;
assert_eq!(CellValue::default(), CellValue::Unknown);
```

## [`CellValue`]

[`CellValue`] décrit une case de la grille parsée par [`GridParser`]:

* `line_column`: [`LineColumn`] de la case dans la grille (base 0)
* `region`: [`Region`] de la case
* `value`: [`CellValue`] de la case. Par défaut, `CellValue::Unknown`.

```rust
use star_battle::{GridParser, LineColumn, CellValue};
let parser = GridParser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").unwrap();
assert_eq!(parser.cell(&LineColumn::new(0, 0)).unwrap().region, 'A');
assert_eq!(parser.cell(&LineColumn::new(0, 0)).unwrap().value, CellValue::Unknown);
```

## [`GridHandler`]

[`GridHandler`] définit les caractéristiques d'une grille à résoudre:

* `nb_lines`: nombre de lignes de la grille
* `nb_columns`: nombre de colonnes de la grille
* `nb_stars`: nombre d'étoiles à placer dans chaque ligne, colonne et région de la grille
* `regions`: liste des régions de la grille (par ordre alphabétique)
* `cell_region`: région d'une case de la grille

Les contenus des cases de la grille ne sont pas définis dans la structure [`GridHandler`].

```rust
use star_battle::{GridParser, GridHandler, LineColumn};
let parser = GridParser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").unwrap();
let grid = GridHandler::new(&parser, 1);
assert_eq!(grid.nb_lines(), 5);
assert_eq!(grid.nb_columns(), 5);
assert_eq!(grid.nb_stars(), 1);
assert_eq!(grid.regions(), vec!['A', 'B', 'C', 'D', 'E']);
assert_eq!(grid.cell_region(&LineColumn::new(0, 0)), 'A');
```

*/

/// Une région est identifiée par un caractère.
pub type Region = char;

// Modules
mod cell_value;
mod grid_cell;
mod grid_handler;
mod grid_parser;
mod grid_parser_checker;
mod line_column;

// Internal
use grid_parser_checker::GridParserChecker;

// Exported
pub use cell_value::CellValue;
pub use grid_cell::GridCell;
pub use grid_handler::GridHandler;
pub use grid_parser::GridParser;
pub use line_column::LineColumn;
