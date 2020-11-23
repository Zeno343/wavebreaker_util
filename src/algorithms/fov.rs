use crate::data_structures::Graph;

pub trait FovNode: 'static { 
    fn blocks_view(&self) -> bool;
}

pub fn compute_fov<N: FovNode>(origin: (usize, usize), map: &dyn Graph<(usize, usize), N>, range: usize) -> Vec<(usize, usize)> {
    static QUADRANTS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];
    let mut revealed_tiles = vec![origin];

    for &direction in &QUADRANTS {
        let quadrant = Quadrant { direction, origin };
        let mut first_row = Row { depth: 1, start_slope: -1.0, end_slope: 1.0 };
        revealed_tiles.extend(scan(&mut first_row, &quadrant, map));
    }

    revealed_tiles
        .iter()
        .filter(|&&point| is_in_range(point, origin, range)) 
        .map(|point| *point)
        .collect()
}

fn is_in_range((x, y): (usize, usize), (o_x, o_y): (usize, usize), range: usize) -> bool {
    let x_dist = if x > o_x { x - o_x } else { o_x - x }; 
    let y_dist = if y > o_y { y - o_y } else { o_y - y };

    (x_dist << 1) + (y_dist << 1) < range << 1
}

fn scan<N: FovNode>(row: &mut Row, quadrant: &Quadrant, map: &dyn Graph<(usize, usize), N>) -> Vec<(usize, usize)> {
    let mut previous_tile: Option<(i64, i64)> = None;
    let mut visible_tiles = Vec::new();

    for tile in row.tiles() {
        let abs_tile = quadrant.transform(tile);
        if map.contains(&abs_tile) {
            if map[abs_tile].blocks_view()
                || !map[abs_tile].blocks_view()
                || is_symmetric(&row, tile) 
            {
                visible_tiles.push(abs_tile);
            }

            if let Some(prev_tile) = previous_tile {
                let prev_abs_tile = quadrant.transform(prev_tile);

                if map[prev_abs_tile].blocks_view() && !map[abs_tile].blocks_view() {
                    row.start_slope = slope(tile);
                } else if !map[prev_abs_tile].blocks_view() && map[abs_tile].blocks_view() {
                    let mut next_row = row.next();
                    next_row.end_slope = slope(tile);

                    visible_tiles.extend(scan(&mut next_row, &quadrant, map));
                }
            }

            previous_tile = Some(tile)
        }
    }

    if let Some(prev_tile) = previous_tile {
        let prev_abs_tile = quadrant.transform(prev_tile);

        if !map[prev_abs_tile].blocks_view() {
            visible_tiles.extend(scan(&mut row.next(), &quadrant, map));
        }
    }

    visible_tiles
}

fn slope((col, row): (i64, i64)) -> f64 {
    (2 * col - 1) as f64 / (2 * row) as f64
}

fn is_symmetric(row: &Row, (col, _): (i64, i64)) -> bool {
    (col as f64) >= row.depth as f64 * row.start_slope 
        && (col as f64) <= row.depth as f64 * row.end_slope
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Quadrant {
    direction: Direction,
    origin: (usize, usize),
}

impl Quadrant {
    fn transform(&self, point: (i64, i64)) -> (usize, usize) {
        let (col, row) = point;
        let (o_col, o_row) = self.origin;

        match self.direction {
            Direction::North => {
                ((o_col as i64 + col) as usize, (o_row as i64 - row) as usize)
            }
            Direction::East => {
                ((o_col as i64 + row) as usize, (o_row as i64 + col) as usize)
            }
            Direction::South => {
                ((o_col as i64 + col) as usize, (o_row as i64 + row) as usize)
            }
            Direction::West => {
                ((o_col as i64 - row) as usize, (o_row as i64 + col) as usize)
            }
        }
    }
}

struct Row {
    depth: i64,
    start_slope: f64,
    end_slope: f64,
}

impl Row {
    fn next(&self) -> Row {
        Row {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }

    fn tiles(&self) -> Vec<(i64, i64)> {
        let min_col = (self.depth as f64 * self.start_slope + 0.5).floor() as i64;
        let max_col = (self.depth as f64 * self.end_slope - 0.5).ceil() as i64; 

        (min_col ..= max_col)
            .map(|col| (col, self.depth as i64))
            .collect()
    }
}

