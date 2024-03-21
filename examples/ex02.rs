use anyhow::Result;
use csv::{Reader, ReaderBuilder};
use reqwest::Response;

async fn fetch_data() -> Result<Vec<f32>> {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";
    let response = reqwest::get(url).await?.text().await?;
    let mut reader = ReaderBuilder::new().from_reader(response.as_bytes());
    let mut salaries = Vec::new();

    for result in reader.records() {
        let record = result?;
        let salary: f32 = record.get(0).unwrap().parse().unwrap();
        salaries.push(salary)
    }
    // dbg!(&salaries);
    Ok(salaries)
}

fn calc_mean(data: &Vec<f32>) -> Result<f32>{
    let sum: f32 = data.iter().sum();
    let res: f32 = sum / data.len() as f32;
    dbg!(&res);
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<()> {
    let data = fetch_data().await?;
    let _x = calc_mean(&data);
    Ok(())
}
