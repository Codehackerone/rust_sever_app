use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    workers: Vec<Worker>,           // Vector to hold worker threads.
    sender: mpsc::Sender<Message>,      // Channel to send jobs from `execute` function.
}

type Job = Box<dyn FnOnce() + Send + 'static>;  // Type alias for closure job.

enum Message{
    NewJob(Job),
    Terminate,
}

impl ThreadPool{
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool{ 
        assert!(size > 0);       // Checking whether size of pool is greater than zero.

        let (sender, receiver) = mpsc::channel();  // Creating a channel between the main thread and worker threads.

        let receiver = Arc::new(Mutex::new(receiver)); // Wrapping the receiver in `Arc<Mutex<>>` to use it across multiple threads.

        let mut workers = Vec::with_capacity(size);  // Initializing an empty vector of worker threads with given size capacity.

        for id in 0..size{
            // create some threads and store them in the vector
            workers.push(Worker::new(id, 
                Arc::clone(&receiver)));   // Cloning the `receiver` instead of sharing ownership.
        }

        ThreadPool {
            workers,
            sender           
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static    // Ensure that function passed is only called once.
    {
        let job = Box::new(f);       // Wrapping the closure in box before passing to receiver.

        self.sender.send(Message::NewJob(job)).unwrap();  // Sending the job to the receiver.
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();  // Sending terminate message to all workers.
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take(){   // Taking the thread out of the worker.
                thread.join().unwrap();     // Joining the thread to wait for it to finish.
            }
        }
    }
}
struct Worker{
    id: usize,                  // Unique ID for every worker thread.
    thread: Option<thread::JoinHandle<()>>,   // Option to hold the thread.
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop{    // Spawning the thread which will execute the job.
            let job = receiver
            .lock()
            .unwrap()          // Locking the mutex and unwrapping to get access to the data inside the lock.
            .recv()            // Retreiving the message from the channel (blocking call).
            .unwrap();

            println!("Worker {} got a job; executing.", id);
            match Message::Terminate{
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                },
            }
        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}

// This is a Rust program that defines a simple thread pool, which is used for executing jobs in parallel.

// At the beginning of the code, we import a few important packages from the Rust standard library:

// Copy
// Insert
// New
// use std::{thread, sync::{mpsc, Arc, Mutex}};
// std::thread: This package provides everything required for working with threads.
// std::sync: This package contains synchronization primitives such as mutexes and channels, which are used to communicate between threads.
// We then define a ThreadPool struct, which holds a vector of worker threads and a channel through which jobs can be sent to the workers.

// Copy
// Insert
// New
// pub struct ThreadPool{
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }
// workers: A vector of Worker structs representing all the worker threads in the pool.
// sender: A mpsc::Sender<Message> object that allows messages of type Message to be sent to the worker threads.
// We then define a Job type as an alias for a boxed closure that takes no arguments and returns nothing:

// Copy
// Insert
// New
// type Job = Box<dyn FnOnce() + Send + 'static>;
// Box: A smart pointer provided by Rust standard library that allows ownership transfer by wrapping heap-allocated data.
// dyn: dynamic dispatch, makes it work with any trait object which implements the defined signature
// FnOnce(): Trait for closures taking 0 arguments, returning void after being called (only once)
// Send: Adds a marker/interface/enforcement to make sure the closure is sendable - So the reference can be transferred across threads
// 'static: Defines how long the closure should live or whether its lifetime is "static" here defined as "a closure from all possible lifetimes".
// An enum type called Message is also defined in this code, which represents the messages that can be sent over the channel:

// Copy
// Insert
// New
// enum Message{
//     NewJob(Job),
//     Terminate,
// }
// NewJob(Job): With a Job, indicating a new job/task to do.
// Terminate: Without any argument, indicating that the worker thread should stop processing jobs.
// The ThreadPool implementation provides the following functions:

// new: initializes the thread pool with a given