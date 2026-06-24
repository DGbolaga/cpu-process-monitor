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

fn get_percentage() -> Vec<Vec<f64>> {
    let delta = get_delta();
    let mut percentages = vec![vec![0.0; delta[0].len()]; delta.len()];

    // total CPU
    let total: f64 = delta[0].iter().sum();
    percentages[0] = delta[0].iter().map(|val| val/total*100.0).collect();
    

    // per core CPU
    for i in 1..delta.len() {
        let core_total: f64 = delta[i].iter().sum();
        for j in 0..delta[0].len() {
            percentages[i][j] = delta[i][j]/core_total * 100.0;
        }
    
    }

    //println!("{:?}", percentages);
    return percentages;
}

fn get_delta() -> Vec<Vec<f64>> {

    let (aggregate1, per_core_vec1) = get_cpu_report();

    // sleep. I want to compute the delta/difference
    thread::sleep(Duration::from_secs(1));
    let (aggregate2, per_core_vec2) = get_cpu_report();
    
    let n = per_core_vec2.len();
    let mut delta: Vec<Vec<f64>> = vec![vec![0.0; 5]; n+1];

    
    // push aggregate
    delta[0][0] = aggregate2.user - aggregate1.user;
    delta[0][1] = aggregate2.nice - aggregate1.nice;
    delta[0][2] = aggregate2.system - aggregate1.system;
    delta[0][3] = aggregate2.idle - aggregate1.idle;
    delta[0][4] = aggregate2.iowait - aggregate1.iowait;
    
    // push per core
    for j in 0..n {
        delta[j+1][0] = per_core_vec2[j].user - per_core_vec1[j].user;
        delta[j+1][1] = per_core_vec2[j].nice - per_core_vec1[j].nice;
        delta[j+1][2] = per_core_vec2[j].system - per_core_vec1[j].system;
        delta[j+1][3] = per_core_vec2[j].idle - per_core_vec1[j].idle;
        delta[j+1][4] = per_core_vec2[j].iowait - per_core_vec1[j].iowait;

    }
    //println!("{:?}", delta);

   return delta; 

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
    let agg = get_percentage();
    for i in 0..agg.len() {
        if i == 0 {
            println!("CPU total: user {:.1}%  nice {:.1}%  sys {:.1}%  idle {:.1}%  iowait {:.1}%", agg[i][0], agg[i][1], agg[i][2], agg[i][3], agg[i][4]);
        }
        else {
            println!("  cpu{}    user {:.1}%  nice {:.1}%  sys {:.1}%  idle {:.1}%  iowait {:.1}%", i-1, agg[i][0], agg[i][1], agg[i][2], agg[i][3], agg[i][4]);
        }
    }


    
    
}
