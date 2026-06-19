#[path = "libs/kernel_core.rs"]
mod kernel_core;

fn main() {
    println!("Booting Akira Kernel...");
    
    // লাইব্রেরির ফাংশনগুলো কল করা হচ্ছে
    kernel_core::init_hardware();
    kernel_core::Memory_management();
}
