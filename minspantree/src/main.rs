use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap(); // reads the entierty of standard input to one String
    let lines: Vec<&str> = buffer.lines().collect();
    let mut parser = Parser::new(lines);
    parser.run();
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

    fn peak(&self) -> String {
        assert!(!self.lines.is_empty());
        let first = self.lines[0].clone();
        first
    }

    fn next(&mut self) -> String {
        assert!(!self.lines.is_empty());

        let first = self.lines[0].clone();
        self.lines = self.lines[1..].to_vec();
        first
    }

    fn run(&mut self) {
        while self.peak() != "0 0".to_string() {
            let first_line = self.next();
            let params: Vec<&str> = first_line.split(" ").collect();
            let n: usize = params[0].parse::<usize>().unwrap();
            let m: usize = params[1].parse::<usize>().unwrap();
            println!("__________________\n");
            println!("{} {}", n, m);

            for edge_nr in 0..m {
                let cur_line = self.next();
                let edge_params: Vec<&str> = cur_line.split(" ").collect();
                let u: usize = edge_params[0].parse::<usize>().unwrap();
                let v: usize = edge_params[1].parse::<usize>().unwrap();
                let w: i32 = edge_params[2].parse::<i32>().unwrap();

                println!("   {} {} {}", u, v, w);
            }

        }
    }
}