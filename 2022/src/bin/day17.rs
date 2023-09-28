use std::fs;

fn is_valid(tower: &[u8], rows: &[u8], h: usize) -> bool {
    match tower.get(h..) {
        Some(ans_rows) => std::iter::zip(ans_rows, rows).all(|(ans_row, row)| ans_row & row == 0),
        None => true,
    }
}

fn jet(tower: &[u8], rows: &mut [u8], h: usize, c: char) {
    let mut next_rows = vec![];
    match c {
        '<' => {
            if rows.iter().all(|row| *row < 0b100_0000) {
                for row in rows.iter() {
                    next_rows.push(*row * 2);
                }
            }
        }
        '>' => {
            if rows.iter().all(|row| *row % 2 == 0) {
                for row in rows.iter() {
                    next_rows.push(*row / 2);
                }
            }
        }
        _ => unreachable!(),
    }

    if is_valid(tower, &next_rows, h) {
        for (i, row) in next_rows.iter().enumerate() {
            rows[i] = *row;
        }
    }
}

fn calculate_tower(input: &str, n: usize) -> Vec<u8> {
    #[allow(clippy::unreadable_literal)]
    let rocks: [Vec<u8>; 5] = [
        vec![0b0011110],
        vec![0b0001000, 0b0011100, 0b0001000],
        vec![0b0011100, 0b0000100, 0b0000100],
        vec![0b0010000; 4],
        vec![0b0011000; 2],
    ];
    #[allow(clippy::unreadable_literal)]
    let mut tower: Vec<u8> = vec![0b1111111];

    let rock_iterator = rocks.iter().cycle();
    let mut jet_iterator = input.chars().cycle();

    for r in rock_iterator.take(n) {
        let mut rows = r.clone();
        let mut h = tower.len() + 3;

        while is_valid(&tower, &rows, h) {
            if let Some(c) = jet_iterator.next() {
                jet(&tower, &mut rows, h, c);
            }
            h -= 1;
        }

        for (i, &row) in rows.iter().enumerate() {
            if let Some(tower_row) = tower.get_mut(h + 1 + i) {
                *tower_row |= row;
            } else {
                tower.push(row);
            }
        }
    }
    tower
}

fn part1(input: &str) -> usize {
    let tower = calculate_tower(input, 2022);
    tower.len() - 1
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input: String = fs::read_to_string("input/day17").expect("Failed to read input");

    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1_514_285_714_288);
    }
}
