use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::process::Command;
use std::time::Instant;
use sys_info::mem_info;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }

    // Record the start time
    let start_time = Instant::now();
    // Load the CSV file
    let csv_file = "bmi.csv"; // The name of the CSV file
    let file = File::open(csv_file)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut max_bmi: f64 = 0.0; // Initialize max_bmi to a value that will be replaced

    for result in rdr.records() {
        let record = result?;
        let bmi: f64 = record[3].parse()?; // Index 3 is the BMI column based on the provided CSV structure
        if bmi > max_bmi {
            max_bmi = bmi; // Update max_bmi if the current value is greater than the previous maximum
        }
    }

    // Print the maximum BMI value
    println!("Max BMI: {}", max_bmi);

    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    println!(
        "Memory Usage: {}%",
        mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}
