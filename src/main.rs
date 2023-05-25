use std::collections::HashSet;
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    recursive_prompt();
}

fn recursive_prompt() {
    println!("Welcome to the Collatz conjecture recursive prompter!");
    println!("Insert a number to run through the collatz conjecture function.");
    loop {
        let mut input = String::new();
        println!("Enter a number to calculate the Collatz conjecture for:");
        stdin().read_line(&mut input).expect("Failed to read line");
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid input: {}", e);
                continue;
            }
        };
        let mut cache = vec![];
        let mut visited = vec![];
        let collatz = collatz(&mut cache, &mut visited, n);
        if collatz {
            println!("{} calculates into 1", n);
        } else {
            println!("{} does not calculate into 1", n);
        }
        println!("Cache: {:?}", cache);
        println!("Visited: {:?}", visited);
    }
}

fn infinite_loop() {
    let cache: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));
    let num_threads = num_cpus::get(); // Get the number of available CPU cores

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let cache = Arc::clone(&cache); // Clone the Arc for each thread
            thread::spawn(move || {
                let mut local_cache = vec![];
                let mut visited = vec![];
                let mut n = thread_id + 1;
                loop {
                    let collatz = collatz(&mut local_cache, &mut visited, n);
                    let thread_str = format!("Thread {:>2}", thread_id + 1);
                    if collatz {
                        println!("{:<15} - {} calculates into 1", thread_str, n);
                    } else {
                        println!("{:<15} - {} does not calculate into 1", thread_str, n);
                    }

                    n += num_threads;
                    if n <= thread_id {
                        break;
                    }
                }

                let mut cache = cache.lock().unwrap();
                cache.extend(local_cache);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    let cache = cache.lock().unwrap();
    println!("Shared cache: {:?}", cache);
}

fn collatz_single(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

type Cache = Vec<usize>;
fn collatz(cache: &mut Cache, visited: &mut Vec<usize>, n: usize) -> bool {
    if visited.contains(&n) {
        false // Cycle detected
    } else if cache.contains(&n) {
        true // Number reached 1 previously
    } else {
        visited.push(n);
        let next = collatz_single(n);
        if next == 1 {
            cache.push(n);
            true
        } else {
            collatz(cache, visited, next)
        }
    }
}
