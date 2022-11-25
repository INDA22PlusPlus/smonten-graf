use std::io;
use std::io::prelude::*;
use std::cmp::min;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap(); // reads the entierty of standard input to one String
    
    let mut lines: Vec<&str> = buffer.lines().collect();
    while ! lines.is_empty() {
        let params: Vec<&str> = lines[0].split(" ").collect();
        // println!("params: {:?}", params);
        let n = params[0].parse::<usize>().unwrap();
        let m = params[1].parse::<usize>().unwrap();
        let q = params[2].parse::<usize>().unwrap();
        let s = params[3].parse::<usize>().unwrap();

        let mut edges: Vec<(usize, usize, u32)> = vec![];
        for i in 1..m+1 {
            let edge_params: Vec<&str> = lines[i].split(" ").collect();
            let from = edge_params[0].parse::<usize>().unwrap();
            let to = edge_params[1].parse::<usize>().unwrap();
            let w = edge_params[2].parse::<u32>().unwrap();
            edges.push((from, to, w));
        }

        // dbg!((n, m, q, s));
        // dbg!(edges);

        let mut graph = Graph::new(n, s, edges);

        for i in m+1..m+q+1 {
            let q = lines[i].parse::<usize>().unwrap();
            // dbg!(q);
            match graph.find_sp(q) {
                Ok(len) => println!("{}", len),
                Err(e) => println!("{}", e)
            }
        }
        println!();

        lines = lines[m+q+1..].to_vec();
        // dbg!(&lines);

        if lines.len() == 1 && lines[0] == "0 0 0 0" {
            break;
        };
    }
}   


#[derive(Debug, Clone)]
struct Vertex {
    is_unvisited: bool,
    sp: Option<u32>,
    prev_v: Option<usize>,
}
impl Vertex {
    fn new() -> Vertex {
        Vertex {is_unvisited: true, sp: None, prev_v: None }
    }
}

struct Graph {
    n: usize,
    s: usize,
    cur: usize,
    vertecies: Vec<Vertex>,
    am: Vec<Vec<(bool, u32)>>
}

impl Graph {
    fn new(n: usize, s: usize, edges: Vec<(usize, usize, u32)>) -> Graph {
        let mut am = vec![vec![(false, 0); n]; n];

        for edge in edges {
            am[edge.0][edge.1] = (true, edge.2);
        }

        let vertecies = vec![Vertex::new(); n];
        let cur = s;

        Graph{n, s, cur, vertecies, am}
    }

    fn get_closest_unvisited_neighbour(&mut self) -> Result<usize, String> {
        if self.cur == self.s && self.vertecies[self.s].is_unvisited {
            self.vertecies[self.cur].sp = Some(0);
            return Ok(self.cur);
        }

        let mut closest: Option<usize> = None;
        let mut shortest_dist: Option<u32> = None;

        for n in 0..self.n {
            if self.am[self.cur][n].0 && self.vertecies[n].is_unvisited {
                // an unvisited neighbour
                let dist_from_s_to_n = self.vertecies[n].sp.unwrap();

                match shortest_dist {
                    None => {
                        shortest_dist = Some(dist_from_s_to_n);
                        closest = Some(n);
                    },
                    Some(d) => {
                        if dist_from_s_to_n < d {
                            shortest_dist = Some(dist_from_s_to_n);
                            closest = Some(n);
                        }
                    }
                }
            }
        }
        return match closest {
            Some(n) => Ok(n),
            None => Err("Impossible".to_string())
        };
    }

    fn find_sp(&mut self, end: usize) -> Result<u32, String> {
        // reset:
        self.cur = self.s;
        self.vertecies = vec![Vertex::new(); self.n];

        // begin the algorithm

        while ! self.vertecies.is_empty() {

            self.cur = match self.get_closest_unvisited_neighbour() {
                Ok(n) => n,
                Err(_) => break
            };

            self.vertecies[self.cur].is_unvisited = false;

            if self.cur == end {
                return Ok(self.vertecies[self.cur].sp.unwrap());
            }

            for n in 0..self.n {
                if self.am[self.cur][n].0 {
                    // this is a neighbour to cur
                    let dist_from_cur_to_n = self.am[self.cur][n].1;
                    let dist_from_s_to_cur = self.vertecies[self.cur].sp.unwrap();
                    let dist_from_s_to_n = dist_from_s_to_cur + dist_from_cur_to_n;
                    match self.vertecies[n].sp {
                        None => {
                            // we can definde a distance
                            self.vertecies[n].sp = Some(dist_from_s_to_n);
                            self.vertecies[n].prev_v = Some(self.cur);
                        },
                        Some(d) => {
                            // we found a shorer distance!
                            if dist_from_s_to_n < d {
                                self.vertecies[n].sp = Some(dist_from_s_to_n);
                                self.vertecies[n].prev_v = Some(self.cur);
                            }
                        }
                    }

                }
            }
        }
        Err("Impossible".to_string())
    }
}


// #[derive(Debug, Clone)]
// struct Node {
//     visited: bool,
//     shortest_distance: u32
// }
// impl Node {
//     fn new() -> Node {
//         Node { visited: false, shortest_distance: 0 }
//     }
// }

// struct Graph {
//     n: usize,
//     s: usize,
//     cur: usize,
//     nodes: Vec<Node>,
//     am: Vec<Vec<(bool, u32)>> //change to u64?????
// }

// impl Graph {
//     fn new(n: usize, s: usize, edges: Vec<(usize, usize, u32)>) -> Graph {
//         let mut am = vec![vec![(false, 0); n]; n];

//         for edge in edges {
//             am[edge.0][edge.1] = (true, edge.2);
//         }

//         let nodes = vec![Node::new(); n];
//         let cur = s;

//         Graph{n, s, cur, nodes, am}
//     }

//     fn get_first_neighbour(&self) -> Result<usize, String> {
//         for n in 0..self.n {
//             if self.am[self.cur][n].0 {
//                 return Ok(n);
//             }
//         }
//         Err("Impossible".to_string())
//     }

//     fn closest_unvisited(&self) -> Result<usize, String> {
//         let mut closest_unvisited = match self.get_first_neighbour() {
//             Ok(i) => i,
//             Err(e) => return Err(e)
//         };
//         let mut sp = self.am[self.cur][closest_unvisited].1;

//         for n in closest_unvisited..self.n {
//             if self.am[self.cur][n].0 && self.am[self.cur][n].1 < sp {
//                 closest_unvisited = n;
//                 sp = self.am[self.cur][n].1;
//             }
//         }

//         Ok(closest_unvisited)
//     }

//     fn has_unvisited(&self) -> bool {
//         for n in 0..self.n {
//             if !self.nodes[n].visited {
//                 return true;
//             }
//         }
//         return true;
//     }

//     fn find_sp(&mut self, end: usize) -> Result<u32, String> {
//         // reset Graph
//         self.nodes = vec![Node::new(); self.n];
//         self.cur = self.s;

//         if self.cur == end {
//             return Ok(0);
//         }

//         while self.has_unvisited() {
//             let prev = self.cur;
//             self.cur = match self.closest_unvisited() {
//                 Ok(n) => n,
//                 Err(e) => return Err(e)
//             };
//             self.nodes[self.cur].shortest_distance = self.am[prev][self.cur].1;

//             if self.cur == end {
//                 return Ok(self.nodes[self.cur].shortest_distance)
//             } else if !self.has_unvisited() {
//                 return Err("Impossible".to_string());
//             }

//             for n in 0..self.n {
//                 if self.am[self.cur][n].0 {
//                     let new_dist = self.nodes[self.cur].shortest_distance + self.am[self.cur][n].1;
//                     if self.nodes[n].visited {
//                         self.nodes[n].shortest_distance = min(
//                             self.nodes[n].shortest_distance,
//                             new_dist
//                         );
//                     } else {
//                         self.nodes[n].shortest_distance = new_dist;
//                     }
//                 }
//             }

//         }
//         panic!("should not reach here");
//     }
// }


