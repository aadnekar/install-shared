use anyhow::Result;
use std::process::Command;

pub fn install(version: &str) -> Result<()> {
    let version = version.replace('v', "");

    println!("Installing pc-nrfconnect-shared v{version}...");

    let result = Command::new("npm")
        .args([
            "install",
            &format!(
                "github:NordicSemiconductor/pc-nrfconnect-shared#v{}",
                version
            ),
        ])
        .output()?;

    if !result.status.success() {
        anyhow::bail!(
            "Failed to install pc-nrfconnect-shared: {:?}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    println!("Installed pc-nrfconnect-shared v{version}");

    Ok(())
}

pub fn get_versions(limit: usize) -> Result<Vec<String>> {
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

    if result.len() > limit {
        let head = result.len() - limit;
        result = result.splice(head..result.len(), vec![]).collect();
    }

    Ok(result)
}

pub fn list_versions(number: usize) -> Result<()> {
    let versions = get_versions(number)?;
    println!(
        "Listing the last {} versions from github...",
        versions.len()
    );
    let padding = 10;
    let columns = 5;
    for (i, v) in versions.iter().enumerate() {
        let v = format!("v{}", v);
        let number_of_chars = v.len();
        let padding_left = (padding - number_of_chars) / 2;
        let padding_right = if number_of_chars % 2 == 1 {
            padding_left + 1
        } else {
            padding_left
        };
        // |12PAIR12| vs |12ODD123|
        // Here: padding = 4
        // So, padding_left = 4 - 4/2 = 2
        // and, padding_right = 2 + 1

        print!(
            "|{}{}{}",
            " ".repeat(padding_left),
            v,
            " ".repeat(padding_right)
        );

        if (i + 1) % columns == 0 {
            println!("|");
        }
    }
    if versions.len() % columns != 0 {
        println!("|");
    }
    Ok(())
}
