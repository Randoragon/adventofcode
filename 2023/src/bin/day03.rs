use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Span {
    row: usize,
    col_begin: usize,
    col_end: usize,
}

#[derive(Debug)]
struct Engine {
    matrix: Box<[char]>,
    matrix_shape: (usize, usize),
    nums: Box<[(u32, Span)]>,
    syms: Box<[Position]>,
}

impl Engine {
    fn from_schematic(mut s: String) -> Self {
        let mut matrix: Vec<char> = vec![];
        let mut matrix_shape = (0usize, 0usize);
        let mut nums: Vec<(u32, Span)> = vec![];
        let mut syms: Vec<Position> = vec![];

        let mut row = 0;
        let mut col = 0;
        let mut matrix_width = Option::<usize>::None;
        let mut num_begin: Option<usize> = None;
        if !s.ends_with('\n') {
            s.push('\n');
        }
        for c in s.chars() {
            assert!(c.is_ascii());


            if c.is_digit(10) {
                if num_begin.is_none() {
                    num_begin = Some(col);
                }
                matrix.push(c);
            } else {
                if let Some(first_col) = num_begin {
                    let row_offset = row * (1 + matrix_width.unwrap_or(0));
                    let str_begin_idx = row_offset + first_col;
                    let str_end_idx = row_offset + col;
                    let num_str = &s[str_begin_idx..str_end_idx];
                    let num = num_str.parse::<u32>().unwrap();
                    nums.push((num, Span { row, col_begin: first_col, col_end: col - 1 }));
                    num_begin = None;
                }

                if c == '\n' {
                    if matrix_width.is_none() {
                        matrix_width = Some(col);
                    }
                    row += 1;
                    col = 0;
                    continue;  /* Don't push to matrix or increment col */
                } else if c != '.' {
                    syms.push(Position { row, col });
                }
                matrix.push(c);
            }
            col += 1;
        }
        matrix_shape.0 = row;
        matrix_shape.1 = matrix_width.unwrap();

        Self {
            matrix: Box::from(matrix),
            matrix_shape,
            nums: Box::from(nums),
            syms: Box::from(syms),
        }
    }

    fn get_numbers_adjacent_to_symbols(&self) -> Vec<u32> {
        self.nums.iter()
            .filter(|x| self.num_is_adjacent_to_symbol(x))
            .map(|x| x.0)
            .collect()
    }

    fn num_is_adjacent_to_symbol(&self, num: &(u32, Span)) -> bool {
        let (h, w) = self.matrix_shape;
        let x_begin = if num.1.col_begin != 0 { num.1.col_begin - 1 } else { 0 };
        let x_end = if num.1.col_end < w - 1 { num.1.col_end + 1 } else { w - 1 };

        let mut ys = vec![];
        if num.1.row != 0 {
            ys.push(num.1.row - 1);
        }
        if num.1.row != h - 1 {
            ys.push(num.1.row + 1);
        }
        for y in ys {
            for x in x_begin..=x_end {
                if self.matrix[y * w + x] != '.' {
                    return true;
                }
            }
        }

        let left = self.matrix[num.1.row * w + x_begin];
        let right = self.matrix[num.1.row * w + x_end];
        if left != '.' && !left.is_digit(10) {
            return true;
        }
        if right != '.' && !right.is_digit(10) {
            return true;
        }

        false
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        self.syms.iter()
            .map(|x| self.get_gear_parts(*x))
            .filter(|x| x.len() == 2)
            .map(|x| x.iter().product())
            .collect()
    }

    fn get_gear_parts(&self, sym: Position) -> Vec<u32> {
        let (h, w) = self.matrix_shape;
        if self.matrix[sym.row * w + sym.col] != '*' {
            return vec![];
        }
        let x_begin = if sym.col != 0 { sym.col - 1 } else { 0 };
        let x_end = if sym.col < w - 1 { sym.col + 1 } else { w - 1 };
        let y_begin = if sym.row != 0 { sym.row - 1} else { 0 };
        let y_end = if sym.row != h - 1 { sym.row + 1 } else { h - 1 };

        let num_is_adjacent = |num: &(u32, Span)| -> bool {
            if num.1.row > y_end || num.1.row < y_begin {
                return false;
            }
            if num.1.col_end < x_begin || num.1.col_begin > x_end {
                return false;
            }
            true
        };

        let ret = self.nums.iter()
            .filter(|x| num_is_adjacent(*x))
            .map(|x| x.0)
            .collect();
        ret
    }
}


fn part1(fpath: &str) -> u32 {
    Engine::from_schematic(read_to_string(fpath).unwrap())
        .get_numbers_adjacent_to_symbols().iter()
        .sum()
}

fn part2(fpath: &str) -> u32 {
    Engine::from_schematic(read_to_string(fpath).unwrap())
        .get_gear_ratios().iter()
        .sum()
}

fn main() {
    const INPUT: &str = "data/day03.txt";

    let part1_result = part1(INPUT);
    println!("[PART1] Final sum: {}", part1_result);
    let part2_result = part2(INPUT);
    println!("[PART2] Final sum: {}", part2_result);
}

#[test]
fn test_part1() {
    assert_eq!(4361, part1("data/day03_example.txt"));
}

#[test]
fn test_part2() {
    assert_eq!(467835, part2("data/day03_example.txt"));
}
