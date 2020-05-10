pub enum CellType { Empty, Wall }

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub units_per_cell: u32,
    cells: Vec<CellType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            width: 10,
            height: 10,
            units_per_cell: 60,
            cells: vec![
                CellType::Wall, CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Empty, CellType::Wall, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall, CellType::Wall, CellType::Wall, CellType::Wall, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Empty, CellType::Wall,
                CellType::Wall, CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,  CellType::Wall,
            ]
        }
    }

    pub fn cell_at(&self, row: usize, col: usize) -> &CellType {
        if row >= self.height || col >= self.width {
            panic!("Cell ({}, {}) is not valid for map with height {} and width {}", row, col, self.height, self.width);
        }

        let index = row * self.height + col;
        self.cells.get(index).unwrap()
    }
}