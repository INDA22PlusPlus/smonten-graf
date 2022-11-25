use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap(); // reads the entierty of standard input to one String
    let lines: Vec<&str> = buffer.lines().collect();
    let mut parser = Parser::new(lines);
    parser.run().unwrap();

    // let v = Vertex::new(vec![false, true, false, false]);
    // dbg!(v);

    // let bin_idx = "011";
    // let intval = u32::from_str_radix(bin_idx, 2).unwrap().to_string();
    // println!("{}", intval);
}

#[derive(Debug, Clone)]
enum Operation {
    N,
    F,
    S,
    C
}

impl Operation {
    fn get(ch: char) -> Operation {
        match ch {
            'N' => Operation::N,
            'F' => Operation::F,
            'S' => Operation::S,
            'C' => Operation::C,
            _ => panic!("unvalid char")
        }
    }
}

struct Parser {
    lines: Vec<String>
}

impl Parser {
    fn new(inp_lines: Vec<&str>) -> Parser {
        let mut lines: Vec<String> = vec![]; 
        for inp_line in inp_lines {
            lines.push(inp_line.to_string());
        }
        Parser { lines }
    }

    fn next(&mut self) -> String {
        assert!(!self.lines.is_empty());

        let first = self.lines[0].clone();
        self.lines = self.lines[1..].to_vec();
        first
    }

    fn skip(&mut self) {
        assert!(!self.lines.is_empty());
        self.lines = self.lines[1..].to_vec();
    }

    fn run(&mut self) -> Result<String, String> {
        let N = self.next().parse::<usize>().unwrap();
        for case_nr in 0..N {
            let first_line = self.next();
            let params: Vec<&str> = first_line.split(" ").collect();
            let L = params[0].parse::<usize>().unwrap();
            let n_op = params[1].parse::<usize>().unwrap();
            let n_w = params[2].parse::<usize>().unwrap();

            let mut available_ops: Vec<(Vec<Operation>, u32)> = vec![];
            for op_nr in 0..n_op {
                let line: Vec<char> = self.next().chars().collect();
                let mut ops: Vec<Operation> = vec![];
                for i in 0..L {
                    ops.push(Operation::get(line[i]));
                }
                let c = (
                    line[L+1].to_string()
                ).parse::<u32>().unwrap();
                available_ops.push((ops, c));
            }

            let mut tasks: Vec<(String, String)> = vec![];
            for task_nr in 0..n_w {
                let line = &self.next();
                let start = line[0..L].to_string();
                let end = line[L+1..2*L+1].to_string();
                tasks.push((start, end));
            }

            println!("creating a case in Parser");
            let mut case = Case::new(L, n_op, n_w, available_ops, tasks);
            case.run();

            // println!("Case {}", case_nr);
            // println!("{} {} {}", L, n_op, n_w);
            // for available_op in available_ops {
            //     println!("{:?}", available_op);
            // }
            // for task in tasks {
            //     println!("{:?}", task);
            // }
            // println!("_______________________\n\n");
        }
        Ok("".to_string())
    }

    fn char_to_bool(&self, c: char) -> bool {
        match c {
            '0' => false,
            '1' => true,
            _ => panic!()
        }
    }
}

struct Case {
    L: usize,
    n_op: usize,
    n_w: usize,
    vertecies: HashMap<String, Vertex>,
    available_ops: Vec<(Vec<Operation>, u32)>,
    tasks: Vec<(String, String)>,
    start: String,
    end: String,
    cur: String,
    total_cost: u32
}

impl Case {
    fn new(
        L: usize,
        n_op: usize,
        n_w: usize,
        available_ops: Vec<(Vec<Operation>, u32)>,
        tasks: Vec<(String, String)>
    ) -> Case {
        let vertecies = HashMap::new();
        let start = "0000".to_string();
        let end = "0000".to_string();
        let cur = "0000".to_string();
        let total_cost = 0;
        Case {L, n_op, n_w, vertecies, available_ops, tasks, start, end, cur, total_cost}
    }

    fn run(&mut self) {
        println!("running in Case");
        let mut output = "".to_string();
        let mut insert_space = false;
        for task_nr in 0..self.n_w {
            let task = self.tasks[task_nr].clone();
            let result = match self.find_sp(task.0, task.1) {
                Ok(d) => d.to_string(),
                Err(e) => e
            };
            if insert_space {
                output += " ";
            } else {
                insert_space = true;
            }
            output += &result;
        }
        println!("{}", output);
    }

    fn get_vertex(&self, s: &String) -> Vertex {
        self.vertecies.get(s).unwrap().clone()
    }

    fn calc_new_string(&self, op: Vec<Operation>) -> String {
        let cur_string_chars: Vec<char> = self.cur.clone().chars().collect();
        let mut new_string = "".to_string();
        for i in 0..self.L {
            new_string += match op[i] {
                Operation::N => {
                    match cur_string_chars[i] {
                        '0' => "0",
                        '1' => "1",
                        _ => panic!("unvalid char")
                    }
                },
                Operation::F => {
                    match cur_string_chars[i] {
                        '0' => "1",
                        '1' => "0",
                        _ => panic!("unvalid char")
                    }
                },
                Operation::S => "1",
                Operation::C => "0"
            };
        }
        return new_string;
    }

    fn visit_neighbour(&mut self, op: Vec<Operation>, cost: u32) {

        let cur_string_chars: Vec<char> = self.cur.clone().chars().collect();
        println!("cur_string_chars: {:?}", cur_string_chars);
        let mut new_string = "".to_string();
        for i in 0..self.L {
            dbg!(&op[i]);
            new_string += match op[i] {
                Operation::N => {
                    match cur_string_chars[i] {
                        '0' => "0",
                        '1' => "1",
                        _ => panic!("unvalid char")
                    }
                },
                Operation::F => {
                    match cur_string_chars[i] {
                        '0' => "1",
                        '1' => "0",
                        _ => panic!("unvalid char")
                    }
                },
                Operation::S => "1",
                Operation::C => "0"
            };
        }

        println!("new string: {}", new_string.clone());

        if self.vertecies.contains_key(&new_string) {
            let mut other_vertex = self.vertecies.get(&new_string).unwrap().clone();
            if self.total_cost + cost < other_vertex.sp {
                other_vertex.sp = self.total_cost + cost;
                self.vertecies.insert(new_string.clone(), other_vertex);
            }
        } else {
            let new_vertex = Vertex {
                word: new_string.clone(),
                unvisited: true,
                sp: self.total_cost + cost
            };
            self.vertecies.insert(new_string.clone(), new_vertex);
        }
        

        
    }

    fn get_closest_unvisited_vertex(& mut self) -> Result<Vertex, String> {
        if self.cur == self.end {
            return Ok(
                Vertex {
                    word: self.cur.clone(),
                    sp: self.total_cost,
                    unvisited: false 
                }
            )
        } else {

            println!("now looking at all the neighbours to {}", &self.cur);

            let mut least_cost = 1001;
            let mut cur_vertex: Vertex = Vertex { word: "0000".to_string(), sp: 0, unvisited: false};
            for i in 0..self.n_op {
                let (op, cost) = self.available_ops[i].clone();
                let mut new_string = self.calc_new_string(op);
                

                if !self.vertecies.contains_key(&new_string) {
                    // CASE 1: first time visiting this vertex
                    self.vertecies.insert(
                        new_string.clone(),
                        Vertex { word: new_string.clone(), sp: self.total_cost+cost, unvisited: true }
                    );
                }
                
                let other_vertex = self.vertecies.get(&new_string).unwrap().clone();
                if other_vertex.unvisited {
                    if other_vertex.sp < least_cost {
                        least_cost = other_vertex.sp;
                        cur_vertex = other_vertex;
                    }
                }
            }
            if least_cost == 1001 {
                return Err("NP".to_string());
            } else {
                return Ok(cur_vertex)
            }
        }
    }

    fn find_sp(&mut self, start: String, end: String) -> Result<u32, String> {
        self.start = start.clone();
        self.cur = start;
        self.end = end;

        loop {
            println!("self.cur = {}", &self.cur);
            let mut closest_vertex = match self.get_closest_unvisited_vertex() {
                Ok(v) => v,
                Err(e) => break,
            };
            closest_vertex.unvisited = false;
            if closest_vertex.word == self.end {
                return Ok(closest_vertex.sp);
            }
            self.vertecies.insert(self.cur.clone(), closest_vertex);
            
    
            println!("Finding neighbours to {} :", self.start);
            // println!("   Operations: {:?}", self.available_ops);
            for i in 0..self.n_op {
                let (op, cost) = self.available_ops[i].clone();
                self.visit_neighbour(op, cost);
            }
    
            println!("_________________\n");
        }
        Err("NP".to_string())
    }

    // fn new_string_and_cost(&self, s: String, op_stmt: (Vec<Operation>, u32)) -> (String, u32) {
    //     let chars: Vec<char> = s.chars().collect();
    //     let mut new_String = "".to_string();
    //     for j in 0..self.L {
    //         new_String += match op_stmt.0[j] {
    //             Operation::N => "",
    //             Operation::F => {
    //                 match chars[j] {
    //                     '0' => "1",
    //                     '1' => "0",
    //                     _ => panic!("unvalid char")
    //                 }
    //             },
    //             Operation::S => "1",
    //             Operation::C => "0"
    //         }
    //     }
    //     (new_String, op_stmt.1)
    // }

    // fn find_sp(&mut self, start: String, end: String) -> Result<u32, String> {
    //     //update:
    //     self.start = start.clone();
    //     self.cur = start;
    //     self.end = end;


    //     let mut total_cost = 0;

    //     loop {
    //         self.cur = match self.get_closest_unvisited() {
    //             Ok(s) => s,
    //             Err(e) => return Err(e)
    //         };
    //         if self.cur == self.end {
    //             let cur_vertex = self.get_vertex(&self.cur);
    //             total_cost += cur_vertex.sp;
    //             return Ok(total_cost)
    //         }

    //         for op in self.available_ops.clone() {
    //             let (other_string, cost) = self.new_string_and_cost(self.cur.clone(), op);
    //             match self.vertecies.get(&other_string) {
    //                 None => self.vertecies.insert(
    //                     other_string, 
    //                     Vertex { word: other_string, sp: total_cost+cost, unvisited: true }
    //                 ),
    //                 Some(v) => {
    //                     let vertex = v.clone();
    //                 }
    //                 _ => (),
    //             }
    //         }

    //         for nk in nks {
    //             if self.vertecies.contains_key(&nk.0) {
    //                 let mut other_vertex = self.vertecies.get(&nk.0).unwrap().clone();
    //                 if total_cost + nk.1 < other_vertex.sp {
    //                     other_vertex.sp = total_cost + nk.1;
    //                     other_vertex.unvisited = false;
    //                     self.vertecies.insert(nk.0, other_vertex);
    //                 }
    //             }
    //         }
    //     }






    //     Err("NP".to_string())
    // }


    // fn get_all_neighbour_keys(&self) -> Vec<(String, u32)> {
    //     let mut nks: Vec<(String, u32)> = vec![];
    //     for i in 0..self.n_op {
    //         let cur_chars: Vec<char> = self.cur.clone().chars().collect();
    //         let mut new_String = "".to_string();
    //         let (ops, c) = self.available_ops[i].clone();
    //         for j in 0..self.L {
    //             new_String += match ops[j] {
    //                 Operation::N => "",
    //                 Operation::F => {
    //                     match cur_chars[j] {
    //                         '0' => "1",
    //                         '1' => "0",
    //                         _ => panic!("unvalid char")
    //                     }
    //                 },
    //                 Operation::S => "1",
    //                 Operation::C => "0"
    //             }
    //         }
    //         nks.push((new_String, c));
    //     }
    //     nks
    // }

//     fn get_closest_unvisited(&mut self) -> Result<String, String> {
//         if self.end == self.cur && !self.vertecies.contains_key(&self.end){
//             self.vertecies.insert(
//                 self.cur.clone(),
//                 Vertex::new(self.cur.clone(), false)
//             );
//             return Ok(self.cur.clone())
//         }

//         let nks = self.get_all_neighbour_keys();
//         let mut smallest_cost = 1001;
//         let mut closest_n_k: Option<String> = None;
//         for nk in nks {
//             let nbv = self.vertecies.get(&nk.0).unwrap().clone();
//             if nbv.unvisited && nk.1 < smallest_cost {
//                 closest_n_k = Some(nk.0.clone());
//                 smallest_cost = nk.1;
//             }

//         }
//         return match closest_n_k {
//             None => Err("NP".to_string()),
//             Some(s) => Ok(s)
//         };
//     }
}


#[derive(Debug, Clone)]
struct Vertex {
    word: String,
    sp: u32,
    unvisited: bool
}

impl Vertex {
    fn new(word: String, unvisited: bool) -> Vertex {
        let sp = 0;
        Vertex { word, sp, unvisited}
    }
}

