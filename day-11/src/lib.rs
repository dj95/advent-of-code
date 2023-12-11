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

impl Position {
    pub fn distance(&self, other: &Self) -> usize {
        let x_dist = match self.x > other.x {
            true => self.x - other.x,
            false => other.x - self.x,
        };

        let y_dist = match self.y > other.y {
            true => self.y - other.y,
            false => other.y - self.y,
        };

        x_dist + y_dist
    }
}

fn get_empty_row_indice(space: Vec<String>) -> Vec<usize> {
    space
        .iter()
        .enumerate()
        .filter(|(_, r)| r.chars().all(|c| c == '.'))
        .map(|(i, _)| i)
        .collect()
}

fn get_empty_col_indice(space: Vec<String>) -> Vec<usize> {
    let col_count = space.first().unwrap().len();
    (0..col_count)
        .filter(|y| space.iter().all(|r| r.chars().nth(*y) == Some('.')))
        .collect()
}

fn get_expanded_position(
    x: usize,
    y: usize,
    empty_row_indice: &[usize],
    empty_col_indice: &[usize],
    factor: usize,
) -> Position {
    let x_dist = empty_col_indice.iter().filter(|c| *c < &x).count() * factor;

    let y_dist = empty_row_indice.iter().filter(|c| *c < &y).count() * factor;

    Position {
        x: x + x_dist,
        y: y + y_dist,
    }
}

pub fn find_galaxies_with_expansion(space: Vec<String>, factor: usize) -> Vec<Position> {
    let factor = (factor - 1).max(1);

    let empty_row_indice = get_empty_row_indice(space.clone());
    let empty_col_indice = get_empty_col_indice(space.clone());

    let galaxy_map: Vec<Vec<usize>> = space
        .iter()
        .map(|s| {
            s.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    trace!(?galaxy_map);

    galaxy_map
        .iter()
        .enumerate()
        .flat_map(|(y, x_positions)| {
            x_positions
                .iter()
                .map(|x| get_expanded_position(*x, y, &empty_row_indice, &empty_col_indice, factor))
                .collect::<Vec<Position>>()
        })
        .collect()
}

fn get_shortest_path_sum(inp: Vec<String>, factor: usize) -> usize {
    let mut galaxies = find_galaxies_with_expansion(inp, factor);

    let mut output = 0;
    while let Some(galaxy) = galaxies.pop() {
        output += galaxies
            .iter()
            .map(|og| galaxy.distance(og))
            .sum::<usize>();
    }

    output
}

pub fn part_one(inp: Vec<String>) -> usize {
    get_shortest_path_sum(inp, 1)
}

pub fn part_two(inp: Vec<String>) -> usize {
    get_shortest_path_sum(inp, 1000000)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_get_shortest_path_sum() {
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];

        assert_eq!(get_shortest_path_sum(input.clone(), 1), 374);
        assert_eq!(get_shortest_path_sum(input.clone(), 10), 1030);
        assert_eq!(get_shortest_path_sum(input.clone(), 100), 8410);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 374);
    }

    #[test]
    pub fn test_part_two() {
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 82000210);
    }
}
