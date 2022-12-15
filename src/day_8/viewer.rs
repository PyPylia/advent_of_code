pub struct Viewer {
    height_map: Vec<Vec<u8>>,
    visible_grid: Vec<Vec<bool>>,
}

impl Viewer {
    pub fn new(input: &str) -> Self {
        let mut height_map: Vec<Vec<u8>> = vec![];
        let mut visible_grid: Vec<Vec<bool>> = vec![];

        for line in input.lines() {
            let mut current_heights = vec![];
            let mut current_visible = vec![];

            for char in line.trim().chars() {
                current_visible.push(false);
                current_heights.push(char.to_digit(10).unwrap() as u8);
            }

            height_map.push(current_heights);
            visible_grid.push(current_visible);
        }

        Viewer {
            height_map: height_map,
            visible_grid: visible_grid,
        }
    }

    pub fn reset(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.visible_grid[y][x] = false;
            }
        }
    }

    pub fn count_visible(&self) -> u32 {
        let mut total_visible: u32 = 0;

        for row in &self.visible_grid {
            for tree in row {
                if *tree {
                    total_visible += 1;
                }
            }
        }

        total_visible
    }

    pub fn get_height(&self, x: usize, y: usize) -> u8 {
        self.height_map[y][x]
    }

    pub fn get_visible(&self, x: usize, y: usize) -> bool {
        self.visible_grid[y][x]
    }

    pub fn set_visible(&mut self, x: usize, y: usize) {
        self.visible_grid[y][x] = true;
    }

    pub fn height(&self) -> usize {
        self.height_map.len()
    }

    pub fn width(&self) -> usize {
        self.height_map[0].len()
    }
}
