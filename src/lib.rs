pub fn find_max(values: &Vec<f64>) -> f64 {
    if values.is_empty() {
        panic!("Input data is empty.");
    }

    let mut max_value = values[0];

    for &value in values.iter() {
        if value > max_value {
            max_value = value;
        }
    }

    max_value
}
