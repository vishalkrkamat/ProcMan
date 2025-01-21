# Process Monitor (ps Command Implementation in Rust)

This Rust program is a basic implementation of the `ps` command, which lists running processes on a Linux system by reading the `/proc` filesystem.

## Implementation Overview

The program performs the following tasks:

1. **Read process directories from `/proc`**, filtering numeric PIDs.  
2. **Parse `/proc/[pid]/status`** to extract:  
   - **Name** (process name)  
   - **VmRSS** (memory usage in KB)  
   - **Threads** (number of threads)  
3. **Display process details** including PID, name, memory usage, and thread count.

## Code Structure

- **`Process` struct:** Represents a process and gathers process information.
- **`Status` struct:** Parses and stores process details from `/proc/[pid]/status`.
- **Functions:**
  - `Process::process_list()`: Reads processes from `/proc`.
  - `Status::parse(pid)`: Parses status file for details.
  - `Status::display_memory()`: Formats memory output.


