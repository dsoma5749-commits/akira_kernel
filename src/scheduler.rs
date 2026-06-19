#![allow(dead_code)]
// আকিরা কার্নেলের AI-Driven প্রসেস শিডিউলার

#[derive(Debug, Clone)]
pub struct Process {
    pub id: u32,
    pub name: String,
    pub priority: u8,
    pub cpu_usage: f32,    // কতটা CPU ব্যবহার করছে
    pub wait_time: u32,    // কতক্ষণ অপেক্ষা করছে
    pub state: ProcessState,
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
}

pub struct AiScheduler {
    pub queue: Vec<Process>,
}

impl AiScheduler {
    pub fn new() -> Self {
        AiScheduler { queue: Vec::new() }
    }

    pub fn add_process(&mut self, process: Process) {
        println!("Process added: {}", process.name);
        self.queue.push(process);
    }

    // AI Score calculate করে — কোন process আগে চলবে
    fn ai_score(p: &Process) -> f32 {
        let priority_score = p.priority as f32 * 2.0;
        let wait_score = p.wait_time as f32 * 1.5;
        let cpu_penalty = p.cpu_usage * 0.5;

        priority_score + wait_score - cpu_penalty
    }

    pub fn next(&mut self) -> Option<Process> {
        if self.queue.is_empty() {
            return None;
        }

        // AI সবচেয়ে বেশি score-এর process বেছে নেয়
        let best_idx = self.queue
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                Self::ai_score(a)
                    .partial_cmp(&Self::ai_score(b))
                    .unwrap()
            })
            .map(|(i, _)| i)?;

        let mut process = self.queue.remove(best_idx);

        println!(
            "AI Decision → Running: {} | Score: {:.1} | Priority: {} | Wait: {}",
            process.name,
            Self::ai_score(&process),
            process.priority,
            process.wait_time
        );

        process.state = ProcessState::Running;

        // Wait time বাড়িয়ে দাও বাকিদের
        for p in &mut self.queue {
            p.wait_time += 1;
        }

        Some(process)
    }
}

