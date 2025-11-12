use rand::{rng, Rng};
use std::collections::HashSet;

const MAZE_WIDTH: usize = 20;
const MAZE_HEIGHT: usize = 15;
const CELL_SIZE: f32 = 40.0;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Cell {
    Wall,
    Path,
}

pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
impl Maze {
    pub fn new() -> Self {
        let mut maze = Self {
            grid: vec![vec![Cell::Wall; MAZE_WIDTH]; MAZE_HEIGHT],
        };
        maze.generate();
        maze
    }

    fn generate(&mut self) {
        let mut rng = rng();
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        let start = (1, 1);
        self.grid[start.1][start.0] = Cell::Path;
        visited.insert(start);
        stack.push(start);

        while let Some(current) = stack.last().cloned() {
            let neighbors = self.get_unvisited_neighbors(current, &visited);
            if !neighbors.is_empty() {
                let next = neighbors[rng.random_range(0..neighbors.len())];
                self.remove_wall(current, next);
                self.grid[next.1][next.0] = Cell::Path;
                visited.insert(next);
                stack.push(next);
            } else {
                stack.pop();
            }
        }
    }

    fn get_unvisited_neighbors(&self, (x, y): (usize, usize), visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(0, -2), (0, 2), (-2, 0), (2, 0)]; // Up, Down, Left, Right

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < MAZE_WIDTH as isize && ny >= 0 && ny < MAZE_HEIGHT as isize {
                let npos = (nx as usize, ny as usize);
                if !visited.contains(&npos) {
                    neighbors.push(npos);
                }
            }
        }
        neighbors
    }

    fn remove_wall(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let wx = ((x1 + x2) / 2) as usize;
        let wy = ((y1 + y2) / 2) as usize;
        self.grid[wy][wx] = Cell::Path;
    }

    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                if self.grid[y][x] == Cell::Wall {
                    let rect = ggez::graphics::Rect::new(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                    let mesh = ggez::graphics::Mesh::new_rectangle(
                        ctx,
                        ggez::graphics::DrawMode::fill(),
                        rect,
                        ggez::graphics::Color::BLACK,
                    )?;
                    canvas.draw(&mesh, ggez::graphics::DrawParam::default());
                }
            }
        }
        Ok(())
    }

    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;
        if grid_x >= MAZE_WIDTH || grid_y >= MAZE_HEIGHT {
            true
        } else {
            self.grid[grid_y][grid_x] == Cell::Wall
        }
    }
}