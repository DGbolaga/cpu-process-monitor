use std::fs; 
use std::f64;
use std::thread;
use std::time::Duration;


#[derive(Debug, Default)]
struct CPU {
    user: f64, // time in user mode (regular user progams)
    nice: f64, // user mode (this runs de-prioritized user program) 
    system: f64, // kernel mode (running syscalls on behalf of the kernel)
    idle: f64, // time spent doing nothing, literally. 
    iowait: f64 // idle waiting for I/O (blocked because of I/O or disk task) 
}

fn get_aggregate_percentage() -> Vec<f64> {
    let delta = get_delta();
    
    // total CPU
    let mut total: f64 = 0.0; // total 
    for x in &delta {
        total += x
    }

    let percentages = delta.iter().map(|val| val/total*100.0).collect();
    
    // per core CPU
    let mut per_core_total: Vec<f64> = vec![0.0; delta.len()];
    // for y in &delta
    return percentages 
}

fn get_delta() -> Vec<f64> {

    let (aggregate1, _per_core_vec1) = get_cpu_report();

    // sleep. I want to compute the delta/difference
    thread::sleep(Duration::from_secs(1));
    let (aggregate2, _per_core_vec2) = get_cpu_report();
    

    let mut delta: Vec<Vec<f64>> = vec![vec![0.0; 5]; 5];

    
    // push aggregate
    delta[0][0] = aggregate2.user - aggregate1.user;
    delta[0][1] = aggregate2.nice - aggregate1.nice;
    delta[0][2] = aggregate2.system - aggregate1.system;
    delta[0][3] = aggregate2.idle - aggregate1.idle;
    delta[0][4] = aggregate2.iowait - aggregate1.iowait;

    // for info in &delta {
    //     println!("{}: {}", info.0, info.1);
    // }
    // println!("{:?}", delta);

   return delta[0].clone(); 

}

fn get_cpu_report() -> (CPU, Vec<CPU>) {
    let contents = fs::read_to_string("/proc/stat")
        .expect("Should be a valid path");

    let mut cpus: Vec<CPU> = Vec::new();
    let mut aggregate_cpu = CPU::default();

    for line in contents.lines() {
        if !line.starts_with("cpu") {
            break 
        }
        
        let fields: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
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
    
    return (aggregate_cpu, cpus);
   

}

fn main() {
    let agg = get_aggregate_percentage();
    println!("CPU total: user {:.1}%  nice {:.1}%  sys {:.1}%  idle {:.1}%  iowait {:.1}%", agg[0], agg[1], agg[2], agg[3], agg[4]);

    
    
}
