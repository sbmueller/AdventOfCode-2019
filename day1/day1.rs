use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let mut total_fuel = 0;
    let mut additional_fuel;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        additional_fuel = calculate_fuel(line.unwrap().parse::<i64>().unwrap());
        total_fuel += additional_fuel;
        // calculate fuel necessary for fuel mass
        while additional_fuel > 0 {
            additional_fuel = calculate_fuel(additional_fuel);
            total_fuel += additional_fuel;
        }
    }

    println!("Required fuel: {}", total_fuel);

    Ok(())
}

fn calculate_fuel(mass: i64) -> i64 {
    // integer division equals flooring
    let fuel = (mass / 3) - 2;
    if fuel < 0 { 0 } else { fuel }
}
