#![allow(dead_code)]
// আকিরা কার্নেলের AI Security Monitor
// সন্দেহজনক process detect করে kill বা flag করে

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    Safe,
    Suspicious,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ProcessActivity {
    pub id: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub syscall_rate: u32,   // প্রতি সেকেন্ডে কতটা system call
    pub network_access: bool, // network ব্যবহার করছে কিনা
}

pub struct AiSecurityMonitor {
    pub cpu_limit: f32,
    pub memory_limit: f32,
    pub syscall_limit: u32,
}

impl AiSecurityMonitor {
    pub fn new() -> Self {
        AiSecurityMonitor {
            cpu_limit: 80.0,
            memory_limit: 70.0,
            syscall_limit: 1000,
        }
    }

    // AI threat score calculate করে
    fn threat_score(&self, p: &ProcessActivity) -> f32 {
        let mut score = 0.0;

        // CPU বেশি খাচ্ছে?
        if p.cpu_usage > self.cpu_limit {
            score += (p.cpu_usage - self.cpu_limit) * 1.5;
        }

        // Memory বেশি খাচ্ছে?
        if p.memory_usage > self.memory_limit {
            score += (p.memory_usage - self.memory_limit) * 1.2;
        }

        // System call অনেক বেশি?
        if p.syscall_rate > self.syscall_limit {
            score += (p.syscall_rate - self.syscall_limit) as f32 * 0.01;
        }

        // Network access suspicious হতে পারে
        if p.network_access {
            score += 10.0;
        }

        score
    }

    // Threat level নির্ধারণ করে
    fn classify(&self, score: f32) -> ThreatLevel {
        if score < 15.0 {
            ThreatLevel::Safe
        } else if score < 40.0 {
            ThreatLevel::Suspicious
        } else {
            ThreatLevel::Critical
        }
    }

    // প্রতিটা process analyze করে সিদ্ধান্ত নেয়
    pub fn analyze(&self, process: &ProcessActivity) {
        let score = self.threat_score(process);
        let level = self.classify(score);

        match level {
            ThreatLevel::Safe => {
                println!(
                    "✅ SAFE     | {} | Score: {:.1}",
                    process.name, score
                );
            }
            ThreatLevel::Suspicious => {
                println!(
                    "⚠️  FLAGGED  | {} | Score: {:.1} | Monitoring...",
                    process.name, score
                );
            }
            ThreatLevel::Critical => {
                println!(
                    "🚨 CRITICAL  | {} | Score: {:.1} | KILLING PROCESS!",
                    process.name, score
                );
                self.kill_process(process);
            }
        }
    }

    fn kill_process(&self, process: &ProcessActivity) {
        println!(
            "💀 Process '{}' (ID:{}) terminated by Akira AI Security!",
            process.name, process.id
        );
    }

    pub fn scan_all(&self, processes: &Vec<ProcessActivity>) {
        println!("\n🔍 Akira AI Security Scan Started...\n");
        for p in processes {
            self.analyze(p);
        }
        println!("\n✅ Scan Complete.");
    }
}
