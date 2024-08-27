/*!
Star Battle Solver

Star Battle est un puzzle logique avec des règles simples et des solutions stimulantes.

Les règles de Star Battle sont simples :
Vous devez placer des étoiles sur la grille selon ces règles :

* 2 étoiles ne peuvent pas être adjacentes horizontalement, verticalement ou en diagonale.
* Pour les puzzles 1★, vous devez placer 1 étoile sur chaque ligne, colonne et région.
* Pour les puzzles 2★, les étoiles par ligne, colonne et région doivent être 2, etc.
* Il existe également des puzzles 3★.

## Sites internet

* [Site en français](https://fr.puzzle-star-battle.com/)
* [Vidéo en français](https://www.youtube.com/watch?v=dG-xkOYYkwY)
* [Site en anglais](https://starbattle.puzzlebaron.com/)

## [`Region`]

[`Region`] est une zone de cases dans laquelle il faut également placer une étoile (ou le nombre d'étoiles à placer).
Pour ce crate, c'est identifié par un [`char`] issu de la formalisation textuelle reconnue par le [`GridParser`].

## [`GridParser`]

[`GridParser`] construit une grille depuis une formalisation textuelle d'une grille à résoudre.

Le constructeur est une forme de [`TryFrom`] pour l'un des types suivants :

* `TryFrom<&Vec<String>> for Parser`
* `TryFrom<Vec<String>> for Parser`
* `TryFrom<&[String]> for Parser`
* `TryFrom<Vec<&str>> for `
* `TryFrom<&str> for Parser`

Chaque ligne du texte (ou chaque élément du vecteur) correspond à une ligne de la grille.<br>
Les différentes régions de la grille sont identifiées par des caractères distincts dans les cases correspondantes.<br>
Les espaces ou séparateurs équivalents (e.g. TAB) sont ignorés.<br>
Les lignes 'vides' ou qui débutent par l'un des caractères suivants sont ignorées : '*', '#' ou '/'
(considérés comme d'éventuels commentaires).<br>

```rust
use star_battle::GridParser;

// Représentation textuelle d'une grille de 5 lignes et 5 colonnes contenant 5 régions
// distinctes repérées par les lettres 'A', 'B', 'C', 'D' et 'E'.
assert!(GridParser::try_from("
    ABBBB
    ABBBB
    CCBBB
    DDDDD
    DEEED
").is_ok());
```

Le [`GridParser`] est utilisé pour définir la grille initiale. La cohérence de la grille est vérifiée:

* Syntaxe correcte dans le texte descriptif de la grille
* Nombre cohérent de colonnes dans chaque ligne
* Régions connexes dans la grille

## [`LineColumn`]

[`LineColumn`] repère une case dans la grille par ses coordonnées (`line`, `column`) base 0.

Lorsque les coordonnées d'une case sont affichées (`Display`), les colonnes sont référencées par les lettres
'A', 'B', ... et les lignes par des chiffres'1', '2'.<br>
La case (0, 0) en haut et à gauche de la grille correspond donc avec 'A1'.

```rust
use star_battle::LineColumn;

let line_column = LineColumn::new(0, 1);

assert_eq!(line_column.line(), 0);
assert_eq!(line_column.column(), 1);
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

## [`GridCell`]

[`GridCell`] décrit une case de la grille parsée par [`GridParser`] ou gérée par [`Grid`]:

* `line_column`: [`LineColumn`] de la case dans la grille (base 0)
* `region`: [`Region`] de la case
* `value`: [`CellValue`] de la case. Par défaut, `CellValue::Unknown`.

```rust
use star_battle::{GridParser, LineColumn, CellValue};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();

assert_eq!(grid_parser.cell(LineColumn::new(0, 0)).unwrap().region, 'A');
assert_eq!(grid_parser.cell(LineColumn::new(0, 0)).unwrap().value, CellValue::Unknown);
```

## [`GridHandler`]

[`GridHandler`] définit les caractéristiques d'une grille à résoudre:

* `nb_lines`: nombre de lignes de la grille
* `nb_columns`: nombre de colonnes de la grille
* `nb_stars`: nombre d'étoiles à placer dans chaque ligne, colonne et région de la grille
* `regions`: liste des régions de la grille (par ordre de taille croissante)
* `cell_region`: région d'une case de la grille

Les contenus des cases de la grille ne sont pas définis dans la structure [`GridHandler`].<br>
C'est la structure [`Grid`] qui représente le contenu des cases de la grille.

Initialement, le [`GridHandler`] est construite à partir d'un [`GridParser`].

```rust
use star_battle::{GridParser, GridHandler, LineColumn};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid = GridHandler::new(&grid_parser, 1);

assert_eq!(grid.nb_lines(), 5);
assert_eq!(grid.nb_columns(), 5);
assert_eq!(grid.nb_stars(), 1);
assert_eq!(grid.regions().len(), 5);
assert_eq!(grid.cell_region(LineColumn::new(0, 0)), 'A');
```

## [`Grid`]

[`Grid`] est la structure avec le contenu des cases de la grille.

Cette structure est utilisée pour la resolution du jeu. Elle est allégée des informations détenues par la
structure associée [`GridHandler`]; Ce qui permet d'examiner des évolutions de la grille avec un minimum
d'occupation de mémoire.

Initialement, la [`Grid`] est construite à partir d'un [`GridHandler`].

```rust
use star_battle::{GridParser, GridHandler, Grid};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let grid = Grid::from(&grid_handler);

assert_eq!(grid.nb_lines(), 5);
assert_eq!(grid.nb_columns(), 5);
```

On peut ainsi utiliser la structure [`Grid`] pour résoudre le jeu en clonant cette structure et en
postulant sur la valeur des cases de la grille pour évaluer les possibilités.

La fonction [`GridHandler::is_done`] retourne `true` si toutes les cases de la grille ont une valeur définie.

```rust
use star_battle::{GridParser, GridHandler, Grid, LineColumn, CellValue};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let grid = Grid::from(&grid_handler);

let mut grid_cloned = grid.clone();
let line_column = LineColumn::new(0, 0);
grid_cloned.cell_mut(line_column).value = CellValue::Star;
assert_eq!(grid.cell(line_column).value, CellValue::Unknown);
assert_eq!(grid_cloned.cell(line_column).value, CellValue::Star);
```

## [`GridSurfer`]

[`GridSurfer`] est une  énumération qui permet de naviguer sur les case de la grille qui répondre à certains
critères à travers la grille.

 Cette énumération est applicable sur un objet [`GridHandler`] associé à une grille définie par un [`Grid`].

 On peut ainsi parcourir les cases de la grille suivant les critères suivants:

* Toutes les cases de la grille
* Toutes les cases d'une region
* Toutes les cases adjacentes à une case donnée (y compris les diagonales)
* Toutes les cases d'une ligne
* Toutes les cases d'une colonne
* Toutes les cases de plusieurs lignes consécutives
* Toutes les cases de plusieurs colonnes consécutives

```rust
use star_battle::{GridParser, GridHandler, Grid, LineColumn, GridSurfer};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let grid = Grid::from(&grid_handler);

// Liste des cases d'une région
let grid_surfer = grid_handler.surfer(&grid, &GridSurfer::Region('A'));
assert_eq!(grid_surfer, vec![LineColumn::new(0, 0), LineColumn::new(1, 0)]);
```

## [`BadRuleError`]

[`BadRuleError`] identifie une situation qui invalide le contenu d'une grille.

La fonction [`check_bad_rules`] permet de vérifier qu'une une grille est valide ou non.

Les situations invalidant étant :

* 2 cases adjacentes contenant chacune une étoile
* Une 'zone' qui contient trop d'étoiles
* Une 'zone' dans laquelle il n'est pas possible de placer suffisamment d'étoiles

Ici une 'zone' étant :

* Une [`Region`]
* Une ligne de la grille
* Une colonne de la grille

```rust
use star_battle::{GridParser, GridHandler, Grid, check_bad_rules};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let grid = Grid::from(&grid_handler);

assert!(check_bad_rules(&grid_handler, &grid).is_ok());
```

# [`GridAction`]

[`GridAction`] représente une action possible sur une case de la grille :

* Placer une étoile
* Indiquer qu'une étoile n'est possible dans cette case
* Indiquer que le contenu d'une case est inconnu

Ces actions sont liées au contenu possible d'une case de la grille défini par un [`CellValue`].

[`GridAction`] implémente la méthode [`GridAction::apply_action`] qui permet d'appliquer une action sur une [`Grid`].

Symétriquement, le module [`Grid`] implémente la méthode [`Grid::apply_action`] qui permet d'appliquer une
de ces actions à une case de la grille.

```rust
use star_battle::{GridParser, GridHandler, Grid, CellValue, GridAction, LineColumn};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let mut grid = Grid::from(&grid_handler);

grid.apply_action(&GridAction::SetStar(LineColumn::new(1, 1)));
assert_eq!(grid.cell(LineColumn::new(1, 1)).value, CellValue::Star);

GridAction::SetNoStar(LineColumn::new(1, 1)).apply_action(&mut grid);
assert_eq!(grid.cell(LineColumn::new(1, 1)).value, CellValue::NoStar);
```

# [`GoodRule`]

[`GoodRule`] identifie les règles qui permettent d'avancer dans la construction/résolution d"une grille :

* `NoStarAdjacentToStar(LineColumn, Vec<GridAction>)`:  Indique les cases adjacentes à une étoile qui ne peuvent
   pas contenir une étoile et indique les actions à effectuer pour les définir
* `ZoneNoStarCompleted`: Indique les cases restantes dans une zone ne peuvent pas être des étoiles
* `ZoneStarCompleted`: Indique les cases restantes dans une zone sont forcement des étoiles
* `InvariantWithZone(GridSurfer, Vec<GridAction>)`: Indique que quelle que soit la façon de placer les étoiles
   dans une zone, des cases n'ont toujours qu'une seule et même possibilité

La fonction [`get_good_rule`] recherche une règle [`GoodRule`] applicable à une grille.<br>
Cette fonction retourne une erreur [`BadRuleError`] si la grille n'est pas valide.<br>
Sinon un `Option<GoodRule>` est retourné. None signifie alors qu'aucune règle permettant d'avancer dans la
construction de la grille n'a été trouvée.

Les règles examinées (et dans cet ordre) sont :

* Une case non définie et adjacente à une étoile ne peut pas être une étoile
* Toutes les cases non définies dans une 'zone' (région, ligne ou colonne) qui possède déjà toutes ces étoiles
  sont des cases qui ne peuvent pas contenir une étoile
* S'il reste autant de cases non définies dans une 'zone' (région, ligne ou colonne) que d'étoiles manquantes
  dans cette 'zone' alors ce sont forcément des étoiles
* Toutes les combinaisons possibles pour positioner une étoile dans une région ont des cases toujours avec une
  étoile ou jamais une étoile
* Des case autour d'une région sont toujours adjacente à une étoile pour toutes les combinaisons possibles d'étoiles
  dans cette région. Ces cases ne peuvent donc pas être des étoiles
* Toutes les combinaisons possibles pour positionner une étoile dans une 'zone' (région, ligne ou colonne) ont des
  cases toujours avec une étoile ou jamais une étoile dans toutes les grilles possibles pour ces combinaisons

```rust
use star_battle::{GridParser, GridHandler, Grid, get_good_rule};

let grid_parser = GridParser::try_from(vec!["ABBBB", "ABBBB", "CCBBB", "DDDDD", "DEEED"]).unwrap();
let grid_handler = GridHandler::new(&grid_parser, 1);
let mut grid = Grid::from(&grid_handler);

let ok_good_rule = get_good_rule(&grid_handler, &grid);
assert!(ok_good_rule.is_ok());
let some_good_rule = ok_good_rule.unwrap();
assert!(some_good_rule.is_some());
let good_rule = some_good_rule.unwrap();
grid.apply_good_rule(&good_rule);
```

*/

/// Une région est identifiée par un caractère.
pub type Region = char;

// Modules
mod cell_value;
mod grid;
mod grid_action;
mod grid_bad_ruler;
mod grid_cell;
mod grid_good_ruler;
mod grid_handler;
mod grid_parser;
mod grid_parser_checker;
mod grid_surfer;
mod line_column;

// Internal
use grid_parser_checker::GridParserChecker;
use line_column::{display_column, display_line};

// Exported
pub use cell_value::CellValue;
pub use grid::Grid;
pub use grid_action::GridAction;
pub use grid_bad_ruler::{check_bad_rules, BadRuleError};
pub use grid_cell::GridCell;
pub use grid_good_ruler::{get_good_rule, GoodRule};
pub use grid_handler::GridHandler;
pub use grid_parser::GridParser;
pub use grid_surfer::GridSurfer;
pub use line_column::LineColumn;
