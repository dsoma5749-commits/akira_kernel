# Akira Kernel
Akira Kernel is a modern, AI-driven process scheduler implemented in Rust. Designed for efficiency and high performance, it utilizes a heuristic scoring algorithm to manage system processes intelligently.
🚀 Features
AI-Driven Scheduling: Makes intelligent decisions on process execution based on priority, wait time, and CPU load.
Rust-Powered: Built with Rust to ensure memory safety and high-speed execution.
Efficiency: Dynamically tracks process wait times to prevent starvation and optimize system throughput.
🛠 Technical Overview
The scheduler calculates an AI Score for each process using the following heuristic model:
Priority Score: Based on the assigned priority level.
Wait Time: Rewards processes that have been waiting in the queue.
CPU Penalty: Penalizes processes with high CPU usage to maintain balance.
💻 How to Run 
1.Ensure you have Rust installed on your system.
2.Clone this repository:
 bash
git clone https://github.com/dsoma5749-commits/akira_kernel.git
cd akira_kernel
3.Build and run the project:
cargo run
📜 License
This project is open-source under the MIT License.
🤝 Contribution
Contributions are welcome! If you have ideas for new features or optimizations, feel free to open an Issue or submit a Pull Request.