use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> Option<u32> {
    let garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut result: u32 = 0;
    for (ri, row) in garden.iter().enumerate()
    {
        for (ci, plot) in row.iter().enumerate()
        {
            if visited.contains(&(ri, ci)) {
                continue;
            }

            let mut current_region: HashSet<(usize, usize)> = HashSet::new();
            visited.insert((ri, ci));
            current_region.insert((ri, ci));

            let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
            stack.push_back((ri, ci));

            while stack.len() > 0 {
                let current_plot = stack.pop_front().unwrap();

                if current_plot.0 > 0 {
                    if garden[current_plot.0 - 1][current_plot.1] == *plot && !visited.contains(&(current_plot.0 - 1, current_plot.1)) {
                        current_region.insert((current_plot.0 - 1, current_plot.1));
                        visited.insert((current_plot.0 - 1, current_plot.1));
                        stack.push_back((current_plot.0 - 1, current_plot.1));
                    }
                }

                if current_plot.1 > 0 {
                    if garden[current_plot.0][current_plot.1 - 1] == *plot && !visited.contains(&(current_plot.0, current_plot.1 - 1)) {
                        current_region.insert((current_plot.0, current_plot.1 - 1));
                        visited.insert((current_plot.0, current_plot.1 - 1));
                        stack.push_back((current_plot.0, current_plot.1 - 1));
                    }
                }

                if current_plot.0 < garden.len() - 1 {
                    if garden[current_plot.0 + 1][current_plot.1] == *plot && !visited.contains(&(current_plot.0 + 1, current_plot.1)) {
                        current_region.insert((current_plot.0 + 1, current_plot.1));
                        visited.insert((current_plot.0 + 1, current_plot.1));
                        stack.push_back((current_plot.0 + 1, current_plot.1));
                    }
                }

                if current_plot.1 < garden[0].len() - 1 {
                    if garden[current_plot.0][current_plot.1 + 1] == *plot && !visited.contains(&(current_plot.0, current_plot.1 + 1)) {
                        current_region.insert((current_plot.0, current_plot.1 + 1));
                        visited.insert((current_plot.0, current_plot.1 + 1));
                        stack.push_back((current_plot.0, current_plot.1 + 1));
                    }
                }
            }
            // println!("Region with plant {:?}: {:?}", plot, current_region);

            let mut perimeter: u32 = 0;
            for slot in current_region.iter() {
                perimeter += 4 - current_region.iter().filter(|p| (p.0 == slot.0 && p.1 == slot.1 + 1) || (p.1 == slot.1 && p.0 == slot.0 + 1)).count() as u32;
                if slot.0 > 0 {
                    if current_region.contains(&(slot.0 - 1, slot.1)) { perimeter -= 1 }
                }
                if slot.1 > 0 {
                    if current_region.contains(&(slot.0, slot.1 - 1)) { perimeter -= 1 }
                }
            }
            result += current_region.len() as u32 * perimeter
        }
    }
    Some(result)
}

pub fn part_two(file_path: &str) -> Option<u32> {
    let input= fs::read_to_string(file_path).unwrap();
    let input: &str = &input;
    let garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut result: u32 = 0;
    for (ri, row) in garden.iter().enumerate()
    {
        for (ci, veg) in row.iter().enumerate()
        {
            if visited.contains(&(ri, ci)) {
                continue;
            }

            let mut current_region: HashSet<(usize, usize)> = HashSet::new();
            visited.insert((ri, ci));
            current_region.insert((ri, ci));

            let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
            stack.push_back((ri, ci));

            while stack.len() > 0 {
                let current_plot = stack.pop_front().unwrap();

                if current_plot.0 > 0 {
                    if garden[current_plot.0 - 1][current_plot.1] == *veg && !visited.contains(&(current_plot.0 - 1, current_plot.1)) {
                        current_region.insert((current_plot.0 - 1, current_plot.1));
                        visited.insert((current_plot.0 - 1, current_plot.1));
                        stack.push_back((current_plot.0 - 1, current_plot.1));
                    }
                }

                if current_plot.1 > 0 {
                    if garden[current_plot.0][current_plot.1 - 1] == *veg && !visited.contains(&(current_plot.0, current_plot.1 - 1)) {
                        current_region.insert((current_plot.0, current_plot.1 - 1));
                        visited.insert((current_plot.0, current_plot.1 - 1));
                        stack.push_back((current_plot.0, current_plot.1 - 1));
                    }
                }

                if current_plot.0 < garden.len() - 1 {
                    if garden[current_plot.0 + 1][current_plot.1] == *veg && !visited.contains(&(current_plot.0 + 1, current_plot.1)) {
                        current_region.insert((current_plot.0 + 1, current_plot.1));
                        visited.insert((current_plot.0 + 1, current_plot.1));
                        stack.push_back((current_plot.0 + 1, current_plot.1));
                    }
                }

                if current_plot.1 < garden[0].len() - 1 {
                    if garden[current_plot.0][current_plot.1 + 1] == *veg && !visited.contains(&(current_plot.0, current_plot.1 + 1)) {
                        current_region.insert((current_plot.0, current_plot.1 + 1));
                        visited.insert((current_plot.0, current_plot.1 + 1));
                        stack.push_back((current_plot.0, current_plot.1 + 1));
                    }
                }
            }
            // println!("Region with plant {:?}: {:?}", plot, current_region);
            let mut walls: HashSet<((usize, usize), Direction)> = HashSet::new();
            for slot in current_region.iter() {
                // Up wall
                if !current_region.contains(&(if slot.0 > 0 { slot.0 - 1 } else { usize::MAX }, slot.1)) {
                    let mut wall_found = false;

                    if slot.1 > 0 {
                        let mut x = slot.1 - 1;
                        loop {
                            if !current_region.contains(&(slot.0, x))
                                || (current_region.contains(&(slot.0, x)) && current_region.contains(&(if slot.0 > 0 { slot.0 - 1 } else { usize::MAX }, x))) {
                                break;
                            } else if walls.contains(&((slot.0, x), Direction::Up)) {
                                wall_found = true;
                                break;
                            } else if x > 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                    }

                    let mut x = slot.1 + 1;
                    loop {
                        if !current_region.contains(&(slot.0, x))
                            || (current_region.contains(&(slot.0, x)) && current_region.contains(&(if slot.0 > 0 { slot.0 - 1 } else { usize::MAX }, x)))  {
                            break;
                        } else if walls.contains(&((slot.0, x), Direction::Up)) {
                            wall_found = true;
                            break;
                        } else if x < garden.len() {
                            x += 1;
                        } else {
                            break;
                        }
                    }

                    if !wall_found {
                        walls.insert((*slot, Direction::Up));
                    }
                }

                // Down wall
                if !current_region.contains(&(slot.0 + 1, slot.1)) {
                    let mut wall_found = false;

                    if slot.1 > 0 {
                        let mut x = slot.1 - 1;
                        loop {
                            if !current_region.contains(&(slot.0, x))
                                || (current_region.contains(&(slot.0, x)) && current_region.contains(&(slot.0 + 1, x))) {
                                break;
                            } else if walls.contains(&((slot.0, x), Direction::Down)) {
                                wall_found = true;
                                break;
                            } else if x > 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                    }

                    let mut x = slot.1 + 1;
                    loop {
                        if !current_region.contains(&(slot.0, x))
                            || (current_region.contains(&(slot.0, x)) && current_region.contains(&(slot.0 + 1, x))) {
                            break;
                        } else if walls.contains(&((slot.0, x), Direction::Down)) {
                            wall_found = true;
                            break;
                        } else if x < garden.len() {
                            x += 1;
                        } else {
                            break;
                        }
                    }

                    if !wall_found {
                        walls.insert((*slot, Direction::Down));
                    }
                }

                // Left wall
                if !current_region.contains(&(slot.0, if slot.1 > 0 { slot.1 - 1 } else { usize::MAX })) {
                    let mut wall_found = false;

                    if slot.0 > 0 {
                        let mut x = slot.0 - 1;
                        loop {
                            if !current_region.contains(&(x, slot.1))
                                || (current_region.contains(&(x, slot.1)) && current_region.contains(&(x, if slot.1 > 0 { slot.1 - 1 } else { usize::MAX }))) {
                                break;
                            } else if walls.contains(&((x, slot.1), Direction::Left)) {
                                wall_found = true;
                                break;
                            } else if x > 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                    }

                    let mut x = slot.0 + 1;
                    loop {
                        if !current_region.contains(&(x, slot.1))
                            || (current_region.contains(&(x, slot.1)) && current_region.contains(&(x, if slot.1 > 0 { slot.1 - 1 } else { usize::MAX }))) {
                            break;
                        } else if walls.contains(&((x, slot.1), Direction::Left)) {
                            wall_found = true;
                            break;
                        } else if x < garden.len() {
                            x += 1;
                        } else {
                            break;
                        }
                    }

                    if !wall_found {
                        walls.insert((*slot, Direction::Left));
                    }
                }

                // Right wall
                if !current_region.contains(&(slot.0, slot.1 + 1)) {
                    let mut wall_found = false;

                    if slot.0 > 0 {
                        let mut x = slot.0 - 1;
                        loop {
                            if !current_region.contains(&(x, slot.1))
                                || (current_region.contains(&(x, slot.1)) && current_region.contains(&(x, slot.1 + 1))) {
                                break;
                            } else if walls.contains(&((x, slot.1), Direction::Right)) {
                                wall_found = true;
                                break;
                            } else if x > 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                    }

                    let mut x = slot.0 + 1;
                    loop {
                        if !current_region.contains(&(x, slot.1))
                            || (current_region.contains(&(x, slot.1)) && current_region.contains(&(x, slot.1 + 1))) {
                            break;
                        } else if walls.contains(&((x, slot.1), Direction::Right)) {
                            wall_found = true;
                            break;
                        } else if x < garden.len() {
                            x += 1;
                        } else {
                            break;
                        }
                    }

                    if !wall_found {
                        walls.insert((*slot, Direction::Right));
                    }
                }
            }

            // println!("Region {veg} has {:?} walls", walls.len())
            result += (current_region.len() * walls.len()) as u32;
        }
    }

    println!("result {result}");

    Some(result)
}