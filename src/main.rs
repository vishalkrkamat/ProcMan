use std::fs;
use std::io;

struct Process {
    pid: String,
    status: Status,
}
struct Status {
    name: String,
    memory: u64, //VmRss
    threads: u32,
}

impl Process {
    fn process_list() -> Result<Vec<Process>, io::Error> {
        let dir = "/proc/";
        let mut processes = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if name_str.parse::<u64>().is_ok() {
                            if let Ok(status) = Status::parse(name_str) {
                                processes.push(Process {
                                    pid: name_str.to_string(),
                                    status,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(processes)
    }
}
impl Status {
    fn parse(pid: &str) -> Result<Status, io::Error> {
        let dir_path = format!("/proc/{pid}/status");
        let content = fs::read_to_string(&dir_path)?;
        let mut name = String::new();
        let mut memory: u64 = 0;
        let mut threads: u32 = 0;
        for line in content.lines() {
            if line.starts_with("Name") {
                name = line.split_whitespace().nth(1).unwrap_or("").to_string();
            } else if line.starts_with("VmRSS:") {
                memory = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
            } else if line.starts_with("Threads:") {
                threads = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
            }
        }
        Ok(Status {
            name,
            memory,
            threads,
        })
    }
    fn display_memory(&self) -> String {
        if self.memory > 1024 {
            format!("{} MB", self.memory / 1024) // Convert to MB if > 1024 KB
        } else {
            format!("{} KB", self.memory) // Keep in KB
        }
    }
}
fn main() {
    match Process::process_list() {
        Ok(processes) => {
            println!("Number of processes: {}", processes.len());

            for process in processes {
                println!(
                    "PID: {} | Name: {} | Memory: {} | Threads: {}",
                    process.pid,
                    process.status.name,
                    process.status.display_memory(),
                    process.status.threads
                );
            }
        }
        Err(e) => {
            eprintln!("Error reading process list: {}", e);
        }
    };
}
