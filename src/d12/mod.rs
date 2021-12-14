use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::collections::{HashMap, hash_map::Entry};
use std::fmt::{Display, Formatter};

pub struct Day12;

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
    // fn with_capacity(c: usize) -> Self { Subpath(String::with_capacity(c)) }

    fn from_with_capacity(node: &str, c: usize) -> Subpath {
        let mut this = Subpath(String::with_capacity(capacity));
        this.0 += node;
        this
    }


    fn push(&mut self, node: &str) {
        self.0 += &node[(node.len()-2)..node.len()];
        let mut chars: Vec<char> = self.0.chars().collect();
        chars.sort_unstable();
        self.0 = chars.into_iter().collect();
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
        self.0.find(s).is_some()
    }
}

fn is_lower(s: &str) -> bool {
    s.find(char::is_uppercase) == None
}

// Unzipped (node, children) pairs indicating a traversed node and its remaining children.
struct PathState {
    path: Subpath,
    child_counts: Vec<usize>,
}

impl PathState {
    fn with_capacity(c: usize) -> PathState {
        PathState { path: Subpath::with_capacity(c), child_counts: Vec::with_capacity(c) }
    }

    fn pop(&mut self) -> Option<(String, usize)> {
        if let Some(top) = self.child_counts.pop() {
            Some(self.path.pop().expect("subpath not in sync with counts"), top)
        } else {
            None
        }
    }
}

struct PathCounter {
    stack: Vec<&str>,
    state: PathState,
    // counts: HashMap<Subpath, usize>, // TODO use to recognize common subtrees
}

impl PathCounter {
    fn with_capacity(g: &'a Graph, c: usize) -> PathCounter {
        PathCounter { Vec::with_capacity(c), PathState::with_capacity(c) }
    }

    fn push(&mut self, g: &Graph, node: &str) {
        self.stack.push(start);
    }

    fn count(&mut self, g: &Graph, start: &str, end: &str) -> usize {
        let mut counts = 0;
        self.push(start);
        let mut popped = false;
        while let Some(path) = stack.pop() {
            path.push((path, self.adj[path]));
            if let Some((node, mut children)) = path.top() {
                // If we just popped off the status, we finished up a child traversal.
                if popping {
                    children -= 1;
                    *path.1.last_mut() = children;
                }
                if children == 0 {
                    path.pop();
                    popped = true;
                } else {
                    // Still have children to visit.
                    if top == self.end {
                        // prefix is now a complete path.
                        1
                    } else {
                        path.push(node);
                        match self.counts.entry(path) {
                            Entry::Occupied(e) => *e.get(),
                            Entry::Vacant(e) => {
                                let mut sum = 0;
                                for adjacent in self.g.adj[node].iter()
                                    .filter(|adj| !is_lower(adj) || !e.key().contains(adj))
                                {
                                    let next_path = e.key().clone();
                                    self.count_paths(e.key().clone(), adjacent);
                                }
                                *e.insert(sum)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Graph {
    fn from(input: &mut dyn BufRead) -> Result<Graph, Box<dyn Error>> {
        let mut adj = HashMap::<String, Vec<String>>::new();
        for line in input.lines() {
            let line = line?;
            let mut items = line.split("-");
            let node1 = items.next().ok_or("missing left node")?;
            let node2 = items.next().ok_or("missing right node")?;
            adj.entry(node1.to_string())
                .or_insert_with(|| Vec::<String>::new())
                .push(node2.to_string());
            adj.entry(node2.into())
                .or_insert_with(|| Vec::new())
                .push(node1.into())
        }
        Ok(Graph { adj })
    }

    fn count_paths(&self, start: &str, end: &str) -> usize {
        PathCounter::with_capacity(g.adj.len()).count(self, start, end)
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
        Ok((PartResult::from(|| g.count_paths("start".into(), "end".into())),
            PartResult::new()))
    }
}

