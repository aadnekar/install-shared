use anyhow::Result;
use std::{
    env,
    path::Path,
    process::{abort, Command},
};

pub fn install(generate_types: bool) -> Result<()> {
    let shared_workspace = env::var("SharedWorkspace").expect("Environment variable SharedWorkSpace not set.\nYou need to set up an environment variable called\"SharedWorkspace\" with path to pc-nrfconnect-shared.");

    println!("Installing pc-nrfconnect-shared.");

    if generate_types {
        println!("Generating types.");

        let generate_types_output = Command::new("npm")
            .current_dir(shared_workspace.clone())
            .args(["run", "generate-types"])
            .output()
            .expect("failed to execute process");
        if generate_types_output.status.success() {
            println!("Types generated.");
        } else {
            println!("Failed to generate types.");
        }
    }

    println!("Packing pc-nrfconnect-shared.");
    let pack_output = Command::new("npm")
        .current_dir(shared_workspace.clone())
        .arg("pack")
        .output()
        .expect("failed to execute process");

    println!("Packed: {}", String::from_utf8_lossy(&pack_output.stdout));
    if !pack_output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&pack_output.stderr));
        abort();
    }
    let tarball = String::from_utf8_lossy(&pack_output.stdout)
        .trim()
        .to_string();

    let tarball = Path::join(Path::new(&shared_workspace), Path::new(&tarball));
    if !tarball.exists() {
        anyhow::bail!("Failed to find tarball: {}", tarball.display());
    }

    let tarball = tarball.to_str().unwrap();
    let npm_install = Command::new("npm")
        .args(["install", tarball])
        .output()
        .expect("Failed to install {shared_workspace}{tarball}");
    if npm_install.status.success() {
        println!("Installed {tarball}");
        return Ok(());
    }

    println!("Failed to install {tarball}");
    println!("stderr: {}", String::from_utf8_lossy(&npm_install.stderr));
    println!("Install complete");
    Ok(())
}
