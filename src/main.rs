use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
fn file_write(res:&str){
let mut file=OpenOptions::new()
        .write(true).append(true).create(true).open("ip_scan.txt").expect("failed to open");
    writeln!(file,"{}",res);
}

fn rustscan(ip:&str){
     // Define the command you want to run
    let command = "rustscan";
    //println!("{}",ip);
    // Specify the command arguments
    let args = &[
        "-a",
        &format!("{}/24",ip),
        "-p",
        "22,80,21",
        "-g",
    ];
    // Start the timer

    // Use the Command struct to spawn a new process and execute the command
    let output = match Command::new(command)
        .args(args)
        .output() // Collect the output of the command
    {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute command: {}", e),
    };

    // End the timer

    // Calculate the elapsed time in seconds

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a string and print it
        let result = String::from_utf8_lossy(&output.stdout);
        if !result.trim().is_empty(){
        println!("Command output:\n{}", result);
        file_write(&result);
        }
    } else {
        // Print the error message if the command failed
        let result = String::from_utf8_lossy(&output.stderr);
        println!("Command failed:\n{}", result);
    }
}

fn generate_random_ip() -> Vec<String> {
    let mut rng=rand::thread_rng();
    let mut ip=Vec::new();
    for i in  0..=1000{
    let ip1: u8 = rng.gen_range(0..=230);
    let ip2: u8 = rng.gen_range(0..=255);

    ip.push(format!("{}.{}.0.0", ip1, ip2))
    }
    ip
}

fn main() {
    let mut random_ip = generate_random_ip();
    let pool=rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();
    //println!("Random IP: {:?}", random_ip);
    random_ip.into_par_iter().for_each(|x|rustscan(&x))
}

