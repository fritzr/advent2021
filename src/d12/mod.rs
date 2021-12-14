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
    fn with_capacity(c: usize) -> Self { Subpath(String::with_capacity(c)) }

    fn push(&mut self, node: &str) {
        self.0 += &node[(node.len()-2)..node.len()];
        let mut chars: Vec<char> = self.0.chars().collect();
        chars.sort_unstable();
        self.0 = chars.into_iter().collect();
    }

    /*
    fn top(&self) -> Option<&str> {
        if self.0.len() > 1 {
            Some(&self.0[(self.0.len()-2)..self.0.len()])
        } else {
            None
        }
    }
    */

    fn contains(&self, s: &str) -> bool {
        self.0.find(s).is_some()
    }
}

struct PathCounter<'a> {
    g: &'a Graph,
    start: String,
    end: String,
    counts: HashMap<Subpath, usize>,
}

fn is_lower(s: &str) -> bool {
    s.find(char::is_uppercase) == None
}

impl<'a> PathCounter<'a> {
    fn from(g: &'a Graph, start: String, end: String) -> Self {
        // assert_eq!(is_lower(start), true);
        Self { g, start, end, counts: HashMap::with_capacity(g.adj.len()) }
    }

    fn count_paths(&mut self, mut path: Subpath, node: &str)
        -> usize
    {
        if self.end == node {
            // prefix + end is now a complete distinct path
            1
        } else {
            path.push(node);
            match self.counts.entry(path) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => *e.insert(
                    self.g.adj[node].iter()
                        .filter_map(|adj| {
                            // Don't visit lowercase nodes twice in one path.
                            if !is_lower(node) || !path.contains(node) {
                                Some(self.count_paths(path.clone(), adj))
                            } else {
                                None
                            }
                        })
                        .sum()
                )
            }
        }
    }

    fn count(&mut self) -> usize {
        let start = &self.start;
        self.count_paths(Subpath::with_capacity(2 * self.g.adj.len()), start)
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

    fn count_paths(&self, from: String, to: String) -> usize {
        PathCounter::from(self, from, to).count()
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

