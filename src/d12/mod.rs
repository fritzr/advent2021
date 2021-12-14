use std::io::BufRead;
use crate::{cli, Day, PartResult};
use std::error::Error;
use std::collections::HashMap;
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
type Subpath = String;

struct PathCounter<'a> {
    g: &'a Graph,
    start: String,
    end: String,
    counts: HashMap<Subpath, usize>,
}

fn is_lower(s: &String) -> bool {
    s.find(char::is_uppercase) == None
}

fn is_upper(s: &String) -> bool {
    s.find(char::is_lowercase) == None
}

impl<'a> PathCounter<'a> {
    fn from(g: &'a Graph, start: String, end: String) -> Self {
        // assert_eq!(is_lower(start), true);
        Self { g, start, end, counts: HashMap::with_capacity(g.adj.len()) }
    }

    fn count_paths(&mut self, prefix: Subpath, node: String)
        -> usize
    {
        if node == self.end {
            1
        } else {
            let previous = if prefix.len() > 1 {
                prefix[(prefix.len()-2)..prefix.len()]
            } else {
                self.start.to_string()
            };
            let prefix = prefix + &node[(node.len()-2)..node.len()];
            counts.entry(*prefix).or_insert(|| self.g.adj[node].filter_map(|adj| {
                // Don't visit the previous node, and don't visit lowercase nodes twice.
                if adj != previous && (!is_lower(node) || subpath.find(node) == None) {
                    Some(self.count_paths(*prefix, adj))
                } else {
                    None
                }
            }).sum())
        }
    }

    fn count(&mut self) -> usize {
        let prefix = String::with_capacity(2 * self.g.adj.len());
        self.count_paths(prefix, self.start)
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

