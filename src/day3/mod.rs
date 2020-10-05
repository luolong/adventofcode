use std::fs::File;
use std::io::{self, BufReader, Lines};

use self::point::*;
use self::wire::*;
use std::cmp;

mod direction;
mod point;
mod wire;

#[derive(Debug, Eq, PartialEq)]
struct Intersections {
    closest_to_origin: Point,
    closest_by_wire: (usize, Point),
}

impl Intersections {
    fn update(&mut self, distance: usize, point: Point) {
        self.closest_to_origin = cmp::min(self.closest_to_origin, point);
        if distance < self.closest_by_wire.0 {
            self.closest_by_wire = (distance, point);
        }
    }
}

impl From<ParseWireError> for String {
    fn from(err: ParseWireError) -> Self {
        format!("Failed to parse wire: {:?}", err)
    }
}

pub fn day3(lines: Lines<BufReader<File>>) -> Result<(), String> {
    let lines = lines
        .collect::<io::Result<Vec<String>>>()
        .map_err(|err| format!("Failed to read a line: {}", err))?;

    let wires = lines
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Wire>, ParseWireError>>()?;

    if let Some(i) = find_intersections(wires) {
        println!("Closest intersection to origin is {} with manhattan distance of {}", i.closest_to_origin, i.closest_to_origin.distance_from_origin());

        let (distance, point) = i.closest_by_wire;
        println!("Closest intersection by wire is {}, with total wire distance of {}", point, distance);
    } else {
        println!("All wires are neatly organized! None of them cross");
    }

    Ok(())
}

fn find_intersections(wires: Vec<Wire>) -> Option<Intersections> {
    let mut intersections: Option<Intersections> = None;

    let mut rest = wires.as_slice();
    while let Some((first, others)) = rest.split_first() {
        for other in others {
            for (distance, pt) in first.intersections(other) {
                match intersections.as_mut() {
                    Some(it) => it.update(distance, pt),
                    None => {
                        intersections = Some(Intersections {
                            closest_to_origin: pt,
                            closest_by_wire: (distance, pt),
                        })
                    }
                }
            }
        }
        rest = others;
    }
    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let wires = vec![
            "R8,U5,L5,D3".parse().unwrap(),
            "U7,R6,D4,L4".parse().unwrap(),
        ];

        let intersections = find_intersections(wires).unwrap();
        assert_eq!(intersections.closest_to_origin, Point::from((3, 3)));
        assert_eq!(intersections.closest_to_origin.distance_from_origin(), 6);
        assert_eq!(intersections.closest_by_wire, (30, Point::from((6, 5))));
    }

    #[test]
    fn example_2() {
        let wires = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap(),
            "U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap(),
        ];

        let intersections = find_intersections(wires).unwrap();
        assert_eq!(intersections.closest_to_origin, Point::from((155, 4)));
        assert_eq!(intersections.closest_to_origin.distance_from_origin(), 159);
        assert_eq!(intersections.closest_by_wire, (610, Point::from((158, -12))));
    }

    #[test]
    fn example_3() {
        let wires = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".parse().unwrap(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap(),
        ];

        let intersections = find_intersections(wires).unwrap();
        assert_eq!(intersections.closest_to_origin, Point::from((124, 11)));
        assert_eq!(intersections.closest_to_origin.distance_from_origin(), 135);
        assert_eq!(intersections.closest_by_wire, (410, Point::from((107, 47))));
    }
}
