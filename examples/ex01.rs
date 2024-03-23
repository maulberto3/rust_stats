use anyhow::Result;
use csv::ReaderBuilder;
use reqwest::blocking::get;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
}

fn fetch_dataset(url: &str) -> Result<String> {
    let mut response = get(url)?;
    // println!("{:?}", &response.status());
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    //
    // dbg!(&content[..1000]);
    //
    Ok(content)
}

fn load_dataset(csv_data: &str) -> Result<Vec<SalaryRecord>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: SalaryRecord = result?;
        // if i == 0 { dbg!(&record); }
        records.push(record);
    }
    // println!("{:?}", &records);
    dbg!(&records[0]);
    Ok(records)
}

fn filter_and_convert(dataset: &[SalaryRecord]) -> Result<Vec<(i32, String, f64)>> {
    let data = dataset
        .iter()
        .filter(|record| record.experience_level == "SE")
        .map(|record| {
            let salary_in_usd_rounded = record.salary_in_usd.round();
            (
                record.work_year,
                record.job_title.to_string(),
                salary_in_usd_rounded,
            )
        })
        .collect();
    Ok(data)
}

fn main() {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";

    match fetch_dataset(url) {
        Ok(csv_data) => {
            match load_dataset(&csv_data) {
                Ok(dataset) => {
                    // data ready
                    println!("Loaded {} records", dataset.len());
                    // filter data
                    let parsed_data = filter_and_convert(&dataset);
                    println!("Filtered and converted data: {:?}", parsed_data.unwrap());
                }
                Err(error) => {
                    eprint!("Error loading dataset: {}", error)
                }
            }
        }
        Err(error) => {
            eprint!("Error fetching dataset: {}", error)
        }
    }
}
