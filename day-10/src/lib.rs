use std::i32;

use tracing::trace;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Directions {
    North,
    East,
    South,
    West,
}

fn directions_from_symbol(c: char) -> Vec<Directions> {
    match c {
        '|' => vec![Directions::North, Directions::South],
        '-' => vec![Directions::East, Directions::West],
        'J' => vec![Directions::North, Directions::West],
        'L' => vec![Directions::North, Directions::East],
        '7' => vec![Directions::South, Directions::West],
        'F' => vec![Directions::South, Directions::East],
        'S' => vec![
            Directions::North,
            Directions::East,
            Directions::South,
            Directions::West,
        ],
        _ => vec![],
    }
}

#[tracing::instrument(skip(maze))]
fn find_start_position(maze: &[Vec<char>]) -> Option<Position> {
    for (y, row) in maze.iter().enumerate() {
        match row.iter().position(|c| *c == 'S') {
            Some(x) => return Some(Position { x, y }),
            None => continue,
        };
    }

    None
}

fn parse_maze(inp: Vec<String>) -> Vec<Vec<char>> {
    inp.iter()
        .map(|r| {
            let o = r.as_bytes();
            let o: Vec<char> = o.iter().map(|c| *c as char).collect();
            o
        })
        .collect()
}

#[tracing::instrument]
fn is_connected(a: Vec<Directions>, a_pos: Position, b: Vec<Directions>, b_pos: Position) -> bool {
    for dir in a {
        if dir == Directions::North
            && b.contains(&Directions::South)
            && a_pos.x == b_pos.x
            && a_pos.y > b_pos.y
        {
            trace!("n 2 s");
            return true;
        }

        if dir == Directions::South
            && b.contains(&Directions::North)
            && a_pos.x == b_pos.x
            && a_pos.y < b_pos.y
        {
            trace!("s 2 n");
            return true;
        }

        if dir == Directions::West
            && b.contains(&Directions::East)
            && a_pos.y == b_pos.y
            && a_pos.x > b_pos.x
        {
            trace!("w 2 e");
            return true;
        }

        if dir == Directions::East
            && b.contains(&Directions::West)
            && a_pos.y == b_pos.y
            && a_pos.x < b_pos.x
        {
            trace!("e 2 w");
            return true;
        }
    }

    return false;
}

fn position_is_connected(
    maze: &[Vec<char>],
    pos: Position,
    offset: Offset,
    prev: Position,
) -> Option<Position> {
    let directions = directions_from_symbol(maze[pos.y][pos.x]);

    let pos_x = pos.x as i32 + offset.x;
    let pos_y = pos.y as i32 + offset.y;

    if pos_y < 0 || pos_y >= maze.len() as i32 {
        return None;
    }

    let pos_y = pos_y as usize;

    if pos_x < 0 || pos_x > maze[pos_y].len() as i32 {
        return None;
    }

    let pos_x = pos_x as usize;

    if pos_x == prev.x && pos_y == prev.y {
        return None;
    }

    trace!(
        "{} {} {:?}",
        pos_x,
        pos_y,
        directions_from_symbol(maze[pos_y][pos_x])
    );
    let candidate_dirs = directions_from_symbol(maze[pos_y][pos_x]);

    if !is_connected(
        directions.clone(),
        pos.clone(),
        candidate_dirs,
        Position { x: pos_x, y: pos_y },
    ) {
        return None;
    }

    Some(Position { x: pos_x, y: pos_y })
}

fn adjacent_connected_pipes(maze: &[Vec<char>], pos: Position, prev: Position) -> Option<Position> {
    let around = vec![
        Offset { x: 0, y: 1 },
        Offset { x: -1, y: 0 },
        Offset { x: 0, y: -1 },
        Offset { x: 1, y: 0 },
    ];

    for offset in around {
        match position_is_connected(maze, pos.clone(), offset, prev.clone()) {
            Some(p) => return Some(p),
            None => continue,
        }
    }

    None
}

#[tracing::instrument(skip_all)]
fn find_loop_length(maze: &[Vec<char>], start_pos: Position) -> usize {
    let mut prev = start_pos.clone();
    let mut player = start_pos;
    let mut counter = 0;
    loop {
        let new_player = match adjacent_connected_pipes(maze, player.clone(), prev.clone()) {
            Some(p) => p.clone(),
            None => todo!("no further way"),
        };

        prev = player;
        player = new_player;

        trace!(?player);

        counter += 1;

        if maze[player.y][player.x] == 'S' {
            break;
        }
    }

    trace!(counter);
    counter
}

pub fn is_char_with_side(c: char) -> bool {
    "F|7S".contains(c)
}

#[tracing::instrument(skip(main_loop))]
pub fn find_tile_count_in_row(row: &str, y: usize, main_loop: &[Position]) -> usize {
    let mut inside_loop = false;

    let count = row
        .chars()
        .enumerate()
        .filter(|(x, c)| {
            if main_loop.contains(&Position { x: *x, y }) {
                if is_char_with_side(*c) {
                    inside_loop = !inside_loop;
                }

                return false;
            }

            inside_loop
        })
        .count();

    trace!("{}", count);

    count
}

pub fn find_main_loop(maze: &[Vec<char>], start_pos: Position) -> Vec<Position> {
    let mut prev = start_pos.clone();
    let mut player = start_pos.clone();
    let mut output = vec![start_pos];
    loop {
        let new_player = match adjacent_connected_pipes(maze, player.clone(), prev.clone()) {
            Some(p) => p.clone(),
            None => todo!("no further way"),
        };

        prev = player;
        player = new_player;

        output.push(player.clone());

        if maze[player.y][player.x] == 'S' {
            break;
        }
    }

    output
}

fn get_char_for_start_pos(maze: &[Vec<char>], pos: Position) -> char {
    let around = vec![
        (Offset { x: 0, y: 1 }, Directions::South),
        (Offset { x: -1, y: 0 }, Directions::West),
        (Offset { x: 0, y: -1 }, Directions::North),
        (Offset { x: 1, y: 0 }, Directions::East),
    ];

    let mut con_dirs = vec![];
    for (offset, con_dir) in around {
        match position_is_connected(maze, pos.clone(), offset, Position { x: 0, y: 0 }) {
            Some(_) => con_dirs.push(con_dir),
            None => continue,
        }
    }

    match con_dirs.as_slice() {
        [Directions::North, Directions::South] => '|',
        [Directions::East, Directions::West] => '-',
        [Directions::North, Directions::West] => 'J',
        [Directions::North, Directions::East] => 'L',
        [Directions::South, Directions::West] => '7',
        [Directions::South, Directions::East] => 'F',
        &[] | &[_] | &[_, ..] => 'S',
    }
}

#[tracing::instrument(skip_all)]
pub fn part_one(inp: Vec<String>) -> usize {
    let maze = parse_maze(inp);
    let maze = maze.as_slice();

    let start_pos = match find_start_position(maze) {
        Some(p) => p,
        None => todo!("no start position"),
    };

    let loop_length = find_loop_length(maze, start_pos);

    if loop_length % 2 == 0 {
        return loop_length / 2;
    }

    (loop_length / 2) + 1
}

pub fn part_two(inp: Vec<String>) -> usize {
    let maze = parse_maze(inp.clone());
    let maze = maze.as_slice();

    let start_pos = match find_start_position(maze) {
        Some(p) => p,
        None => todo!("no start position"),
    };

    let main_loop = find_main_loop(maze, start_pos.clone());
    let start_symbol = get_char_for_start_pos(maze, start_pos);

    inp.iter()
        .map(|row| {
            if row.contains('S') {
                return row.replace('S', &start_symbol.to_string());
            }
            row.to_string()
        })
        .enumerate()
        .map(|(y, row)| find_tile_count_in_row(&row, y, &main_loop))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_get_char_for_start_pos() {
        let maze = parse_maze(vec![
            "..........".to_string(),
            ".S------7.".to_string(),
            ".|F----7|.".to_string(),
            ".||....||.".to_string(),
            ".||....||.".to_string(),
            ".|L-7F-J|.".to_string(),
            ".|..||..|.".to_string(),
            ".L--JL--J.".to_string(),
            "..........".to_string(),
        ]);
        let maze = maze.as_slice();
        let start_pos = Position { x: 1, y: 1 };

        assert_eq!(get_char_for_start_pos(maze, start_pos), 'F');
    }

    #[test_log::test]
    pub fn test_find_tile_count_in_row() {
        let maze = parse_maze(vec![
            "..........".to_string(),
            ".S------7.".to_string(),
            ".|F----7|.".to_string(),
            ".||....||.".to_string(),
            ".||....||.".to_string(),
            ".|L-7F-J|.".to_string(),
            ".|..||..|.".to_string(),
            ".L--JL--J.".to_string(),
            "..........".to_string(),
        ]);
        let maze = maze.as_slice();

        let main_loop = find_main_loop(maze, Position { x: 1, y: 1 });
        let main_loop = main_loop.as_slice();

        assert_eq!(find_tile_count_in_row("..........", 0, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".S------7.", 1, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".|F----7|.", 2, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".||....||.", 3, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".||....||.", 4, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".|L-7F-J|.", 5, main_loop), 0);
        assert_eq!(find_tile_count_in_row(".|..||..|.", 6, main_loop), 4);
        assert_eq!(find_tile_count_in_row(".L--JL--J.", 7, main_loop), 0);
        assert_eq!(find_tile_count_in_row("..........", 8, main_loop), 0);
    }

    #[test_log::test]
    pub fn test_adjacent_connected_pipes() {
        let maze = &[
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        let res = adjacent_connected_pipes(maze, Position { x: 1, y: 1 }, Position { x: 0, y: 0 });

        assert_eq!(res, Some(Position { x: 1, y: 2 }));
    }

    #[test_log::test]
    pub fn test_find_start_position() {
        let pos = find_start_position(&[
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ]);

        assert_eq!(pos, Some(Position { x: 1, y: 1 }));
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 4);

        let input = vec![
            "..F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ...".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 8);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec![
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        ];

        assert_eq!(part_two(input), 4);

        let input = vec![
            ".F----7F7F7F7F-7....".to_string(),
            ".|F--7||||||||FJ....".to_string(),
            ".||.FJ||||||||L7....".to_string(),
            "FJL7L7LJLJ||LJ.L-7..".to_string(),
            "L--J.L7...LJS7F-7L7.".to_string(),
            "....F-J..F7FJ|L7L7L7".to_string(),
            "....L7.F7||L7|.L7L7|".to_string(),
            ".....|FJLJ|FJ|F7|.LJ".to_string(),
            "....FJL-7.||.||||...".to_string(),
            "....L---J.LJ.LJLJ...".to_string(),
        ];

        assert_eq!(part_two(input), 8);

        let input = vec![
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        ];

        assert_eq!(part_two(input), 10);
    }
}
