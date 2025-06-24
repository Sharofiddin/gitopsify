use duct::cmd;
use std::fs;
use anyhow::Result;
use crate::fluxgen;

pub fn run(namespace: &str, chart: &str, output_path: &str, repo_url: &str) -> Result<()> {
    println!("Converting Helm release '{}' in namespace '{}'", chart, namespace);

    let values = cmd!("helm", "get", "values", chart, "-n", namespace).read()?;
    println!("Retrieved Helm values");

    fs::create_dir_all(output_path)?;
    let helmrelease_yaml = fluxgen::generate_helmrelease(chart, namespace, &values);
    fs::write(format!("{}/values.yaml", output_path), values)?;
    println!("Wrote values.yaml to '{}'", output_path);
    fs::write(format!("{}/helmrelease.yaml", output_path), helmrelease_yaml)?;
    println!("Wrote helmrelease.yaml to '{}'", output_path);
    let helmrepo_yaml = fluxgen::generate_helmrepo(chart,namespace, repo_url);
    fs::write(format!("{}/helmrepository.yaml", output_path), helmrepo_yaml)?;
    println!(" Wrote helmrepository.yaml to '{}'", output_path);
    Ok(())
}
