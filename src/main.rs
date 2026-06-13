use std::fs; 

#[derive(Debug, Default)]
struct CPU {
    user: i32, // time in user mode (regular user progams)
    nice: i32, // user mode (this runs de-prioritized user program) 
    system: i32, // kernel mode (running syscalls on behalf of the kernel)
    idle: i32, // time spent doing nothing, literally. 
    iowait: i32 // idle waiting for I/O (blocked because of I/O or disk task) 
} 

fn main() {
    let contents = fs::read_to_string("/proc/stat")
        .expect("Should be a valid path");
    
    let mut cpus: Vec<CPU> = Vec::new();
    let mut aggregate_cpu = CPU::default();

    for line in contents.lines() {
        if !line.starts_with("cpu") {
            break 
        }
        
        let fields: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if line.starts_with("cpu") && line.chars().nth(3).is_some_and(|c| c.is_ascii_digit()) {
            let cpu = CPU{
                user: fields[0],
                nice: fields[1],
                system: fields[2],
                idle: fields[3],
                iowait: fields[4]
            };

            cpus.push(cpu);
        }
        else {
            aggregate_cpu.user = fields[0];
            aggregate_cpu.nice = fields[1];
            aggregate_cpu.system = fields[2];
            aggregate_cpu.idle = fields[3];
            aggregate_cpu.iowait = fields[4];
        }
        
    }
    
    println!("Aggregated CPU: \n{:#?}", aggregate_cpu);

    println!("\nCPU per cores: ");
    for (i, line) in cpus.iter().enumerate() {
        println!("{i}. {:#?}", line);
    }

    
    // println!("Path content: \n{}", contents);
}
