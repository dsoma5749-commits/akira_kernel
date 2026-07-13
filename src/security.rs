#![allow(dead_code)]
use serde::{Deserialize};
use std::fs;

// --- সব স্ট্রাক্ট এবং এনাম ---
#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel { Safe, Suspicious, Critical }

#[derive(Debug, Clone)]
pub struct ProcessActivity {
    pub id: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub syscall_rate: u32,
    pub network_access: bool,
}

#[derive(Deserialize, Debug)]
pub struct Signature {
    pub name: String,
    pub threat_level: String,
}

pub struct AiSecurityMonitor {
    pub cpu_limit: f32,
    pub memory_limit: f32,
    pub syscall_limit: u32,
}

// --- একমাত্র এবং একটিই impl ব্লক ---
impl AiSecurityMonitor {
    pub fn new() -> Self {
        AiSecurityMonitor { cpu_limit: 80.0, memory_limit: 70.0, syscall_limit: 1000 }
    }

    pub fn check_database(&self, process_name: &str) -> bool {
        if let Ok(data) = fs::read_to_string("signatures.json") {
            if let Ok(signatures) = serde_json::from_str::<Vec<Signature>>(&data) {
                for sig in signatures {
                    if sig.name == process_name { return true; }
                }
            }
        }
        false
    }

    fn threat_score(&self, p: &ProcessActivity) -> f32 {
        let mut score = 0.0;
        if p.cpu_usage > self.cpu_limit { score += (p.cpu_usage - self.cpu_limit) * 1.5; }
        if p.memory_usage > self.memory_limit { score += (p.memory_usage - self.memory_limit) * 1.2; }
        if p.syscall_rate > self.syscall_limit { score += (p.syscall_rate - self.syscall_limit) as f32 * 0.01; }
        if p.network_access { score += 10.0; }
        score
    }

    fn classify(&self, score: f32) -> ThreatLevel {
        if score < 15.0 { ThreatLevel::Safe } 
        else if score < 40.0 { ThreatLevel::Suspicious } 
        else { ThreatLevel::Critical }
    }

    pub fn analyze(&self, process: &ProcessActivity) {
        if self.check_database(&process.name) {
            println!("🚨 DATABASE MATCH: {} is a known threat!", process.name);
            self.kill_process(process);
            return;
        }
        let score = self.threat_score(process);
        let level = self.classify(score);
        match level {
            ThreatLevel::Safe => println!("✅ SAFE     | {} | Score: {:.1}", process.name, score),
            ThreatLevel::Suspicious => println!("⚠️  FLAGGED  | {} | Score: {:.1} | Monitoring...", process.name, score),
            ThreatLevel::Critical => {
                println!("🚨 CRITICAL  | {} | Score: {:.1} | KILLING PROCESS!", process.name, score);
                self.kill_process(process);
            }
        }
    }

    fn kill_process(&self, process: &ProcessActivity) {
        println!("💀 Process '{}' (ID:{}) terminated by Akira AI Security!", process.name, process.id);
    }

    pub fn scan_all(&self, processes: &Vec<ProcessActivity>) {
        println!("\n🔍 Akira AI Security Scan Started...\n");
        for p in processes { self.analyze(p); }
        println!("\n✅ Scan Complete.");
    }
}
