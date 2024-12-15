use crate::graph::Graph;
use crate::search::{dijkstra, Order};
use chrono::Datelike;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use std::iter::Sum;
use std::ops::Add;
use std::time::Instant;

pub fn run(year: u16, day: u16, cb: fn(&mut Runner, input: &[u8])) {
    let args: Vec<String> = std::env::args().collect();
    let selected_day = args
        .get(1)
        .map(|v| v.parse::<u32>().unwrap())
        .or(Some(chrono::Local::now().day()))
        .expect("Day number not provided") as u16;
    let op = args.get(2).cloned().or(Some(String::from("once"))).unwrap();

    let mut runner = Runner::new(day, op);

    if selected_day == day || selected_day == 0 {
        let input_data = load_input(year, day);

        cb(&mut runner, input_data.as_slice());
        runner.print();
    }
}

pub struct Runner {
    graph: Graph<Run, (), 16>,
    info: Vec<(String, String)>,
    tail: usize,
    day: u16,
    op: String,
}

impl Runner {
    fn new(day: u16, op: String) -> Self {
        Self {
            day,
            op,
            graph: Graph::<Run, (), 16>::new(),
            info: Vec::new(),
            tail: usize::MAX,
        }
    }

    pub fn start_over(&mut self) {
        self.tail = usize::MAX;
    }

    pub fn connect(&mut self, src: &str, dst: &str) {
        self.graph.connect(
            self.graph.node_index_by_ref(src).unwrap(),
            self.graph.node_index_by_ref(dst).unwrap(),
            (),
        );
    }

    pub fn set_tail(&mut self, tail: &str) {
        self.tail = self.graph.node_index_by_ref(tail).unwrap()
    }

    pub fn is_cold(&self) -> bool {
        match self.op.as_str() {
            "once" | "table_once" => true,
            _ => false,
        }
    }

    pub fn print(&self) {
        match self.op.as_str() {
            "once" | "bench" => {
                println!("--- Day {} ---", self.day);
                println!("Results:");
                for run in self.graph.nodes().iter() {
                    if run.value_str.is_empty() {
                        continue;
                    }

                    println!("  {}: {}", run.name, run.value_str);
                }
                if !self.info.is_empty() {
                    println!();
                    println!("Info:");
                    for (key, value) in self.info.iter() {
                        if value.contains("\n") {
                            let value2 = value.trim_end_matches(|v| v == '\n').replace("\n", "\n    ");
                            println!("  {}: \n    {}", key, value2);
                        } else {
                            println!("  {}: {}", key, value);
                        }
                    }
                }
                println!();
                println!("Times:");
                for run in self.graph.nodes().iter() {
                    println!("  {}: {}", run.name, format_duration(run.duration_ns));
                }
                if let Some(shortest) = self.shortest_time() {
                    println!();
                    println!("Total: {}", format_duration(shortest));
                }
            }
            _ => {
                panic!("Unknown op: {}", self.op);
            }
        };
    }

    pub fn prep<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: Fn() -> T,
    {
        let (v, _) = self.run(name, f);
        v
    }

    pub fn part<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: Fn() -> T,
        T: Display,
    {
        let (v, index) = self.run(name, f);
        self.graph.node_mut(index).value_str = format!("{}", v).to_string();
        v
    }

    pub fn info<T>(&mut self, name: &str, value: &T)
    where
        T: Display,
    {
        self.info.push((name.to_string(), format!("{value}")));
    }

    pub fn info_debug<T>(&mut self, name: &str, value: &T)
    where
        T: Debug,
    {
        self.info.push((name.to_string(), format!("{value:?}")));
    }

    fn run<T, F>(&mut self, name: &str, f: F) -> (T, usize)
    where
        F: Fn() -> T,
    {
        let before = Instant::now();
        let mut res = f();
        let after = Instant::now();
        let mut dur = after - before;

        let mut runs = 1;
        if !self.is_cold() {
            runs = match dur.as_millis() {
                0 => 2500,
                1 => 1000,
                2..=4 => 500,
                5..=9 => 100,
                10..=19 => 50,
                20..=49 => 20,
                50..=99 => 10,
                100..=299 => 4,
                300..=499 => 2,
                _ => 1,
            };

            let before = Instant::now();
            for _ in 0..runs {
                res = f();
            }
            let after = Instant::now();
            dur = after - before;
        }

        let index = self.graph.add_node(Run {
            name: name.to_owned(),
            value_str: String::new(),
            duration_ns: (dur.as_nanos() as i64) / runs,
        });
        if self.tail != usize::MAX {
            self.graph.connect(self.tail, index, ());
        }
        self.tail = index;

        (res, index)
    }

    fn shortest_time(&self) -> Option<i64> {
        let mut search = dijkstra().with_seen_space(0u64);
        for (root_index, run) in self.graph.roots() {
            search.push((root_index, run.duration_ns));
        }

        search.find(|s, (index, dur)| {
            let edges = self.graph.edges(index);
            if edges.is_empty() {
                return Some(dur);
            }

            for (next, _) in edges.iter() {
                s.push((*next, dur + self.graph.node(*next).duration_ns));
            }

            None
        })
    }
}

struct Run {
    name: String,
    duration_ns: i64,
    value_str: String,
}

impl AsRef<str> for Run {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Eq for Run {}

impl PartialEq<Self> for Run {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd<Self> for Run {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Run {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

pub struct WithExtra<TD, TX>(pub TD, pub TX);

impl<TD, TX> Display for WithExtra<TD, TX>
where
    TD: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (+data for next part)", self.0)
    }
}

pub fn format_duration(ns: i64) -> String {
    if ns == i64::MAX {
        return "-".to_string();
    }

    if ns > 10_000_000_000 {
        format!("{:.1}s", (ns as f64) / (1_000_000_000f64))
    } else if ns > 1_000_000_000 {
        format!("{:.2}s", (ns as f64) / (1_000_000_000f64))
    } else if ns > 1_000_000 {
        format!("{:.2}ms", (ns as f64) / (1_000_000f64))
    } else if ns > 1_000 {
        format!("{:.2}µs", (ns as f64) / (1_000f64))
    } else {
        format!("{}ns", ns)
    }
}

pub fn load_input(year: u16, day_number: u16) -> Vec<u8> {
    let mut buf = Vec::with_capacity(2048);
    let file_name = format!("./input/{}/day_{:02}.txt", year, day_number);
    match File::open(file_name.clone()) {
        Ok(mut file) => {
            file.read_to_end(&mut buf).expect("Could not read file");
            buf.to_vec()
        }
        Err(_) => {
            let token = option_env!("AOC_SESSION").expect("token not found");
            if token == "" {
                panic!("Env is not set")
            }

            eprintln!("Downloading input for day {}...", day_number);

            create_dir_all(format!("./input/{}", year)).expect("Could not create dir");
            let data = reqwest::blocking::Client::builder()
                .build()
                .unwrap()
                .get(format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day_number
                ))
                .header(
                    "User-Agent",
                    "AOC Runner (github.com/gissleh/aoc2023, by dev@gisle.me)",
                )
                .header("Authority", "adventofcode.com")
                .header("Cookie", format!("session={}", token))
                .send()
                .expect("failed to send request")
                .bytes()
                .expect("could not read file");

            buf.extend(data.iter());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_name)
                .expect("Could not open file");
            file.write_all(&buf).expect("Could not write file");

            data.to_vec()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct BothParts<T1, T2>(pub T1, pub T2)
where
    T1: Display,
    T2: Display;

impl<T1, T2> Sum<BothParts<T1, T2>> for BothParts<T1, T2>
where
    T1: Display,
    T2: Display,
    T1: Add<Output = T1> + Default + Copy,
    T2: Add<Output = T2> + Default + Copy,
{
    fn sum<I: Iterator<Item = BothParts<T1, T2>>>(iter: I) -> Self {
        iter.fold(Self(Default::default(), Default::default()), |a, b| {
            BothParts(a.0 + b.0, a.1 + b.1)
        })
    }
}

impl<T1, T2> Display for BothParts<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}
