use std::fs;
use std::io;
use std::path::Path;
use std::time;

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
    fn process_list() -> Result<Vec<String>, io::Error> {
        let dir = "/proc/";
        let mut procid = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if name_str.parse::<u64>().is_ok() {
                            procid.push(name_str.to_string());
                        }
                    }
                }
            }
        }
        Ok(procid)
    }
}

fn main() {
    match Process::process_list() {
        Ok(procid) => {
            let no_process = procid.len();
            println!("Process IDs found: {:?}", procid);
            println!("NO of process: {no_process}");
            for id in procid.iter() {
                status(id.to_string());
            }
        }
        Err(_) => eprintln!("Error reading process list:"),
    };
}
fn status(id: String) -> io::Result<()> {
    let dir_path = format!("/proc/{}/status", id);
    let read = fs::read_to_string(dir_path);
    println!("{read:?}");
    Ok(())
}
