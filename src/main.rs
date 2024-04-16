extern crate csv;

use std::error::Error;
use std::mem;
use std::process;

// **********************************************************************************************************************
// Calculate a series of Pythagorean Triplets based on the A^2 + B^2 = C^2 formula.
// Use the M and N values to calculate the values of A,B, and C as integers instead of doing floating point calculations.
// Paul T. Saletzki
// March 16, 2024
// **********************************************************************************************************************
fn main() {
    println!("Calculate Pythagorean Triplets.");

    const MAX_NUM: u16 = 30;
    let mut cnt: u16 = 0;

    //let mut pyth: [[u16; 200]; 4];
    let mut pyths = [[0; 3]; 440];

    // Define your 2-dimensional array with 3 integers as columns
    let mut data: Vec<[u16; 3]> = Vec::new();

    for m in 1..=MAX_NUM {
        for n in 1..m {
            let mut a = calc_a(m, n);
            let mut b = calc_b(m, n);
            let c = calc_c(m, n);

            // If A is greater than B then swap the values to order the numbers in ascending order in the list.
            if a > b {
                mem::swap(&mut a, &mut b);
            }

            // Store into a 3 Dimension vector for writting to file.
            data.push([a, b, c]);

            // Store in 2 dimensional array to see how arrays work in Rust.
            pyths[cnt as usize][0] = a;
            pyths[cnt as usize][1] = b;
            pyths[cnt as usize][2] = c;

            cnt += 1;
        }
    }

    // remove zeros out of vector array.
    for lin in data.len()..1 {
        if data[lin][0] == 0 && data[lin][1] == 0 {
            data.remove(lin);
        }
    }

    // Sort the 2 dimensional array of values by A and B.
    pyths.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Sort the Vector array of triplets by A and B ascending.
    data.sort_by(|a, b| {
        if a[0] != b[0] {
            a[0].cmp(&b[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Print the output.
    print_output(pyths);

    let csv_file_path = "Pythagorean_Triplets.csv";

    // Attempt to write the data to the CSV file
    if let Err(err) = write_to_csv(&data, csv_file_path) {
        eprintln!("Error writing to CSV file: {}", err);
        process::exit(1);
    }

    println!("Data has been successfully written to {}", csv_file_path);

    //write_to_file(pyths);
}

fn calc_a(m: u16, n: u16) -> u16 {
    return m * m - n * n;
}

fn calc_b(m: u16, n: u16) -> u16 {
    return 2 * m * n;
}

fn calc_c(m: u16, n: u16) -> u16 {
    return m * m + n * n;
}

fn print_output(lines: [[u16; 3]; 440]) {
    println!("#: A, B, C");

    for i in 0..lines.len() {
        if lines[i][0] > 0 {
            println!(
                "{}: {},{}, {}",
                i + 1,
                lines[i][0],
                lines[i][1],
                lines[i][2]
            );
        }
    }
}

fn write_to_csv(data: &Vec<[u16; 3]>, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a new CSV writer
    let mut writer = csv::Writer::from_path(file_path)?;

    let _result = writer.write_record(&["#", "A", "B", "C"]);

    let mut row_num: u16 = 0;
    // Write each row of data to the CSV file
    for row in data {
        row_num += 1;
        writer.write_record(&[
            row_num.to_string(),
            row[0].to_string(),
            row[1].to_string(),
            row[2].to_string(),
        ])?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}
