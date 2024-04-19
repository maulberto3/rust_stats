use anyhow::Result;
// use csv::ReaderBuilder;
// use plotly::{common::Title, Histogram, Layout, Plot};
// use reqwest::blocking::get;
// use serde::Deserialize;
// use std::collections::{HashMap, HashSet};
// use std::io::Read;
// use std::vec;
use statrs::distribution::{Bernoulli, Binomial, Discrete, DiscreteCDF};

fn calc_bern_prob(p: f64) -> Result<f64> {
    let bern = Bernoulli::new(p).unwrap();
    let prob = bern.pmf(1);
    Ok(prob)
}

fn calc_binom_prob(p: f64, n: u64, x: u64) -> Result<f64> {
    let binom = Binomial::new(p, n).unwrap();
    let prob = binom.pmf(x);
    Ok(prob)
}

fn calc_binom_prob_less(p: f64, n: u64, x: u64) -> Result<f64> {
    let binom = Binomial::new(p, n).unwrap();
    let prob = binom.cdf(x);
    Ok(prob)
}

fn main() {
    let prob = calc_bern_prob(0.6).unwrap();
    println!("Probability of P(X = 1) given p = 0.6: {}", prob);
    
    let (p, n, x) = (0.6, 10, 8);
    let prob = calc_binom_prob(p, n, x).unwrap();
    println!("Probab. of getting {} dunks in {} throws given that I dunk {} of them: {}", x, n, p, prob);
    let prob = calc_binom_prob_less(p, n, x).unwrap();
    println!("Probab. of getting {} dunks, or less, in {} throws given that I dunk {} of them: {}", x, n, p, prob);
}
