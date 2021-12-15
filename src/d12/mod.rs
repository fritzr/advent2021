use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::collections::HashMap; // , hash_map::Entry};
use std::fmt::{Display, Formatter};

pub struct Day12;

fn name_trans(node: &str) -> String {
    if node.len() == 1 {
        node.to_owned() + node
    } else {
        node[(node.len()-2)..node.len()].to_owned()
    }
}

#[derive(Debug)]
struct Graph {
    adj: HashMap<String, Vec<String>>,
}

// Concatenation of (string) nodes forming a unique subpath.
//
// We abuse the fact that intermediate nodes always have length 2.
// This means the input shouldn't contain "rt" or "nd" as separate nodes,
// as that will be how "start" and "end" are stored.
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Subpath(String);

impl Subpath {
    fn with_capacity(c: usize) -> Self { Subpath(String::with_capacity(c)) }

    fn push(&mut self, node: &str) {
        self.0 += node;
        /*
        let mut chars: Vec<char> = self.0.chars().collect();
        chars.sort_unstable();
        self.0 = chars.into_iter().collect();
        */
    }

    fn top(&self) -> Option<&str> {
        if self.0.len() > 1 {
            Some(&self.0[(self.0.len()-2)..self.0.len()])
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<String> {
        if self.0.len() > 1 {
            Some(self.0.drain((self.0.len()-2)..self.0.len()).collect::<String>())
        } else {
            None
        }
    }

    fn contains(&self, s: &str) -> bool {
        self.0.as_bytes().chunks(2).find(|window| window == &s.as_bytes()).is_some()
    }
}

fn is_lower(s: &str) -> bool {
    s.find(char::is_uppercase) == None
}

// Unzipped (node, children) pairs indicating a traversed node and its remaining children.
#[derive(Debug)]
struct PathState {
    path: Subpath,
    child_counts: Vec<usize>,
}

impl PathState {
    fn with_capacity(c: usize) -> PathState {
        PathState { path: Subpath::with_capacity(c), child_counts: Vec::with_capacity(c) }
    }

    fn push(&mut self, (node, children): (&str, usize)) {
        self.path.push(node);
        self.child_counts.push(children)
    }

    fn pop(&mut self) -> Option<(String, usize)> {
        if let Some(top) = self.child_counts.pop() {
            Some((self.path.pop().expect("subpath not in sync with counts"), top))
        } else {
            None
        }
    }

    fn top(&self) -> Option<(&str, usize)> {
        if let Some(top) = self.child_counts.last() {
            Some((self.path.top().expect("subpath not in sync with counts"), *top))
        } else {
            None
        }
    }
}

struct PathCounter {
    stack: Subpath,
    status: PathState,
    verbose: bool,
    // counts: HashMap<Subpath, usize>, // TODO use to recognize common subtrees
}

impl PathCounter {
    fn with_capacity(c: usize, verbose: bool) -> PathCounter {
        PathCounter {
            stack: Subpath::with_capacity(c),
            status: PathState::with_capacity(c),
            verbose,
        }
    }

    fn push(&mut self, g: &Graph, node: &str) {
        // Don't visit lower-case nodes which have already been visited.
        // Push children onto the stack, and push the node and number
        // of children onto the status.
        let mut count = 0;
        if self.verbose {
            println!("pushing '{}' with children:", node);
        }
        for adj in g.adjacent(node).iter() {
            if !is_lower(adj) || !self.status.path.contains(adj) {
                if self.verbose {
                    print!("  '{}'", adj);
                }
                self.stack.push(adj);
                count += 1;
            }
        }
        if self.verbose {
            println!("  | count = {}", count);
        }
        self.status.push((node, count));
    }

    fn count(&mut self, g: &Graph, start: &str, end: &str) -> usize {
        let start = &name_trans(start);
        let end = &name_trans(end);
        let mut counts = 0;
        self.push(g, start);
        while let Some(node) = self.stack.pop() {
            if &node == end {
                counts += 1;
                if self.verbose {
                    println!("found end node, paths now = {}", counts);
                }
            } else {
                self.push(g, &node);
            }
            if self.verbose {
                println!("  STACK: {:?} | {}", self.stack, node);
                println!(" STATUS: {:?}", self.status);
            }
            let top = self.status.top().expect("empty status right after push");
            // If we're looking at a node with no children, we've reached the end of a path.
            // If the node is 'end' we've found a valid unique path, otherwise it's a deadend.
            if top.1 == 0 || &node == end {
                // Unroll status to the next branch with more children to visit.
                'popping: while let Some(top) = self.status.top() {
                    if top.1 <= 1 {
                        if self.verbose {
                            println!("popping '{}' from status", top.0);
                        }
                        self.status.pop();
                    } else {
                        *self.status.child_counts.last_mut().expect("Some top, None last") -= 1;
                        if self.verbose {
                            let top = self.status.top().expect("duh");
                            println!("now '{}' has {} children left", top.0, top.1);
                        }
                        break 'popping;
                    }
                }
            }
        }
        counts
    }
}

impl Graph {
    fn from(input: &mut dyn BufRead) -> Result<Graph, Box<dyn Error>> {
        let mut adj = HashMap::<String, Vec<String>>::new();
        for line in input.lines() {
            let line = line?;
            let mut items = line.split("-");
            let node1 = name_trans(items.next().ok_or("missing left node")?);
            let node2 = name_trans(items.next().ok_or("missing right node")?);
            adj.entry(node1.to_string())
                .or_insert_with(|| Vec::<String>::new())
                .push(node2.to_string());
            adj.entry(node2.into())
                .or_insert_with(|| Vec::new())
                .push(node1.into())
        }
        Ok(Graph { adj })
    }

    fn adjacent(&self, node: &str) -> &Vec<String> {
        &self.adj[node]
    }

    fn count_paths(&self, start: &str, end: &str, verbose: bool) -> usize {
        PathCounter::with_capacity(self.adj.len(), verbose).count(self, start, end)
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        for (key, values) in self.adj.iter() {
            write!(f, "{:>5}: |{}|", key, values.len())?;
            for value in values {
                write!(f, " {:>5}", value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Day for Day12 {
    fn mod_path(&self) -> &str { file!() }
    fn run(&self, input: &mut dyn BufRead, opts: &cli::Cli)
        -> Result<(PartResult, PartResult), Box<dyn Error>>
    {
        let g = Graph::from(input)?;
        if opts.verbose {
            println!("{}", g);
        }
        Ok((PartResult::from(|| g.count_paths("start".into(), "end".into(), opts.verbose)),
            PartResult::new()))
    }
}

