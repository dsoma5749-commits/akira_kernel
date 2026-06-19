#[path = "libs/kernel_core.rs"]
mod kernel_core;
mod scheduler;

use scheduler::{Process, ProcessState, AiScheduler};

fn main() {
    println!("Booting Akira Kernel...");
    kernel_core::init_hardware();
    kernel_core::Memory_management();

    let mut ai = AiScheduler::new();

    ai.add_process(Process {
        id: 1,
        name: String::from("akira_core"),
        priority: 10,
        cpu_usage: 20.0,
        wait_time: 0,
        state: ProcessState::Ready,
    });

    ai.add_process(Process {
        id: 2,
        name: String::from("memory_daemon"),
        priority: 5,
        cpu_usage: 50.0,
        wait_time: 3,
        state: ProcessState::Ready,
    });

    ai.add_process(Process {
        id: 3,
        name: String::from("security_monitor"),
        priority: 8,
        cpu_usage: 10.0,
        wait_time: 1,
        state: ProcessState::Ready,
    });

    println!("\n--- Akira AI Scheduler ---");
    for _ in 0..3 {
        ai.next();
    }
}
