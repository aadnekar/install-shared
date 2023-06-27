use std::{env, path::Path, process::{Command, exit, abort}};
use clap::Parser;
use install_shared::options::SharedOptions;

use anyhow::Result;

fn main() -> Result<()> {
    let opts: SharedOptions = SharedOptions::parse().try_into()?;
    println!("{:?}", opts);

    let shared_workspace = env::var("SharedWorkspace").expect("Environment variable SharedWorkSpace not set.\nYou need to set up an environment variable called\"SharedWorkspace\" with path to pc-nrfconnect-shared.");

    println!("Shared workspace: {}", shared_workspace);

    if opts.skip_types {
        println!("Will attempt to run npm generate-types in shared workspace");

        let generate_types_output = Command::new("npm").current_dir(shared_workspace.clone()).args(&["run", "generate-types"]).output().expect("failed to execute process");
        if generate_types_output.status.success() {
            println!("Generated types successfully");
        } else {
            println!("Failed to generate types");
        }
    }

    println!("Will attempt to run npm pack in shared workspace");
    let pack_output = Command::new("npm").current_dir(shared_workspace.clone()).arg("pack").output().expect("failed to execute process");

    println!("Packed file: {}", String::from_utf8_lossy(&pack_output.stdout));
    if !pack_output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&pack_output.stderr));
        abort();
    }
    let tarball = String::from_utf8_lossy(&pack_output.stdout).trim().to_string();
    //let tarball = "pc-nrfconnect-shared-62.0.0.tgz";
    println!("Tarball: {}", tarball);
    
    let shared_path = Path::new(&shared_workspace);
    let tarball_path = Path::new(&tarball);
    println!("Shared path: {}", shared_path.display());
    println!("Tarball path: {}", tarball_path.display());

    let tarball = Path::join(shared_path, tarball);
    if !tarball.exists() {

    }
    println!("Tarball exists");
    let tarball = tarball.to_str().unwrap();
    let npm_install = Command::new("npm").args(&["install", tarball]).output().expect("Failed to install {shared_workspace}{tarball}");
    if npm_install.status.success() {
        println!("Installed {tarball} successfully");
        exit(0);
    }

    println!("Failed to install {tarball}");
    println!("stderr: {}", String::from_utf8_lossy(&npm_install.stderr));
    return Ok(());
}
