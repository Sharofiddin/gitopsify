use duct::cmd;
use anyhow::Result;

pub fn run(namespace: &str) -> Result<()> {
    println!("Detecting Helm releases in namespace: {}", namespace);

    let output = cmd!("helm", "list", "-n", namespace, "--short").read()?;
    if output.trim().is_empty() {
        println!("No hem releases found in namespace '{}'.", namespace);
    } else { 
        println!("Found releases:\n{}", output);
    }

    Ok(())
}

