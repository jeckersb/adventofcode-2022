use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NodeKind {
    Start,
    End,
    Normal,
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    height: i8,
}

#[derive(Debug)]
pub struct HeightMap {
    nodes: Vec<Node>,
    width: usize,
}

impl From<u8> for Node {
    fn from(c: u8) -> Self {
        match c {
            b'S' => Self {
                kind: NodeKind::Start,
                height: 0,
            },
            b'E' => Self {
                kind: NodeKind::End,
                height: (b'z' - b'a') as i8,
            },
            c @ b'a'..=b'z' => Self {
                kind: NodeKind::Normal,
                height: (c - b'a') as i8,
            },
            invalid => panic!("Invalid map height '{invalid}'"),
        }
    }
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let nodes = s
            .as_bytes()
            .iter()
            .copied()
            .filter(|c| char::is_ascii_alphabetic(&(*c as char)))
            .map(Node::from)
            .collect();

        Self { nodes, width }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    node: usize,
    cost: usize,
}

impl HeightMap {
    fn solve(&self) -> usize {
        let adj_list = self.adjacency_list();
        self.shortest_path(&adj_list, self.start_idx(), self.end_idx())
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let adj_list = self.adjacency_list();
        let goal = self.end_idx();
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_i, v)| (v.height == 0))
            .filter_map(|(i, _v)| self.shortest_path(&adj_list, i, goal))
            .min()
            .unwrap()
    }

    fn find_nodekind(&self, kind: NodeKind) -> usize {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_i, n)| n.kind == kind)
            .unwrap()
            .0
    }

    fn start_idx(&self) -> usize {
        self.find_nodekind(NodeKind::Start)
    }

    fn end_idx(&self) -> usize {
        self.find_nodekind(NodeKind::End)
    }

    fn adjacency_list(&self) -> Vec<Vec<Edge>> {
        (0..self.nodes.len())
            .map(|i| self.edges_for_node(i))
            .collect()
    }

    fn edges_for_node(&self, node: usize) -> Vec<Edge> {
        let mut edges = Vec::new();

        for candidate in self.neighbors_for_node(node) {
            if self.nodes[candidate].height <= self.nodes[node].height + 1 {
                edges.push(Edge {
                    node: candidate,
                    cost: 1,
                })
            }
        }

        edges
    }

    fn neighbors_for_node(&self, node: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        // up?
        if node >= self.width {
            neighbors.push(node - self.width)
        }

        // left?
        if node % self.width != 0 {
            neighbors.push(node - 1);
        }

        // right?
        if node % self.width != self.width - 1 {
            neighbors.push(node + 1);
        }

        // down?
        let down = node + self.width;
        if down < self.nodes.len() {
            neighbors.push(down)
        }

        neighbors
    }

    fn shortest_path(&self, adj_list: &[Vec<Edge>], start: usize, end: usize) -> Option<usize> {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == end {
                return Some(cost);
            }

            // Important as we may have already found a better way
            if cost > dist[position] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in &adj_list[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position] = next.cost;
                }
            }
        }

        // Goal not reachable
        None
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HeightMap {
    HeightMap::from(input)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &HeightMap) -> usize {
    input.solve()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &HeightMap) -> usize {
    input.solve_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE_INPUT)), 31);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE_INPUT)), 29);
    }

    #[test]
    fn test_neighbors_for_node() {
        let txt = "aaa\n\
	     aaa\n\
	     aaa";

        let map = HeightMap::from(txt);
        assert_eq!(map.neighbors_for_node(0), vec![1, 3]);
        assert_eq!(map.neighbors_for_node(1), vec![0, 2, 4]);
        assert_eq!(map.neighbors_for_node(2), vec![1, 5]);
        assert_eq!(map.neighbors_for_node(3), vec![0, 4, 6]);
        assert_eq!(map.neighbors_for_node(4), vec![1, 3, 5, 7]);
        assert_eq!(map.neighbors_for_node(5), vec![2, 4, 8]);
        assert_eq!(map.neighbors_for_node(6), vec![3, 7]);
        assert_eq!(map.neighbors_for_node(7), vec![4, 6, 8]);
        assert_eq!(map.neighbors_for_node(8), vec![5, 7]);
    }

    #[test]
    fn test_edges_for_node() {
        let none = "zzz\n\
	     zaz\n\
	     zzz";

        let map = HeightMap::from(none);
        assert_eq!(map.edges_for_node(4), vec![]);

        let all = "aaa\n\
		   aza\n\
		   aaa";

        let map = HeightMap::from(all);
        assert_eq!(
            map.edges_for_node(4),
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 1 },
                Edge { node: 5, cost: 1 },
                Edge { node: 7, cost: 1 }
            ]
        );

        let partial_example = "Sab\n\
			       abc\n\
			       acc";
        let map = HeightMap::from(partial_example);
        assert_eq!(
            map.edges_for_node(0),
            vec![Edge { node: 1, cost: 1 }, Edge { node: 3, cost: 1 },]
        );
        assert_eq!(
            map.edges_for_node(1),
            vec![
                Edge { node: 0, cost: 1 },
                Edge { node: 2, cost: 1 },
                Edge { node: 4, cost: 1 },
            ]
        );
    }
}
