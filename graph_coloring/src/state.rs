use std::fmt;
use std::fmt::{Display, Formatter};

pub enum Algorithm {
    DepthFirst,
    ForwardCheck,
    MostConstrained,
}

struct Node {
    pub color: usize,
    pub index: usize,
    pub neighbors: Vec<usize>,
    pub options: Vec<usize>,
}

impl Node {
    pub fn new(i: usize) -> Node {
        Node {
            color: 0,
            index: i,
            neighbors: Vec::with_capacity(0),
            options: Vec::with_capacity(0),
        }
    }
}

pub struct Graph {
    vertices: usize,
    edges: usize,
    colors: usize,
    pub explored: usize,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: 0,
            edges: 0,
            colors: 0,
            explored: 0,
            nodes: Vec::new(),
        }
    }

    pub fn verts(&mut self, verts: usize) -> &mut Self {
        self.vertices = verts;
        let mut new_graph = Vec::with_capacity(verts);

        for i in 0..verts {
            new_graph.push(Node::new(i));
        }
        self.nodes = new_graph;

        self
    }

    pub fn edges(&mut self, edges: usize) -> &mut Self {
        self.edges = edges;
        self
    }

    pub fn colors(&mut self, colors: usize) -> &mut Self {
        self.colors = colors;
        let mut options = Vec::with_capacity(self.colors);
        for i in 1..self.colors + 1 {
            options.push(i);
        }

        for node in self.nodes.iter_mut() {
            node.options = options.clone();
        }
        self
    }

    pub fn connect(&mut self, n1: usize, n2: usize) {
        {
            let node1: &mut Node = self.nodes.get_mut(n1 - 1)
                .expect("Connect nodes don't exist");
            node1.neighbors.push(n2 - 1);
        }
        {
            let ref mut node2 = self.nodes.get_mut(n2 - 1)
                .expect("Connect nodes don't exist");
            node2.neighbors.push(n1 - 1);
        }
    }

    pub fn colorify(&mut self, alg: Algorithm) {
        let solved = match alg {
            Algorithm::DepthFirst => self.color_dfs(0),
            Algorithm::ForwardCheck => self.color_fc(0),
            Algorithm::MostConstrained => self.color_mcv(0),
        };
        if solved {
            self.print_coloring();
        } else {
            println!("No Solution.");
        }
        println!("{} branching nodes explored.", self.explored);
    }

    fn color_dfs(&mut self, index: usize) -> bool {
        if index >= self.vertices {
            if self.is_colored() {
                return true;
            }
            return false;
        }

        for i in 1..self.colors + 1 {
            self.explored += 1;
            unsafe { self.nodes.get_unchecked_mut(index).color = i; }
            if !self.valid(index) {
                continue;
            }
            if self.color_dfs(index + 1) == true {
                return true;
            }
        }
        unsafe { self.nodes.get_unchecked_mut(index).color = 0; }
        false
    }

    fn color_fc(&mut self, index: usize) -> bool {
        if index >= self.vertices {
            if self.is_colored() {
                return true;
            }
            return false;
        }

        // Being limited by borrow checker
        let options = self.nodes.get(index).unwrap().options.clone();
        for color in options.iter() {
            if *color == 0 {
                continue;
            }


            self.explored += 1;
            self.nodes.get_mut(index).unwrap().color = *color;
            self.alert_neighbors(index, *color);
            if self.color_fc(index + 1) == true {
                return true;
            }

            self.unalert_neighbors(index, *color);
        }
        unsafe { self.nodes.get_unchecked_mut(index).color = 0; }
        false
    }

    fn color_mcv(&mut self, index: usize) -> bool {
        if index >= self.vertices {
            if self.is_colored() {
                return true;
            }
            return false;
        }

        // Being limited by borrow checker
        let options = self.nodes.get(index).unwrap().options.clone();
        for color in options.iter() {
            if *color == 0 {
                continue;
            }

            self.explored += 1;
            self.nodes.get_mut(index).unwrap().color = *color;
            let next_i = findmcv(&self.nodes);
            self.alert_neighbors(index, *color);
            if self.color_mcv(next_i) == true {
                return true;
            }

            self.unalert_neighbors(index, *color);
        }
        unsafe { self.nodes.get_unchecked_mut(index).color = 0; }
        false
    }

    fn print_coloring(&self) {
        println!("s col {}", self.colors);
        let mut node_num = 1;
        for node in self.nodes.iter() {
            println!("l {} {}", node_num, node.color);
            node_num += 1;
        }
    }

    fn alert_neighbors(&mut self, i: usize, color: usize) {
        let neighbors = self.nodes.get_mut(i).unwrap().neighbors.clone();
        for neigh in neighbors.iter() {
            let neigh_node = self.nodes.get_mut(*neigh).unwrap();
            let ref mut opts = neigh_node.options;
            opts[color - 1] = 0;
        }
    }

    fn unalert_neighbors(&mut self, i: usize, color: usize) {
        let neighbors = self.nodes.get_mut(i).unwrap().neighbors.clone();
        for neigh in neighbors.iter() {
            let neigh_node = self.nodes.get_mut(*neigh).unwrap();
            let ref mut opts = neigh_node.options;
            opts[color - 1] = color;
        }
    }

    /// Check if node at index x is valid.
    fn valid(&self, i: usize) -> bool {
        let node = unsafe { self.nodes.get_unchecked(i) };
        for neigh in node.neighbors.iter() {
            let neigh_node = self.nodes.get(*neigh).unwrap();
            if neigh_node.color == 0 {
                continue;
            }
            if neigh_node.color == node.color {
                return false;
            }
        }
        true
    }

    fn is_colored(&self) -> bool {
        for node in self.nodes.iter() {
            if node.color == 0 {
                return false;
            }
        }
        true
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut i = 1;
        let mut out = String::new();
        for node in self.nodes.iter() {
            for neighbor in node.neighbors.iter() {
                out.push_str(&format!("({}->{})", i, neighbor));
            }
            i += 1;
        }
        f.write_str(&*out)
    }
}

fn findmcv(nodes: &Vec<Node>) -> usize {
    use std::usize;
    let mut mcv = usize::MAX;
    let mut min_options = usize::MAX;
    let mut index = 0;
    for node in nodes {
        if node.color != 0 {
            // Already colored.
            index += 1;
            continue;
        }
        // Count the number of zeros in a node.
        let mut total_options = 0;
        for opt in node.options.iter() {
            if *opt != 0 {
                total_options += 1;
            }
        }

        if total_options < min_options {
            mcv = index;
            min_options = total_options;
        }
        index += 1;
    }

    mcv
}
