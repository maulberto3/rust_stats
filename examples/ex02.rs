use anyhow::Result;
use csv::{Reader, ReaderBuilder};
use reqwest::Response;
use serde::de;

async fn fetch_data() -> Result<Vec<f32>> {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";
    let response = reqwest::get(url).await?.text().await?;
    let mut reader = ReaderBuilder::new().from_reader(response.as_bytes());
    let mut salaries = Vec::new();

    for (i, result) in reader.records().enumerate() {
        let record = result?;
        if i == 0 {
            dbg!(&record);
        }
        let salary: f32 = record.get(0).unwrap().parse().unwrap();
        salaries.push(salary)
    }
    // dbg!(&salaries);
    Ok(salaries)
}

fn calc_mean(data: &Vec<f32>) -> Result<f32> {
    let sum: f32 = data.iter().sum();
    let res: f32 = sum / data.len() as f32;
    // dbg!(&res);
    Ok(res)
}

fn calc_median(data: &mut Vec<f32>) -> Result<f32> {
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let len = data.len();
    if len % 2 == 0 {
        let mid1 = data[(len / 2) - 1];
        let mid2 = data[len / 2];
        Ok((mid1 + mid2) / 2.0)
    } else {
        Ok(data[len / 2])
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut salaries = fetch_data().await?;
    let mean_salaries = calc_mean(&salaries).unwrap();
    dbg!(&mean_salaries);
    let median_salaries = calc_median(&mut salaries).unwrap();
    dbg!(&median_salaries);
    Ok(())
}
