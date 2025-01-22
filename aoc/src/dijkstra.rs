//! Dijktstra algorithm for shortest path finding

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Define an interface for a Dijkstra algo client

pub trait DijkstraController {
    // The node descriptor used by the controller
    // to uniquely identify its nodes.
    // For a grid-like map, this could be the (x,y) coordinate tuple.
    // It is opaque to the Dijkstra algo itself, but needs to
    // follow some bound/supertraits for hashing and copying.
    type Node: Copy + Clone + Eq + Hash;
    // Return a descriptor to the starting node
    fn get_starting_node(&self) -> Self::Node;
    // Return a descriptor to the destination node to search.
    // Dijkstra will stop as soon as this node is visited.
    // (or return a non-existant node if you want to map all the graph)
    fn get_target_node(&self) -> Self::Node;

    // Return a list of neighbors from a node,
    // along with their distance from it.
    // The controller may returned neighbors that have already
    // been visisted; the Dijkstra algo will filter them out
    // as needed.
    fn get_neighbors_distances(&self, node: &Self::Node) -> Vec<(Self::Node, usize)>;

    // This function will be called for each node that have been finalized
    // and have a known minimal distance from the start, along with their
    // previous node (if not unique, it will be arbitrary).
    // It can be an empty function or used only for debugging,
    // the dijkstra algo doesn't care.
    fn mark_visited_distance(
        &mut self,
        node: Self::Node,
        distance: usize,
        previous: Option<Self::Node>,
    );
}

/*
* Client implementation tips:
* When a "End" node may require different criteria/dimensions (such as direction of
* arrival) that modify the total distance, get_target_node() still needs to return one unique
* node. However it can be virtualized to a fake "node" linked to multiple copies
* of the real end node (with a distance of zero) representing different dimensions.

*/

// FIXME: need to pass controller as mut only to call "mark_visited_distance"
// which is not really needed
pub fn dijkstra<T: DijkstraController>(controller: &mut T) -> usize {
    // List of nodes that have been completely processed and won't be
    // visited again. Used to filter out the return of
    // controller.get_neighbors_distances();
    let mut finalized_nodes = HashSet::<T::Node>::new();

    // the "frontier" of unvisited nodes with their current total distance from start
    // and their previous node accounting for this distance.
    let mut unvisited_frontier = HashMap::<T::Node, (usize, Option<T::Node>)>::new();

    // The last set of the abstract algorithm, "all unvisited", is not needed here
    // and is indirectly implemented by the controller with its get_neighbors_distances()

    unvisited_frontier.insert(controller.get_starting_node(), (0, None));

    let target_node = controller.get_target_node();

    // Follow dijkstra algo
    while !unvisited_frontier.is_empty() {
        // Get the unvisited node with the smallest tentative distance.
        let shortest_node = unvisited_frontier
            .iter()
            .min_by(|a, b| a.1 .0.cmp(&b.1 .0))
            .unwrap();

        // Need to copy it to avoid a immutable borrow from line above to block
        // the mutable borrow of following remove_entry()
        let shortest_node = *shortest_node.0;

        let Some((current_node, (current_distance, previous_node))) =
            unvisited_frontier.remove_entry(&shortest_node)
        else {
            panic!("Impossible to remove node that was found");
        };

        finalized_nodes.insert(current_node);
        controller.mark_visited_distance(current_node, current_distance, previous_node);

        if current_node == target_node {
            return current_distance;
        }

        let neighbors = controller.get_neighbors_distances(&current_node);

        for (next_node, dist) in neighbors {
            if finalized_nodes.contains(&next_node) {
                // Old node, don't visit backward
                continue;
            }
            // distance to "node" via "current_node"
            let path_total_distance = dist + current_distance;
            if let Some((prev_dist, prev_node)) = unvisited_frontier.get_mut(&next_node) {
                // Update the best distance which was already known,
                // and from a better "previous node" (different path)
                if path_total_distance < *prev_dist {
                    *prev_dist = path_total_distance;
                    *prev_node = Some(current_node);
                }
            } else {
                // New unvisited neighbor, set initial best distance
                unvisited_frontier.insert(next_node, (path_total_distance, Some(current_node)));
            }
        }
    }

    eprintln!("Dijkstra algorithm finished exploring all nodes without reaching target !");

    usize::MAX
}

#[cfg(test)]
mod test {
    use super::*;

    struct BasicGraph {
        // Store a graph as each node is an index in the array,
        // and is itself an array of its neighbors as tuples
        // (neighbot_index,distance).
        graph: Vec<Vec<(usize, usize)>>,
        // will store path distance at end of dijkstra
        path: HashMap<usize, usize>,
    }

    impl DijkstraController for BasicGraph {
        type Node = usize;

        fn get_starting_node(&self) -> Self::Node {
            0
        }

        fn get_target_node(&self) -> Self::Node {
            // last element of the graph
            self.graph.len() - 1
        }

        fn get_neighbors_distances(&self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
            let neighbs = self.graph[*node].clone();
            eprintln!("neighbors of {node} are {:?}", neighbs);
            neighbs
        }

        fn mark_visited_distance(
            &mut self,
            node: Self::Node,
            distance: usize,
            _previous: Option<Self::Node>,
        ) {
            self.path.insert(node, distance);
            eprintln!("Visited {node} with distance {distance}");
        }
    }

    #[test]
    fn basic_dijkstra() {
        // Manual assembly required.
        /*
        0 ->(1)  1
          ->(10) 2
        1 ->(10) 2
          ->(5) 3
        2 ->(1) 4
        3 ->(6) 4
        4 -> terminal

        Shortest path is 0->2->4;
        0->1->2->4 or 0->1->3->4 are longer.
         */

        // add a few back-edges for spice.
        let n0 = vec![(1, 1), (2, 10)];
        let n1 = vec![(0, 1), (2, 10), (3, 5)];
        let n2 = vec![(1, 11), (4, 1)];
        let n3 = vec![(4, 6)];
        let n4 = vec![];

        let expected_d = 11;
        let mut expected_paths = HashMap::<usize, usize>::new();
        expected_paths.insert(0, 0);
        expected_paths.insert(1, 1);
        expected_paths.insert(3, 6);
        expected_paths.insert(2, 10);
        expected_paths.insert(4, 11);

        let mut graph = BasicGraph {
            graph: vec![n0, n1, n2, n3, n4],
            path: HashMap::<usize, usize>::new(),
        };

        let d = dijkstra(&mut graph);

        assert_eq!(d, expected_d);
        assert_eq!(graph.path, expected_paths);
    }

    use crate::grid::{Grid, GridBuilder};

    struct GridCost {
        cost: Grid<usize>,
        // will store distance/path at end of dijkstra
        path: Grid<(usize, Option<(usize, usize)>)>,
    }

    impl DijkstraController for GridCost {
        // The X,Y coordinates in the grid
        type Node = (usize, usize);

        fn get_starting_node(&self) -> Self::Node {
            // top-left corner
            (0, 0)
        }

        fn get_target_node(&self) -> Self::Node {
            // bottom-right corner
            (self.cost.width - 1, self.cost.height - 1)
        }

        fn get_neighbors_distances(&self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
            let mut neighbs = Vec::<(Self::Node, usize)>::with_capacity(4);
            let signed_node: (isize, isize) = (node.0 as isize, node.1 as isize);
            for delta in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n = (signed_node.0 + delta.0, signed_node.1 + delta.1);
                if let Some(v) = self.cost.checked_get(n.0, n.1) {
                    let nnode = (n.0 as usize, n.1 as usize);
                    neighbs.push((nnode, v));
                }
            }
            neighbs
        }

        fn mark_visited_distance(
            &mut self,
            node: Self::Node,
            distance: usize,
            previous: Option<Self::Node>,
        ) {
            self.path.set(node.0, node.1, (distance, previous));
        }
    }

    fn fill_backward_path(path: &mut Grid<char>, full: &Grid<(usize, Option<(usize, usize)>)>) {
        let mut node = (full.width - 1, full.height - 1);
        while node != (0, 0) {
            let follow;
            let (_, prev) = full.get(node.0, node.1);
            let c = match prev {
                None => panic!("Following path from end doesn't reach start"),
                Some(prev) => {
                    follow = prev;
                    if node.0 > prev.0 {
                        '>'
                    } else if node.0 < prev.0 {
                        '<'
                    } else if node.1 > prev.1 {
                        'v'
                    } else if node.1 < prev.1 {
                        '^'
                    } else {
                        '?'
                    }
                }
            };
            path.set(node.0, node.1, c);
            node = follow;
        }
        path.set(node.0, node.1, 'S');
    }

    fn grids_equal(g1: &Grid<char>, g2: &Grid<char>) -> bool {
        if g1.width != g2.width || g1.height != g2.height {
            return false;
        }

        for y in 0..g1.height {
            let r1 = g1.get_row_slice(y);
            let r2 = g2.get_row_slice(y);
            if r1 != r2 {
                return false;
            }
        }

        true
    }

    #[test]
    fn grid_maze_dijkstra() {
        let map = vec![
            "0493432911123",
            "0195450909123",
            "2255240909054",
            "1446580909052",
            "4546650909036",
            "1438510909054",
            "4457809909066",
            "3637810909053",
            "4654961909187",
            "4564672909193",
            "1224680909193",
            "2546540909191",
            "4322671119993",
        ];

        let pat = vec![
            "S       ^>>  ",
            "v>      ^ v  ",
            " v>>>>> ^ v  ",
            "      v ^ v  ",
            "      v ^ v  ",
            "     <v ^ v  ",
            "     v  ^ v  ",
            "     v> ^ v  ",
            "      v ^ v  ",
            "      v ^ v  ",
            "      v ^ v  ",
            "      v ^ v>>",
            "      v>>   v",
        ];

        let expected_d = 48;
        let mut gb = GridBuilder::<char>::new();
        for row in pat {
            let line: Vec<char> = row.chars().collect();
            gb.append_line(&line);
        }

        let expected_path = gb.to_grid();

        let mut gb = GridBuilder::<usize>::new();
        for row in map {
            gb.append_char_map(row);
        }

        let costmap = gb.to_grid();
        let (width, height) = (costmap.width, costmap.height);
        let mut graph = GridCost {
            cost: costmap,
            path: Grid::<(usize, Option<(usize, usize)>)>::new(width, height, (999, None)),
        };

        let d = dijkstra(&mut graph);

        println!("Map Distance is {d}");

        let mut path = Grid::<char>::new(width, height, ' ');
        fill_backward_path(&mut path, &graph.path);

        path.pretty_print();
        assert_eq!(d, expected_d);
        assert!(grids_equal(&path, &expected_path));
    }
}
