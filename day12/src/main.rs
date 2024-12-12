use itertools::Itertools;
use std::{char, fs};

const DIRECTIONS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Region {
    plots: Vec<Plot>,
}

impl Region {
    fn get_area(&self) -> u64 {
        self.plots.len() as u64
    }

    fn get_perimeter(&self) -> u64 {
        let mut perimeter = 0u64;
        for plot in self.plots.iter().filter(|plot| self.is_outer_plot(plot)) {
            for dir in DIRECTIONS {
                if !self.plots.contains(&Plot {
                    plant: plot.plant,
                    x: plot.x + dir.0,
                    y: plot.y + dir.1,
                }) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn get_outer_plots(&self) -> Vec<Plot> {
        self.plots.clone()
            .into_iter()
            .filter(|plot| self.is_outer_plot(plot))
            .collect()
    }

    fn is_outer_plot(&self, plot: &Plot) -> bool {
        for dir in DIRECTIONS {
            if !self.plots.contains(&Plot {
                plant: plot.plant,
                x: plot.x + dir.0,
                y: plot.y + dir.1,
            }) {
                return true;
            }
        }
        false
    }

    fn get_sides(&self) -> u64 {
        let mut sides = 0u64;
        for dir in DIRECTIONS {
            sides += self.get_side(dir.0, dir.1);
        }
        sides
    }

    fn get_side(&self, x_off: i64, y_off: i64) -> u64 {
        let mut visited: Vec<Plot> = Vec::new();
        let mut result = 0u64;
        for plot in self.get_outer_plots().iter().sorted_by_key(|plot| (plot.y, plot.x))   {
            if visited.contains(&plot) {
                continue;
            }
            visited.push(plot.clone());
            if !self.plots.contains(&Plot {
                plant: plot.plant,
                x: plot.x + x_off,
                y: plot.y + y_off,
            }) {
                result += 1;
            } else {
                continue;
            }
            let mut next_plot = Plot {
                plant: plot.plant,
                x: plot.x + y_off.abs(),
                y: plot.y + x_off.abs(),
            };
            while self.plots.contains(&next_plot) {
                if !self.plots.contains(&Plot {
                    plant: next_plot.plant,
                    x: next_plot.x + x_off,
                    y: next_plot.y + y_off,
                }) {
                    visited.push(next_plot.clone());
                    next_plot = Plot {
                        plant: next_plot.plant,
                        x: next_plot.x + y_off.abs(),
                        y: next_plot.y + x_off.abs(),
                    };
                } else {
                    break;
                }
            }
        }
        result
    }

    fn get_prize(&self) -> u64 {
        self.get_area() * self.get_perimeter()
    }

    fn get_discount_prize(&self) -> u64 {
        self.get_area() * self.get_sides()
    }
}

#[derive(Hash, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Plot {
    plant: char,
    x: i64,
    y: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("file read works");
    let regions = extract_regions(&input);
    println!(
        "part1: {}",
        regions.iter().map(|region| region.get_prize()).sum::<u64>()
    );
    println!(
        "part2: {}",
        regions
            .iter()
            .map(|region| region.get_discount_prize())
            .sum::<u64>()
    );
}

fn extract_regions(input: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let plots = extract_plots(input);
    for plot in plots.iter() {
        if regions.iter().any(|region| region.plots.contains(plot)) {
            continue;
        }
        regions.push(find_region(&plot, &plots));
    }
    regions
}

fn extract_plots(input: &str) -> Vec<Plot> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, plant)| Plot {
                    plant,
                    x: x as i64,
                    y: y as i64,
                })
                .collect::<Vec<Plot>>()
        })
        .collect()
}

fn find_region(plot: &Plot, plots: &Vec<Plot>) -> Region {
    let mut region_plots: Vec<Plot> = Vec::new();
    find_adjacent_plots(plot, plots, &mut region_plots);
    Region {
        plots: region_plots.into_iter().unique().collect::<Vec<Plot>>(),
    }
}

fn find_adjacent_plots(plot: &Plot, plots: &Vec<Plot>, region_plots: &mut Vec<Plot>) {
    region_plots.push(plot.clone());
    for dir in DIRECTIONS {
        if let Some(next_plot) = plots
            .iter()
            .filter(|other| other.plant == plot.plant)
            .filter(|other| !region_plots.contains(other))
            .find(|other| other.x == plot.x + dir.0 && other.y == plot.y + dir.1)
        {
            find_adjacent_plots(next_plot, plots, region_plots);
        }
    }
}
