use anyhow::Result;
use std::process::Command;

pub fn install(version: &str) -> Result<()> {
    let version = version.replace('v', "");

    let result = Command::new("npm")
        .args([
            "install",
            &format!("github:NordicSemiconductor/pc-nrfconnect-shared#v{}", version),
        ])
        .output()?;

    if !result.status.success() {
        anyhow::bail!("Failed to install pc-nrfconnect-shared: {:?}", String::from_utf8_lossy(&result.stderr));
    }

    println!("Installed pc-nrfconnect-shared v{version}");

    Ok(())
}

pub fn get_versions(limit: Option<usize>) -> Result<Vec<String>> {
    let mut cmd = Command::new("git");
    let cmd = cmd.args([
        "ls-remote",
        "--exit-code",
        "--refs",
        "--tags",
        "https://github.com/NordicSemiconductor/pc-nrfconnect-shared/",
    ]);
    let result = cmd.output()?;
    if !result.status.success() {
        anyhow::bail!("Failed to list versions: {}", result.status.to_string());
    }

    let result = String::from_utf8_lossy(&result.stdout);
    let result = result.to_string();
    let result = result.trim().split('\n');

    let mut result: Vec<String> = result
        .into_iter()
        .map(|s| {
            s.split('/')
                .last()
                .unwrap_or("")
                .to_string()
                .replace('v', "")
        })
        .collect();
    result.sort_by(|a, b| {
        let a_list: Vec<&str> = a.split('.').collect();
        let b_list: Vec<&str> = b.split('.').collect();

        let a = a_list[0].parse::<u16>().unwrap_or(0);
        let b = b_list[0].parse::<u16>().unwrap_or(0);
        a.cmp(&b)
    });

    let limit = limit.unwrap_or(10);
    let head = result.len() - limit;
    let result = result.splice(head..result.len(), vec![]).collect();
    Ok(result)
}

pub fn list_versions(versions: Vec<&str>) -> Result<Vec<&str>> {
    for v in &versions {
        println!("{v}");
    }
    Ok(versions)
}
