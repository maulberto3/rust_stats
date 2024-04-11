use anyhow::Result;
use csv::ReaderBuilder;
use plotly::{common::Title, Histogram, Layout, Plot};
use reqwest::blocking::get;
use serde::Deserialize;
// use std::collections::{HashMap, HashSet};
use std::io::Read;
// use std::vec;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f32,
    salary_currency: String,
    salary_in_usd: f32,
    employee_residence: String,
    remote_ratio: f32,
    company_location: String,
    company_size: String,
}

fn fetch_dataset(url: &str) -> Result<String> {
    let mut response = get(url)?;
    // println!("{:?}", &response.status());
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    // dbg!(&content[..1000]);
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

// fn calc_mean(data: &Vec<f32>) -> Result<f32> {
//     let sum: f32 = data.iter().sum();
//     let res: f32 = sum / data.len() as f32;
//     // dbg!(&res);
//     Ok(res)
// }

// fn calc_median(data: &mut Vec<f32>) -> Result<f32> {
//     data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
//     let len = data.len();
//     if len % 2 == 0 {
//         let mid1 = data[(len / 2) - 1];
//         let mid2 = data[len / 2];
//         Ok((mid1 + mid2) / 2.0)
//     } else {
//         Ok(data[len / 2])
//     }
// }

// fn calc_std(col: &Vec<f32>) -> Result<f32> {
//     let mean = calc_mean(&col)?;
//     let len = col.len();
//     let num: f32 = col
//         .iter()
//         .map(|&value| (value - &mean).powi(2))
//         .sum::<f32>();
//     let std = (num / ((len - 1) as f32)).powf(0.5);
//     Ok(std)
// }

// fn standardize_salary(dataset: &[SalaryRecord]) -> Result<Vec<f32>> {
//     let mean = calc_mean(&dataset.iter().map(|record| record.salary_in_usd).collect()).unwrap();
//     let std = calc_std(&dataset.iter().map(|record| record.salary_in_usd).collect()).unwrap();
//     let std_col = dataset
//         .iter()
//         .map(|record| (record.salary_in_usd - mean) / std)
//         .collect();
//     Ok(std_col)
// }

// fn create_job_title_map(dataset: &[SalaryRecord]) -> Result<HashMap<String, usize>> {
//     let mut job_title_set: HashSet<String> = dataset
//         .iter()
//         .map(|record| record.job_title.clone())
//         .collect();

//     let mut job_title_mapping: HashMap<String, usize> = HashMap::new();
//     for (i, job_title) in job_title_set.drain().enumerate() {
//         job_title_mapping.insert(job_title, i);
//     }
//     Ok(job_title_mapping)
// }

// fn one_hot_encode_job_titles(dataset: &[SalaryRecord],
//                             mapping: &HashMap<String, usize>) -> Result<Vec<Vec<i32>>>{
//     let res: Vec<Vec<i32>> = dataset
//     .iter()
//     .map(|record| {
//         let mut encoding = vec![0; mapping.len()];
//         let index = mapping[&record.job_title];
//         encoding[index] = 1;
//         encoding
//     })
//     .collect();
//     Ok(res)
// }

// fn create_us_based_feature( dataset: &[SalaryRecord]) -> Result<Vec<usize>> {
//     let res: Vec<usize>= dataset
//     .iter()
//     .map(|record| if record.company_location == "US" { 1 } else { 0 })
//     .collect();
//     Ok(res)
// }

// fn filter_and_convert(dataset: &[SalaryRecord]) -> Result<Vec<(i32, String, f32)>> {
//     let data = dataset
//         .iter()
//         .filter(|record| record.experience_level == "SE")
//         .map(|record| {
//             let salary_in_usd_rounded = record.salary_in_usd.round();
//             (
//                 record.work_year,
//                 record.job_title.to_string(),
//                 salary_in_usd_rounded,
//             )
//         })
//         .collect();
//     Ok(data)
// }

// fn range(col: &Vec<f32>) -> Result<f32> {
//     let max = col
//         .iter()
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();

//     let min = col
//         .iter()
//         .min_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     Ok(max - min)
// }

fn plot_histogram(col: &[f32], bins: usize, title: &str) {
    let trace = Histogram::new(col.into())
        .n_bins_x(bins)
        .name("Salary Distribution");
    // .marker(value)

    let layout = Layout::new().title(Title::new(&title));
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    // plot.show();
    plot.write_html("histogram_render.html".to_string());
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
                    println!("");

                    let salary_data: &Vec<f32> =
                        &dataset.iter().map(|record| record.salary_in_usd).collect();

                    plot_histogram(&salary_data, 20, "Salary Distribution in USD");
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
