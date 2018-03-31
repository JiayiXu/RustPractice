extern crate rand;
use rand::{Rng, thread_rng};
use std::sync::{Arc, Mutex, Condvar};
use std::collections::HashSet;
use std::thread::spawn;

struct Context(Vec<String>, HashSet<String>, i32, i32);

fn get_child_urls(url:&str) -> Vec<String> {
    let mut rng = thread_rng();
    let num_child_links = rng.gen_range(1, 10);
    let mut results = vec![];
    let splits: Vec<&str> = url.split('/').collect();
    if splits.len() < 10 {
        for i in 0..num_child_links {
            results.push(format!("{}/{}", url, i).to_owned());
        }
    }

    results
}

fn crawl(thread_num: i32, context_tuple: Arc<(Mutex<Context>, Condvar)>) {
    loop {
        let some_url;
        {
            let mut guard = context_tuple.0.lock().unwrap();
            guard.2 += 1;
            while guard.0.len() == 0 {
                if guard.2 == guard.3 {
                    context_tuple.1.notify_all();
                    return
                }
                guard = context_tuple.1.wait(guard).unwrap();
            }
            some_url = guard.0.pop();
            guard.2 -= 1;
        }
        if let Some(url) = some_url {
            println!("Thread:{} crawling {:?}", thread_num, url);
            let child_links = get_child_urls(&url);
            {
                let mut guard = context_tuple.0.lock().unwrap();
                for link in child_links {
                    if !guard.1.contains(&link) {
                        guard.1.insert(link.clone());
                        guard.0.push(link.clone());
                    }
                }
                context_tuple.1.notify_all();
            }
        }
    }
}

fn crawl_website(url: &str, num_threads: i32) {
    let seen: HashSet<String> = vec![url.to_owned()].into_iter().collect();
    let todo = Arc::new((Mutex::new(Context(vec![url.to_owned()], seen, 0, num_threads)), Condvar::new()));
    let mut thread_handles = vec![];
    
    for i in 0..num_threads {
        let my_todo = todo.clone();
        thread_handles.push(
            spawn(move || crawl(i, my_todo))
        );
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn main() {
    crawl_website("http://www.root.com", 10);
}
