use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x.abs() + self.y.abs()).cmp(&(other.x.abs() + other.y.abs()))
    }
}


fn main() -> Result<(), Error> {
    let mut wires: Vec<Vec<(char, u64)>> = Vec::new();
    load_program(&mut wires)?;
    let mut occupancy = build_occupancy_vector(&wires)?;
    // remove origin
    occupancy[0].remove(&Point{ x: 0, y: 0 });
    let intersections = get_crossings(&occupancy)?;
    let distance = get_min_dist_point(&intersections)?;
    println!("{}", distance);

    Ok(())
}

fn load_program(wires: &mut Vec<Vec<(char, u64)>>) -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let mut wire: Vec<(char, u64)> = Vec::new();
        for segment in line.unwrap().trim().split(",") {
            wire.push((
                    segment.chars().next().unwrap(),
                    segment[1..].parse::<u64>().unwrap()
            ));
        }
        wires.push(wire);
    }
    Ok(())
}

fn build_occupancy_vector(wires: &Vec<Vec<(char, u64)>>) -> Result<Vec<HashSet<Point>>, Error> {
    let mut curr_point = Point{x: 0, y: 0};
    let mut occupancy: Vec<HashSet<Point>> = Vec::new();
    for wire in wires {
        curr_point.x = 0;
        curr_point.y = 0;
        let mut curr_occupancy = HashSet::new();
        for segment in wire {
            match segment.0 {
                'U' => {
                    for _ in 0..segment.1{
                        curr_occupancy.insert(curr_point.clone());
                        curr_point.y += 1;
                    }
                },
                'D' => {
                    for _ in 0..segment.1{
                        curr_occupancy.insert(curr_point.clone());
                        curr_point.y -= 1;
                    }
                },
                'L' => {
                    for _ in 0..segment.1{
                        curr_occupancy.insert(curr_point.clone());
                        curr_point.x -= 1;
                    }
                },
                'R' => {
                    for _ in 0..segment.1{
                        curr_occupancy.insert(curr_point.clone());
                        curr_point.x += 1;
                    }
                },
                _ => panic!("Unknown direction {}", segment.0)
            }
        }
        occupancy.push(curr_occupancy);
    }
    Ok(occupancy)
}

fn get_crossings(occupancy: &Vec<HashSet<Point>>) -> Result<Vec<Point>, Error> {
    let temp = occupancy[0].intersection(&occupancy[1]).cloned().collect::<Vec<Point>>();
    Ok(temp)
}

fn get_min_dist_point(points: &Vec<Point>) -> Result<i64, Error> {
    let min_point = points.iter().min().unwrap();
    Ok(min_point.x.abs() + min_point.y.abs())
}
