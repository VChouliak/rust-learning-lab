use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use infrastructure::config::once_cell_config::{AppConfig, get_config, init_config};
use infrastructure::threads::WorkerThreadPool;
use service::analyzer::AnalyzerService;
use ta_core::traits::Job;

use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Fibonacci {
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<u64>,

        #[arg(short, long)]
        input: Option<PathBuf>,
    },
    Prime {
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<u64>,

        #[arg(short, long)]
        input: Option<PathBuf>,
    },
}

fn read_numbers(numbers: &mut Vec<u64>, input: &Option<PathBuf>) {
    if let Some(file) = input {
        let content = std::fs::read_to_string(file).expect("Failed to read file");
        for line in content.lines() {
            if let Ok(num) = line.trim().parse() {
                numbers.push(num);
            }
        }
    }
}

pub fn fib_big(n: u64) -> BigUint {
    if n == 0 { return Zero::zero(); }
    if n == 1 { return One::one(); }

    let mut a = Zero::zero();
    let mut b = One::one();
    for _ in 2..=n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    b
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn main() {
    let cli = Cli::parse();
    
    let mut cfg = AppConfig::new();
    let logical_cores = num_cpus::get();
    cfg.thread_counts.insert("analyzer_pool".to_string(), logical_cores);
    init_config(cfg);

    let pool = WorkerThreadPool::new(logical_cores);
    let chunk_size = get_config().chunk_size;
    let analyzer = AnalyzerService::new(&pool, chunk_size);

    match cli.command {
        Commands::Fibonacci { mut numbers, input } => {
            read_numbers(&mut numbers, &input);

            let results = Arc::new(Mutex::new(Vec::new()));
            let mut jobs: Vec<Box<dyn Job>> = Vec::new();

            for n in numbers {
                let results = Arc::clone(&results);
                jobs.push(Box::new(move || {
                    let value = fib_big(n);
                    results.lock().unwrap().push((n, value));
                }));
            }

            analyzer.process(jobs);
            
            let results = results.lock().unwrap();
            for (n, value) in results.iter() {
                println!("Fib({}) = {}", n, value);
            }
        }

        Commands::Prime { mut numbers, input } => {
            read_numbers(&mut numbers, &input);

            let results = Arc::new(Mutex::new(Vec::new()));
            let mut jobs: Vec<Box<dyn Job>> = Vec::new();

            for n in numbers {
                let results = Arc::clone(&results);
                jobs.push(Box::new(move || {
                    let prime = is_prime(n);
                    results.lock().unwrap().push((n, prime));
                }));
            }

            analyzer.process(jobs);

            let results = results.lock().unwrap();
            for (n, prime) in results.iter() {
                println!("{} is prime? {}", n, prime);
            }
        }
    }

    pool.shutdown();
}
