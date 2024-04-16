use std::thread::sleep;
//use std::io;
//use rand::Rng;
//use std::io::{Write, BufReader, BufRead, ErrorKind};
//use std::fs::File;
//use std::cmp::Ordering;
//use std::ops::Add;
use std::{env, thread};
use std::time::Duration;

fn is_prime(x: u128, primes: &[u128]) -> bool {
    let mut div = 2;
    for prime in primes {
        if x % prime == 0 {
            return false;
        }
        div = prime + 1;
    }
    // a quick and dirty fix for multi-threaded operation
    let mut ran_max = x;
    if primes.len() > 0 && primes[0] > 2 {
        div = 2;
        ran_max = primes[0];
    }

    while div < ran_max {
        if x % div == 0 {
            return false;
        }
        div += 1;
    }
    return x != 1;
}

fn prime_duty(s: u128, e: u128, th: u8, eepy: f32, verbose: bool) {
    if eepy > 0.0 {
        sleep(Duration::from_millis((eepy * 1000.0) as u64));
    }
    println!("Thread {} started searching from range {} to {}!", th, s, e);
    let mut i = s;
    let mut primes: Vec<u128> = Vec::new();
    if verbose {
        loop {
            if is_prime(i, &mut primes.as_slice()) {
                print!("{}\t", &i);
                primes.push(i);
            }
            if i >= e {
                break;
            }
            i += 1
        }
    } else {
        loop {
            if is_prime(i, &mut primes.as_slice()) {
                primes.push(i);
            }
            if i >= e {
                break;
            }
            i += 1
        }
        for prime in &primes {
            print!("{}\t", prime);
        }
    }
    println!();
    println!("Thread {} finished searching from range {} to {}. Found {} primes!", th, s, e, primes.len());
}

fn help(app_name: &str, threads: &u128) -> bool {
    println!("Command line switches\n");
    println!("--help\t\t\tDisplays this help screen");
    println!("--threads [n]\t\tSpecify the number of threads to allocate. Default is {} for your system.", threads);
    println!("--lim [n]\t\tSpecify the limit where to end searching for primes. Default is 5 000 000.");
    println!("--nonverbose\t\tHides primes during calculation");
    println!("--nolim\t\t\tDeactivate any limits for searching for primes. Useful for stress testing.");
    println!("--delay [n]\t\tThe number of seconds to wait before starting the calculation. Default is 5.0.");
    println!("\nExamples:\n");
    println!("{} --threads 1", app_name);
    println!("{} --threads {} --nonverbose --nolim", app_name, threads / 2);
    println!("{} --delay 0 --lim 1000000", app_name);
    true
}

fn main() {
    println!("\n1_000_000 primes\n");
    let args: Vec<String> = env::args().collect();
    let app_name = &args[0];
    extern crate num_cpus;
    let mut lim = 5_000_000;
    let mut threads = (num_cpus::get_physical() * 4) as u128;
    let mut thread = 1;
    let mut sleeptime: f32 = 0.0;
    let mut handles = vec![];
    let mut verbose = true;
    let mut exit = false;
    if args.len() > 1 {
        let mut allvalid = false;
        let mut next = "";
        for arg in &args {
            let mut valid = true;
            if next != "" {
                match next {
                    "--threads" => threads = arg.parse().expect("Invalid value for --threads flag. Unsigned 128-bit integer expected!"),
                    "--lim" => lim = arg.parse().expect("Invalid value for --lim flag. Unsigned 128-bit integer expected!"),
                    "--delay" => sleeptime = arg.parse().expect("Invalid value for --delay flag. 32-bit float expected!"),
                    _ => continue,
                }
                next = "";
                continue;
            }
            
            match arg.as_str() {
                "--help" => exit = help(&app_name, &threads),
                "--threads" => next = "--threads",
                "--lim" => next = "--lim",
                "--delay" => next = "--delay",
                "--nolim" => lim = u128::max_value(),
                "--nonverbose" => verbose = false,
                _ => valid = false,
            };
            if valid {
                allvalid = true;
            }
        }
        if exit {
            return;
        }
        if !allvalid {
            println!("Syntax error! Run {} --help to see correct syntax!", app_name);
            return;
        }
    }
    if threads < 1 {
        println!("Invalid number of threads! Must be 1 or greater!");
        return;
    }
    let division = lim / threads;
    
    while thread < threads {
        let t = thread::spawn(move || {
            println!("Thread {}: {} to {}", thread, division * (thread + 1) + 1, 2*division + division * thread);
            prime_duty(division * (thread + 1) + 1, 2*division + division * thread, thread as u8, sleeptime, verbose); 
        });
        handles.push(t);
        thread += 1;
    }
    println!("Main thread: {} to {}", 1, division);
    prime_duty(1, division, 0, sleeptime, verbose);
    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}