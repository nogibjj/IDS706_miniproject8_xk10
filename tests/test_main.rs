use csv::ReaderBuilder;
use max_finder::find_max;
use std::error::Error;
use std::fs::File; // Import the find_max function from the max_finder library

#[test]
fn test_bmi_max() -> Result<(), Box<dyn Error>> {
    let csv_file = "bmi.csv"; // Updated to point to bmi.csv
    let file = File::open(csv_file)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b',') // Assuming the delimiter is a comma
        .has_headers(true)
        .from_reader(file);

    let mut bmi_values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let bmi: f64 = record[3].parse()?; // Change index to 3 for BMI, assuming "Age,Height,Weight,Bmi,BmiClass"
        bmi_values.push(bmi);
    }

    // Find the maximum BMI value using the find_max function
    let max_bmi = find_max(&bmi_values);

    // Assert that the maximum BMI value is correct
    // Replace `expected_max_bmi` with the expected value for your test case
    assert!(max_bmi >= 66.30134998); // Or use a more precise condition if known

    Ok(())
}
