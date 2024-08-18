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

## [`Parser`]

[`Parser`] permet la construction d'une grille depuis une formalisation textuelle de l'énoncé.

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
use star_battle::Parser;
assert!(Parser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").is_ok());
```

## [`LineColumn`]

[`LineColumn`] permet de repérer une case dans la grille par ses coordonnées (`line`, `column`) base 0.

```rust
use star_battle::LineColumn;
let lc = LineColumn::new(0, 0);
assert_eq!(lc.line(), 0);
assert_eq!(lc.column(), 0);
```

## [`Value`]

[`Value`] permet de définir une valeur possible d'une case de la grille parmi:

* `Unknown` : Contenu inconnu de la case (valeur par défaut)
* `Star` : La case contient une étoile
* `NoStar` : La case ne contient pas d'étoile

```rust
use star_battle::Value;
assert_eq!(Value::default(), Value::Unknown);
```

## [`Cell`]

[`Cell`] permet de décrire une case de la grille parsée par [`Parser`]:

* `line_column`: ligne et colonne de la case dans la grille (base 0)
* `region`: région de la case (caractère)
* `value`: valeur contenue dans la case

```rust
use star_battle::{Parser, LineColumn, Value};
let parser = Parser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").unwrap();
assert_eq!(parser.cell(&LineColumn::new(0, 0)).unwrap().region, 'A');
assert_eq!(parser.cell(&LineColumn::new(0, 0)).unwrap().value, Value::Unknown);
```

*/

// Modules
mod cell;
mod checker;
mod line_column;
mod parser;
mod value;

// Internal
use checker::Checker;

// Exported
pub use cell::Cell;
pub use line_column::LineColumn;
pub use parser::Parser;
pub use value::Value;
