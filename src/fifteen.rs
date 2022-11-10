use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};
use std::io::{BufRead, Read};

/// Day 15: Chiton
///
/// - Construct a weighted graph
/// - Write Dijkstra's algorithm

#[derive(PartialEq, Debug)]
struct Cavern {
    places: Vec<Vec<Place>>,
    max_x: usize,
    max_y: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct Place {
    x: usize,
    y: usize,
    risk_level: u8,
    tentative_cost: usize,
}

impl Cavern {
    fn from_reader<R: BufRead>(r: R) -> Self {
        let inner: Vec<Vec<Place>> = r
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .map(|ch| ch.to_digit(/*base*/ 10).unwrap() as u8)
                    .enumerate()
                    .map(|(x, risk_level)| Place {
                        x,
                        y,
                        risk_level,
                        tentative_cost: usize::MAX,
                    })
                    .collect()
            })
            .collect();

        let max_y = inner.len() - 1;
        let max_x = inner.get(0).unwrap().len() - 1;

        Cavern {
            places: inner,
            max_x,
            max_y,
        }
    }

    fn expand(&mut self, factor: usize) {
        let mut new_rows = vec![];

        for row in &mut self.places {
            let start_row = row.clone();
            for col_group in 1..factor {
                let mut new_segment = start_row
                    .iter()
                    .map(|place| {
                        let mut new_place = place.clone();
                        new_place.risk_level = next(place.risk_level, col_group as u8);
                        new_place
                    })
                    .collect::<Vec<Place>>();
                row.append(&mut new_segment);
            }
        }

        for row_group in 1..factor {
            for row in self.places.iter() {
                let new_row = row
                    .iter()
                    .map(|place| {
                        let mut new_place = place.clone();
                        new_place.risk_level = next(place.risk_level, row_group as u8);
                        new_place
                    })
                    .collect();
                new_rows.push(new_row);
            }
        }

        self.places.append(&mut new_rows);
        self.max_x += 1;
        self.max_x *= factor;
        self.max_x -= 1;
        self.max_y += 1;
        self.max_y *= factor;
        self.max_y -= 1;
        assert_eq!(self.max_y, self.places.len() - 1);
    }

    fn walk(&self) -> usize {
        let adj_list = self.adjacency_list();

        let mut dist: HashMap<(usize, usize), _> =
            adj_list.iter().map(|(pos, _)| (*pos, usize::MAX)).collect();
        //dbg!(&adj_list);

        *dist.get_mut(&(0, 0)).unwrap() = 0;

        let mut heap = BinaryHeap::new();
        heap.push(State {
            position: (0, 0),
            cost: 0,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == (self.max_x, self.max_y) {
                return cost;
            }

            if cost > *dist.get(&position).unwrap() {
                continue;
            }

            for edge in adj_list.get(&position).unwrap() {
                let next = State {
                    cost: cost + edge.cost as usize,
                    position: edge.node,
                };

                if next.cost < *dist.get(&next.position).unwrap() {
                    heap.push(next);

                    *dist.get_mut(&next.position).unwrap() = next.cost;
                }
            }
        }

        panic!("goal not reachable")
    }

    fn adjacency_list(&self) -> HashMap<(usize, usize), Vec<Edge>> {
        let mut adj = HashMap::new();
        for (y, row) in self.places.iter().enumerate() {
            for (x, place) in row.iter().enumerate() {
                let from_y = y.checked_sub(1).unwrap_or(0);
                let to_y = min(y + 1, self.max_y);
                let from_x = x.checked_sub(1).unwrap_or(0);
                let to_x = min(x + 1, self.max_x);

                let mut ys: Vec<_> = (from_y..=to_y)
                    .filter(|dy| *dy != y)
                    .map(|dy| (x, dy))
                    .collect();
                let mut xs: Vec<_> = (from_x..=to_x)
                    .filter(|dx| *dx != x)
                    .map(|dx| (dx, y))
                    .collect();

                ys.append(&mut xs);
                adj.insert(
                    (x, y),
                    ys.iter()
                        .map(|(rx, ry)| {
                            let neighbour = self.places.get(*ry).unwrap().get(*rx).unwrap();
                            Edge {
                                node: (*rx, *ry),
                                cost: neighbour.risk_level,
                            }
                        })
                        .collect(),
                );
            }
        }

        adj
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Edge {
    node: (usize, usize),
    cost: u8,
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    position: (usize, usize),
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn next(i: u8, step: u8) -> u8 {
    (i + step - 1) % 9 + 1
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(next(1, 1), 2);
        assert_eq!(next(9, 1), 1);
        assert_eq!(next(8, 2), 1);
    }

    #[test]
    fn cavern_from_reader() {
        let input = "123
456
789";

        let cavern = Cavern::from_reader(BufReader::new(input.as_bytes()));

        let want = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.into_iter()
                    .enumerate()
                    .map(|(x, risk_level)| Place {
                        x,
                        y,
                        risk_level,
                        tentative_cost: usize::MAX,
                    })
                    .collect()
            })
            .collect();

        assert_eq!(
            cavern,
            Cavern {
                max_x: 2,
                places: want,
                max_y: 2,
            }
        );
    }

    #[test]
    fn walk() {
        let input = "199\n129\n911";
        let cavern = Cavern::from_reader(BufReader::new(input.as_bytes()));
        assert_eq!(cavern.walk(), 5);
    }

    #[test]
    fn walk_sample() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let cavern = Cavern::from_reader(BufReader::new(input.as_bytes()));
        assert_eq!(cavern.walk(), 40);
    }

    #[test]
    fn expand() {
        let input = "123\n456\n789";
        let mut cavern = Cavern::from_reader(BufReader::new(input.as_bytes()));
        cavern.expand(2);
        for row in cavern.places {
            for place in row {
                print!("{}", place.risk_level);
            }

            print!("\n");
        }
        assert_eq!(1, 2);
    }

    //#[test]
    fn walk_one() {
        let input = File::open("input/15.in").unwrap();
        let cavern = Cavern::from_reader(BufReader::new(input));
        assert_eq!(cavern.walk(), 0);
    }

    #[test]
    fn walk_two() {
        let input = File::open("input/15.in").unwrap();
        let mut cavern = Cavern::from_reader(BufReader::new(input));
        cavern.expand(5);

        assert_eq!(cavern.walk(), 0);
    }
}
