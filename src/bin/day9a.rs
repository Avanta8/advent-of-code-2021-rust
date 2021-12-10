use std::io::Read;

fn parse_input() -> Vec<Vec<u8>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split('\n')
        .map(|row| row.bytes().map(|x| x - 48).collect())
        .collect()
}

fn main() {
    let grid = parse_input();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0i32;

    for y in 0..height {
        for x in 0..width {
            let low = [(0, -1), (-1, 0), (1, 0), (0, 1)]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        Some(grid[ny as usize][nx as usize])
                    } else {
                        None
                    }
                })
                .min()
                .unwrap();
            if low > grid[y][x] {
                count += grid[y][x] as i32 + 1;
            }
        }
    }

    println!("{}", count);
}
