use std::fs;

#[derive(Debug, Clone, Default)]
struct Tile {
    id: i64,
    arr: [Vec<Vec<char>>; 8],
    cached_borders: [[i32; 4]; 8], // N, E, S, W
    orientation: usize,
    selected: bool,
}

const W: usize = 10;

fn rotated_cw(src: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = src.len();
    let mut dst: Vec<Vec<char>> = vec![vec!['.'; size]; size];
    for y in 0..size {
        for x in 0..size {
            dst[y][x] = src[size - x - 1][y];
        }
    }
    dst
}

fn flipped_v(src: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = src.len();
    let mut dst: Vec<Vec<char>> = vec![vec!['.'; size]; size];
    for y in 0..size {
        for x in 0..size {
            dst[y][x] = src[y][size - x - 1];
        }
    }
    dst
}

impl Tile {
    fn new(id: i64, lines: Vec<&str>) -> Self {
        let mut tile = Tile::default();
        tile.id = id;

        tile.arr[0] = lines
            .iter()
            .map(|s| s.chars().map(|c| c).collect())
            .collect();

        tile.arr[4] = flipped_v(&tile.arr[0]);
        for i in 1..4 {
            tile.arr[i] = rotated_cw(&tile.arr[i - 1]);
            tile.arr[i + 4] = rotated_cw(&tile.arr[i + 4 - 1]);
        }

        for o in 0..8 {
            for i in 0..W {
                if tile.arr[o][0][i] == '#' {
                    tile.cached_borders[o][0] += 1 << i;
                }
                if tile.arr[o][i][W - 1] == '#' {
                    tile.cached_borders[o][1] += 1 << i;
                }
                if tile.arr[o][W - 1][i] == '#' {
                    tile.cached_borders[o][2] += 1 << i;
                }
                if tile.arr[o][i][0] == '#' {
                    tile.cached_borders[o][3] += 1 << i;
                }
            }
        }

        tile
    }

    fn get_border(self: &Self, side: usize) -> i32 {
        self.cached_borders[self.orientation][side]
    }
}

#[derive(Debug, Clone, Default)]
struct Puzzle {
    tiles: Vec<Tile>,
    grid: Vec<Vec<Option<usize>>>,
}

impl Puzzle {
    fn check_border(self: &Self, pos: (usize, usize), dir: (i32, i32), tile_id: usize) -> bool {
        let tile = &self.tiles[tile_id];
        let size = self.grid.len() as i32;
        let pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if pos.0 >= 0 && pos.0 < size && pos.1 >= 0 && pos.1 < size {
            if let Some(index) = self.grid[pos.0 as usize][pos.1 as usize] {
                return match dir {
                    (1, 0) => self.tiles[index].get_border(0) == tile.get_border(2),
                    (-1, 0) => self.tiles[index].get_border(2) == tile.get_border(0),
                    (0, 1) => self.tiles[index].get_border(3) == tile.get_border(1),
                    (0, -1) => self.tiles[index].get_border(1) == tile.get_border(3),
                    _ => false,
                };
            }
        }

        true
    }

    fn get_id(self: &Self, pos: (usize, usize)) -> i64 {
        self.tiles[self.grid[pos.1][pos.0].unwrap()].id
    }

    fn solve(self: &mut Self, pos: (usize, usize)) -> bool {
        for i in 0..self.tiles.len() {
            if !self.tiles[i].selected {
                for o in 0..8 {
                    self.tiles[i].orientation = o;

                    if self.check_border(pos, (1, 0), i)
                        && self.check_border(pos, (-1, 0), i)
                        && self.check_border(pos, (0, 1), i)
                        && self.check_border(pos, (0, -1), i)
                    {
                        self.tiles[i].selected = true;
                        self.grid[pos.0][pos.1] = Some(i);

                        let mut next_pos = (pos.0 + 1, pos.1);
                        if next_pos.0 == self.grid.len() {
                            next_pos = (0, next_pos.1 + 1);
                        }
                        if next_pos.1 == self.grid.len() {
                            return true;
                        }

                        if self.solve(next_pos) {
                            return true;
                        }

                        self.grid[pos.0][pos.1] = None;
                        self.tiles[i].selected = false;
                    }
                }
            }
        }

        false
    }

    fn build_image(self: &Self) -> Vec<Vec<char>> {
        let grid_w = self.grid.len();
        let image_w = (W - 2) * grid_w;
        let mut image = vec![vec![' '; image_w]; image_w];

        for grid_x in 0..grid_w {
            for grid_y in 0..grid_w {
                let tile_id = self.grid[grid_y][grid_x].unwrap();
                let tile = &self.tiles[tile_id];
                for x in 0..(W - 2) {
                    for y in 0..(W - 2) {
                        image[y + (grid_y * (W - 2))][x + (grid_x * (W - 2))] =
                            tile.arr[tile.orientation][y + 1][x + 1]
                    }
                }
            }
        }

        image
    }
}

#[derive(Debug, Clone, Default)]
struct SeaMonster {
    arr: Vec<Vec<char>>,
    size: (usize, usize),
    marks: Vec<(usize, usize)>,
}

impl SeaMonster {
    fn new(pattern: &Vec<&str>) -> Self {
        let mut m = SeaMonster::default();
        m.size = (pattern[0].len(), pattern.len());
        m.marks = Vec::new();
        for (y, s) in pattern.iter().enumerate() {
            for (x, c) in s.chars().enumerate() {
                if c == '#' {
                    m.marks.push((x, y));
                }
            }
        }
        m
    }

    fn eat(self: &Self, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut image = image.clone();
        let mut new_image = image.clone();

        for o in 0..8 {
            let mut found_any = false;
            for y in 0..image.len() - self.size.1 {
                for x in 0..image[0].len() - self.size.0 {
                    let mut check = true;
                    for m in self.marks.iter() {
                        if image[y + m.1][x + m.0] != '#' {
                            check = false;
                            break;
                        }
                    }

                    if check {
                        found_any = true;
                        for m in self.marks.iter() {
                            new_image[y + m.1][x + m.0] = '.';
                        }
                    }
                }
            }

            if found_any {
                return new_image;
            }

            image = rotated_cw(&image);
            if o == 4 {
                image = flipped_v(&image);
            }
            new_image = image.clone();
        }

        new_image
    }
}

fn main() {
    let content = fs::read_to_string("input/20.txt").expect("Input file not found.");

    let mut puzzle = Puzzle::default();

    for tile in content.split("\n\n") {
        let mut it = tile.lines();
        let id = it
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let lines: Vec<&str> = it.collect();

        puzzle.tiles.push(Tile::new(id, lines));
    }

    let grid_size = (puzzle.tiles.len() as f32).sqrt() as usize;

    puzzle.grid = vec![vec![None; grid_size]; grid_size];
    puzzle.solve((0, 0));

    println!(
        "part 0: {}",
        puzzle.get_id((0, 0))
            * puzzle.get_id((grid_size - 1, 0))
            * puzzle.get_id((0, grid_size - 1))
            * puzzle.get_id((grid_size - 1, grid_size - 1))
    );

    let image = puzzle.build_image();

    let monster = SeaMonster::new(&vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]);
    let new_image = monster.eat(&image);
    let ans = new_image.iter().flatten().filter(|&&x| x == '#').count();
    println!("part 1: {}", ans);

    //ans 20913499394191
}
