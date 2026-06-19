#[path = "libs/kernel_core.rs"]
mod kernel_core;
mod scheduler;
mod security;

use scheduler::{Process, ProcessState, AiScheduler};
use security::{ProcessActivity, AiSecurityMonitor};

fn main() {
    println!("Booting Akira Kernel...");
    kernel_core::init_hardware();
    kernel_core::Memory_management();

    // AI Scheduler
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

    // AI Security Monitor
    let monitor = AiSecurityMonitor::new();
    let activities = vec![
        ProcessActivity {
            id: 1,
            name: String::from("akira_core"),
            cpu_usage: 30.0,
            memory_usage: 40.0,
            syscall_rate: 200,
            network_access: false,
        },
        ProcessActivity {
            id: 4,
            name: String::from("unknown_process"),
            cpu_usage: 95.0,
            memory_usage: 85.0,
            syscall_rate: 2000,
            network_access: true,
        },
        ProcessActivity {
            id: 5,
            name: String::from("logger"),
            cpu_usage: 5.0,
            memory_usage: 10.0,
            syscall_rate: 100,
            network_access: false,
        },
    ];

    monitor.scan_all(&activities);
}
