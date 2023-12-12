use std::fs::read_to_string;
use std::cmp::min;

#[derive(Debug, Clone, Copy)]
struct MapRange {
    key0: u64,
    val0: u64,
    count: u64,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    count: u64,
}

trait MapRangeUtils {
    fn get(&self, key: u64) -> u64;
}

trait RangeUtils {
    fn contains_range(&self, range: Range) -> bool;
    fn add(&mut self, new_range: Range);
    fn trim_range(&self, trim_range: Range) -> Vec<Range>;
}

impl MapRangeUtils for Vec<MapRange> {
    fn get(&self, key: u64) -> u64 {
        for map in self {
            if map.key0 <= key && key < map.key0 + map.count {
                return map.val0 + (key - map.key0);
            }
        }
        key
    }
}

impl RangeUtils for Vec<Range> {
    fn contains_range(&self, other_range: Range) -> bool {
        for range in self {
            let start = range.start;
            let end = range.start + range.count - 1;
            let new_start = other_range.start;
            let new_end = other_range.start + other_range.count - 1;
            if start <= new_start && new_end <= end {
                return true;
            }
        }
        false
    }

    fn add(&mut self, new_range: Range) {
        for range in self.iter_mut() {
            let start = range.start;
            let end = range.start + range.count - 1;
            let new_start = new_range.start;
            let new_end = new_range.start + new_range.count - 1;
            if start <= new_start && new_end <= end {
                return;
            }
            if new_start < start && start - 1 <= new_end && new_end <= end {
                range.start = new_range.start;
                range.count += start - new_start;
                return;
            }
            if start <= new_start && new_start <= end + 1 && end < new_end {
                range.count += new_end - end;
                return;
            }
            if new_start < start && end < new_end {
                range.start = new_range.start;
                range.count = new_range.count;
                return;
            }
        }
        self.push(new_range);
    }

    fn trim_range(&self, mut range: Range) -> Vec<Range> {
        for trim_range in self {
            let start = range.start;
            let end = range.start + range.count - 1;
            let trim_start = trim_range.start;
            let trim_end = trim_range.start + trim_range.count - 1;
            if trim_start <= start && end <= trim_end {
                return vec![];
            }
            if start < trim_start && trim_start <= end && end <= trim_end {
                range.count -= end - trim_start + 1;
                continue;
            }
            if trim_start <= start && start <= trim_end && trim_end < end {
                range.start += trim_end - start + 1;
                range.count -= trim_end - start + 1;
                continue;
            }
            if start < trim_start && trim_end < end {
                let range1 = Range {
                    start: range.start,
                    count: trim_start - start,
                };
                let range2 = Range {
                    start: trim_end + 1,
                    count: end - trim_end,
                };
                let mut ret = vec![];
                ret.append(&mut self.trim_range(range1));
                ret.append(&mut self.trim_range(range2));
                return ret;
            }
        }
        vec![range]
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed2soil: Vec<MapRange>,
    soil2fert: Vec<MapRange>,
    fert2water: Vec<MapRange>,
    water2light: Vec<MapRange>,
    light2temp: Vec<MapRange>,
    temp2humid: Vec<MapRange>,
    humid2loc: Vec<MapRange>,
}

impl Almanac {
    fn from(s: &str) -> Self {
        let seeds: Vec<u64>;
        let seed2soil: Vec<MapRange>;
        let soil2fert: Vec<MapRange>;
        let fert2water: Vec<MapRange>;
        let water2light: Vec<MapRange>;
        let light2temp: Vec<MapRange>;
        let temp2humid: Vec<MapRange>;
        let humid2loc: Vec<MapRange>;

        let mut line_iter = s.lines();

        // Seeds
        let seeds_line = line_iter.next().unwrap();
        assert!(seeds_line.starts_with("seeds: "));
        seeds = (&seeds_line[7..]).split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        line_iter.next();

        let mut parse_next_map = |name: &str| -> Vec<MapRange> {
            let header = line_iter.next().unwrap();
            assert_eq!(format!("{} map:", name), header);
            let mut ret = vec![];
            while let Some(line) = line_iter.next() {
                if line.is_empty() {
                    break;
                }
                let mut num_iter = line.split(' ').map(|x| x.parse::<u64>().unwrap());
                let val0 = num_iter.next().unwrap();
                let key0 = num_iter.next().unwrap();
                let count = num_iter.next().unwrap();
                assert!(num_iter.next().is_none());
                ret.push(MapRange { val0, key0, count });
            }
            ret
        };

        seed2soil = parse_next_map("seed-to-soil");
        soil2fert = parse_next_map("soil-to-fertilizer");
        fert2water = parse_next_map("fertilizer-to-water");
        water2light = parse_next_map("water-to-light");
        light2temp = parse_next_map("light-to-temperature");
        temp2humid = parse_next_map("temperature-to-humidity");
        humid2loc = parse_next_map("humidity-to-location");

        let ret = Almanac {
            seeds,
            seed2soil,
            soil2fert,
            fert2water,
            water2light,
            light2temp,
            temp2humid,
            humid2loc,
        };
        ret
    }

    fn find_seed_loc(&self, seed: u64) -> u64 {
        let soil = self.seed2soil.get(seed);
        let fert = self.soil2fert.get(soil);
        let water = self.fert2water.get(fert);
        let light = self.water2light.get(water);
        let temp = self.light2temp.get(light);
        let humid = self.temp2humid.get(temp);
        let loc = self.humid2loc.get(humid);
        loc
    }
}

fn part1(fpath: &str) -> u64 {
    let alm = Almanac::from(&read_to_string(fpath).unwrap());
    alm.seeds.iter()
        .map(|&x| alm.find_seed_loc(x))
        .min().unwrap()
}

fn part2(fpath: &str) -> u64 {
    let alm = Almanac::from(&read_to_string(fpath).unwrap());
    let mut seeds_iter = alm.seeds.iter();
    let mut ret = u64::MAX;
    // let mut cache = vec![];
    let mut merged = vec![];
    while let (Some(start), Some(count)) = (seeds_iter.next(), seeds_iter.next()) {
        let range = Range { start: *start, count: *count };
        merged.add(range);
        // for tr in cache.trim_range(range) {
        //     for seed in tr.start..(tr.start + tr.count) {
        //         ret = min(ret, alm.find_seed_loc(seed));
        //     }
        //     cache.add(tr);
        // }
    }
    println!("{:?}", merged);
    ret
}

fn main() {
    const INPUT: &str = "data/day05.txt";

    let part1_result = part1(INPUT);
    println!("[PART1] Final sum: {}", part1_result);

    let part2_result = part2(INPUT);
    println!("[PART2] Final sum: {}", part2_result);
}

#[test]
fn test_part1() {
    assert_eq!(35, part1("data/day05_example.txt"));
}

#[test]
fn test_part2() {
    assert_eq!(46, part2("data/day05_example.txt"));
}

#[test]
fn test_ranges() {
    let r1 = Range { start: 1, count: 4 };
    let r2 = Range { start: 3, count: 4 };
    let r3 = Range { start: 5, count: 4 };

    let v1 = vec![r1];
    let v2 = vec![r2];
    let v3 = vec![r3];
    assert!(v1.contains_range(r1));
    assert!(!v1.contains_range(r2));
    assert!(!v1.contains_range(r3));
    assert!(!v2.contains_range(r1));
    assert!(v2.contains_range(r2));
    assert!(!v2.contains_range(r3));
    assert!(!v3.contains_range(r1));
    assert!(!v3.contains_range(r2));
    assert!(v3.contains_range(r3));

    let mut v123 = vec![r1];
    v123.add(r3);
    assert_eq!(1, v123.len());
    assert_eq!(1, v123[0].start);
    assert_eq!(8, v123[0].count);
    let r4 = Range { start: 10, count: 2 };
    v123.add(r4);
    assert_eq!(2, v123.len());
    assert_eq!(1, v123[0].start);
    assert_eq!(8, v123[0].count);
    assert_eq!(10, v123[1].start);
    assert_eq!(2, v123[1].count);

    let mut trimmed = v123.trim_range(r4);
    assert_eq!(0, trimmed.len());
    trimmed = v123.trim_range(Range { start: 7, count: 4 });
    assert_eq!(1, trimmed.len());
    assert_eq!(9, trimmed[0].start);
    assert_eq!(1, trimmed[0].count);
}
