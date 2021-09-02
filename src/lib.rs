#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]
#![feature(binary_heap_retain)]

use std::cmp::Ordering;
//use std::thread;
//use libc;
//use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;
use std::thread::{current, sleep};

struct SchedCoop {
    current: Thread,
    crt_clock: u64,
    thread_list: BinaryHeap<Thread>,
    sleeping_threads: BinaryHeap<Thread>,
    exited_threads: BinaryHeap<Thread>,
    threads_started: bool,

}

#[derive(Clone)]
struct Thread {
    name: Option<String>,
    stack: u64,
    ctx: u64,
    tls: u64,
    runnable: bool,
    queueable: bool,
    exited: bool,
    flags: u16,
    wakeup_time: u64,
    detached: bool,

}


#[repr(C)]
struct uk_thread_attr_t {
    detached: bool,
    prio: i32,
    timeslice: u32,
}

#[repr(C)]
struct uk_thread {}


#[repr(C)]
struct uk_sched {
    yyield: extern fn(s: uk_sched),
    thread_add: extern fn(s: uk_sched, t: uk_thread) -> u32,
}

#[no_mangle]
pub extern "C" fn rust_start() {
    let a = vec![3, 4, 2, 1, 4, 2, 7];
    let b: Vec<_> = a.iter().map(|x| x + x).collect();
    for n in b {
//      thread::spawn(move || { 
        println!("hi from std {}", n);
        //     });
    }
}

impl SchedCoop {
    fn new() -> Self {
        Self {
            current: Thread::new(Some("Idle".to_string())),
            crt_clock: 0,
            thread_list: BinaryHeap::new(),
            sleeping_threads: BinaryHeap::new(),
            exited_threads: BinaryHeap::new(),
            threads_started: false,
        }
    }
    fn thread_add(&mut self, mut t: Thread) {
        t.runnable = true;
        t.wakeup_time = 0;
        self.thread_list.push(t);
    }
    fn update_sleeping(&mut self) {
        self.crt_clock += 1;
        loop {
            let mut i = self.sleeping_threads.peek();
            match i {
                None => { return }
                Some(mut t) => {
                    if t.wakeup_time < self.crt_clock {
                        self.thread_list.push(self.sleeping_threads.pop().unwrap());
                    } else {
                        return
                    }
                }
            }
        }
    }
    fn choose_next(mut self) -> Thread {
        let mut next;
        if !self.thread_list.is_empty() {
            next = self.thread_list.pop().unwrap();
            if let Some(name) = self.current.name.clone() {
                if name != "Idle".to_string() && self.current.runnable == true {
                    self.thread_list.push(self.current.clone())
                }
            }
        } else {
            if self.current.runnable == true {
                return self.current;
            } else {
                next = Thread::new(Some("Idle".to_string()));
            }
        }
        self.current = next;
        return self.current;
    }
    fn schedule(mut self) -> Thread {
        self.update_sleeping();
        return self.choose_next();
    }

    fn current_sleep(mut self, duration: u64) -> Thread {
        self.current.wakeup_time = self.crt_clock + duration + 1;
        self.current.runnable = false;
        self.sleeping_threads.push(self.current);
        self.current = Thread::new(Some("Idle".to_string()));
        return self.current
    }
    fn thread_woken(mut self, mut t: Thread) {
        if t.runnable {
            return
        }
        if self.sleeping_threads.iter().find(|x| x.eq(&t)).is_some() {
            self.sleeping_threads.retain(|x| !x.eq(&t));
        }
        t.wakeup_time = 0;
        t.runnable = true;
        if self.current != t || t.queueable == true {
            t.queueable == false;
            self.thread_list.push(t);
        }
    }
}

impl Thread {
    fn new(n: Option<String>) -> Self {
        Self {
            name: n,
            stack: 1,
            ctx: 0,
            tls: 0,
            runnable: true,
            queueable: false,
            exited: false,
            flags: 0,
            wakeup_time: 0,
            detached: false,
        }
    }
}


impl Eq for Thread {}

impl PartialEq<Self> for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.wakeup_time == other.wakeup_time
    }
}

impl PartialOrd<Self> for Thread {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Thread {
    fn cmp(&self, other: &Self) -> Ordering {
        other.wakeup_time.cmp(&self.wakeup_time)
    }
}
