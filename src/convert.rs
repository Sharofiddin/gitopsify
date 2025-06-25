use duct::cmd;
use std::fs;
use anyhow::Result;
use crate::fluxgen;
use serde_json;

pub fn run(namespace: &str, chart: &str, release: &str, output_path: &str, repo_url: &str) -> Result<()> {
    println!("Converting Helm release '{}' in namespace '{}'", chart, namespace);

    let raw_values = cmd!("helm", "get", "values", release, "-n", namespace).read()?;
    println!("✅ Retrieved Helm values");
    
    let cleaned_values = clean_helm_values(&raw_values);

    let version = extract_chart_version(namespace, release)?;
    fs::create_dir_all(output_path)?;
    let helmrepo_yaml = fluxgen::generate_helmrepo(chart,namespace, repo_url);
    let helmchart_yaml = fluxgen::generate_helmchart(chart, namespace, &version);
    let helmrelease_yaml = fluxgen::generate_helmrelease(chart, release, namespace, &cleaned_values);
    let all = format!("{helmrepo_yaml}\n---\n{helmchart_yaml}\n---\n{helmrelease_yaml}");
    fs::write(format!("{}/{}_helmresources.yaml", output_path, chart), all)?;
    println!("Wrote {}_helmresources.yaml to '{}'", chart, output_path);
    Ok(())
}

fn extract_chart_version(namespace: &str, release_name: &str) ->Result<String> {
    let json = cmd!("helm", "list", "-n", namespace, "-o", "json").read()?;
    let list: Vec<serde_json::Value> = serde_json::from_str(&json)?;

    for item in list {
        if item["name"] == release_name {
            let chart_str = item["chart"].as_str().unwrap_or("");
            if let Some(ver) = chart_str.rsplitn(2, "-").next() {
                return Ok(ver.to_string());
            }
        }
    }
    Err(anyhow::anyhow!(
            "Could not find chart version for release '{}'", release_name))
}

fn clean_helm_values(input: &str) -> String {
    let mut lines = input.lines();

    if let Some(first) = lines.next() {
        if first.trim().to_ascii_uppercase().contains("VALUES") {
            return lines.collect::<Vec<&str>>().join("\n");
        }
    }
        input.to_string()
        }
