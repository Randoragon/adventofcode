use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct CubeBatch {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    batches: Vec<CubeBatch>,
}

fn parse_line(line: &str) -> Game {
    let mut s = String::from(line);
    assert!(line.starts_with("Game "));
    s.replace_range(..5, "");

    let colon_idx = s.find(':').unwrap();
    let id = (s[..colon_idx]).parse::<usize>().unwrap();
    s.replace_range(..=(colon_idx + 1), "");

    let mut batches = vec![];
    for s in s.split("; ").map(String::from) {
        let mut red = 0usize;
        let mut green = 0usize;
        let mut blue = 0usize;

        for s in s.split(", ").map(String::from) {
            let split_idx = s.find(' ').unwrap();
            let num = (&s[..split_idx]).parse::<usize>().unwrap();
            match &s[(split_idx + 1)..] {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => unreachable!(),
            };
        }

        batches.push(CubeBatch{red, green, blue});
    }
    Game{id, batches}
}

fn check_game(game: &Game, n_cubes: CubeBatch) -> usize {
    for cubes in &game.batches {
        if cubes.red > n_cubes.red || cubes.green > n_cubes.green || cubes.blue > n_cubes.blue {
            return 0;
        }
    }
    game.id
}

fn get_fewest_cubes_per_type(game: &Game) -> CubeBatch {
    // Suboptimal, but I don't care, it's pretty -u-
    CubeBatch {
        red: game.batches.iter().max_by_key(|x| x.red).unwrap().red,
        green: game.batches.iter().max_by_key(|x| x.green).unwrap().green,
        blue: game.batches.iter().max_by_key(|x| x.blue).unwrap().blue,
    }
}

fn part1(fpath: &str, n_cubes: CubeBatch) -> usize {
    read_to_string(fpath)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(|x| check_game(&x, n_cubes))
        .sum()
}

fn part2(fpath: &str) -> usize {
    read_to_string(fpath)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(|x| get_fewest_cubes_per_type(&x))
        .map(|x| x.red * x.green * x.blue)
        .sum()
}

fn main() {
    const INPUT: &str = "data/day02.txt";
    const N_CUBES: CubeBatch = CubeBatch{red: 12, green: 13, blue: 14};

    let part1_result = part1(INPUT, N_CUBES);
    println!("[PART1] Final sum: {}", part1_result);

    let part2_result = part2(INPUT);
    println!("[PART2] Final sum: {}", part2_result);
}

#[test]
fn test_check_game() {
    let games: Vec<Game> = read_to_string("data/day02_example.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
    let n_cubes = CubeBatch{red: 12, green: 13, blue: 14};
    assert_eq!(games[0].id, check_game(&games[0], n_cubes));
    assert_eq!(games[1].id, check_game(&games[1], n_cubes));
    assert_eq!(0, check_game(&games[2], n_cubes));
    assert_eq!(0, check_game(&games[3], n_cubes));
    assert_eq!(games[4].id, check_game(&games[4], n_cubes));
}

#[test]
fn test_part1() {
    let n_cubes = CubeBatch{red: 12, green: 13, blue: 14};
    assert_eq!(8, part1("data/day02_example.txt", n_cubes));
}

#[test]
fn test_part2() {
    assert_eq!(2286, part2("data/day02_example.txt"));
}
