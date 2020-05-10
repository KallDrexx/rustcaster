pub enum CellType { Empty, Wall }

#[derive(Eq, PartialEq)]
pub enum SpawnType {Player}

enum ParsedCell {
    Environmental(CellType),
    Spawn(SpawnType)
}

pub struct SpawnLocation {
    pub entity: SpawnType,
    pub row: usize,
    pub col: usize,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub units_per_cell: u32,
    pub spawns: Vec<SpawnLocation>,
    cells: Vec<CellType>,
}

static DEFAULT_MAP: &str = "
xxxxxxxxxx
x        x
x        x
x        x
xxxxx xxxx
x @ x x  x
x   x x  x
x   x x  x
x xxx    x
x        x
x        x
xxxxxxxxxx
";

impl Map {
    pub fn new() -> Self {
        string_map_to_map(DEFAULT_MAP)
    }

    pub fn cell_at(&self, row: usize, col: usize) -> &CellType {
        if row >= self.height || col >= self.width {
            panic!("Cell ({}, {}) is not valid for map with height {} and width {}", row, col, self.height, self.width);
        }

        let index = row * self.width + col;

        self.cells.get(index).unwrap()
    }
}

fn string_map_to_map(map: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut cells = Vec::new();
    let mut spawns = Vec::new();

    let mut current_row = 0;
    let mut current_col;
    for line in map.lines() {
        if line.is_empty() {
            continue;
        }

        if width == 0 {
            width = line.len();
        } else if width != line.len() {
            panic!("Different lines have different widths.  Line of '{}' doesn't match previous line width of {}", line, width);
        }

        current_col = 0;
        for character in line.chars() {
            let parsed_cell = char_to_cell_type(character);
            match parsed_cell {
                ParsedCell::Environmental(cell_type) => cells.push(cell_type),
                ParsedCell::Spawn(spawn_type) => {
                    cells.push(CellType::Empty);
                    spawns.push(SpawnLocation {
                        row: current_row,
                        col: current_col,
                        entity: spawn_type,
                    });
                }
            }

            current_col += 1;
        }

        height += 1;
        current_row += 1;
    }

    Map {
        width, height, spawns, cells,
        units_per_cell: 20,
    }
}

fn char_to_cell_type(character: char) -> ParsedCell {
    match character {
        ' ' => ParsedCell::Environmental(CellType::Empty),
        'x' => ParsedCell::Environmental(CellType::Wall),
        '@' => ParsedCell::Spawn(SpawnType::Player),
        _ => panic!("No known type of cell type for '{}'", character),
    }
}