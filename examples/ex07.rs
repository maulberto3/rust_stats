use anyhow::Result;
// use csv::ReaderBuilder;
// use plotly::{common::Title, Histogram, Layout, Plot};
// use reqwest::blocking::get;
// use serde::Deserialize;
// use std::collections::{HashMap, HashSet};
// use std::io::Read;
// use std::vec;
use statrs::distribution::{Uniform, Continuous, ContinuousCDF, Normal};

fn calc_unif_prob(min: f64, max: f64, x: f64) -> Result<f64> {
    let unif = Uniform::new(min, max).unwrap();
    let prob = unif.cdf(x);
    Ok(prob)
}

fn calc_normal_density(mean: f64, std: f64, x: f64) -> Result<f64> {
    let normal = Normal::new(mean, std).unwrap();
    let prob = normal.pdf(x);
    Ok(prob)
}

fn calc_normal_prob(mean: f64, std: f64, x: f64) -> Result<f64> {
    let normal = Normal::new(mean, std).unwrap();
    let prob = normal.cdf(x);
    Ok(prob)
}

fn main() {
    let prob = calc_unif_prob(0.0, 2.0, 0.5).unwrap();
    println!("Prob. of 0.5 or less (for uniform from 0 a 2): {}", prob);

    let prob = calc_normal_density(1.25, 0.2, 0.75).unwrap();
    println!("Density of children height at 0.75 meters: {}", prob);

    let prob = calc_normal_prob(1.25, 0.2, 0.9).unwrap();
    println!("Prob of children height at 0.90 meters or less: {}", prob);

}
