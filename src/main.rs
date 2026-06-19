#[path = "libs/kernel_core.rs"]
mod kernel_core;

fn main() {
    println!("Booting Akira Kernel...");
    
    kernel_core::init_hardware();
    kernel_core::Memory_management();
}
