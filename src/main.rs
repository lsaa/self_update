// fuckign windles

use anyhow::Result;
use self_update::cargo_crate_version;
#[cfg(target_os = "linux")] use std::os::linux::fs::MetadataExt;

fn main() -> Result<()>{
    println!("this is {}", cargo_crate_version!());
    let _e = update()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn update() -> Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("lsaa")
        .repo_name("self_update")
        .bin_name("supdate")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

#[cfg(target_os = "linux")]
fn update() -> Result<()> {
    use std::{process::Command, env::current_exe};
    use std::os::unix::prelude::MetadataExt;

    let exe = current_exe()?;
    let has_write = {
        let metadata = exe.metadata();
        println!("exe path: {:?}", exe.to_str());
        if metadata.is_err() {
            false
        } else {
            let metadata = metadata?;
            let file_owner = metadata.uid();
            let user_id = std::fs::metadata("/proc/self").map(|m| m.uid())?;
            let mode = metadata.st_mode();
            println!("mode: {}", mode);
            let (r_user, w_user) = (mode & 0o000400 == 0o000400, mode & 0o000200 == 0o000200);
            println!("r: {r_user:}, w: {w_user:}");
            let (r_other, w_other) = (mode & 0o000004 == 0o000004, mode & 0o000002 == 0o000002);
            println!("r: {r_other:}, w: {w_other:}");
            (r_other && w_other) || ((file_owner == user_id) && (r_user && w_user))
        }
    };

    let instpath = exe.to_str().unwrap();
    println!("write access: {}", has_write);

    // Download and extract executable



    if has_write {
        let _status = self_update::backends::github::Update::configure()
            .repo_owner("lsaa")
            .repo_name("self_update")
            .bin_name("supdate")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
    } else {
        let _status = self_update::backends::github::Update::configure()
            .repo_owner("lsaa")
            .repo_name("self_update")
            .bin_name("supdate")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .bin_install_path("/tmp/supdate")
            .build()?
            .update()?;

        println!("install:");
        Command::new("sudo")
            .arg("install")
            .arg("-Dm775")
            .arg("/tmp/supdate")
            .arg("/usr/bin/supdate")
            .status()?;
    }
        
    println!("setcap:");
    Command::new("sudo")
        .arg("setcap")
        .arg("cap_sys_ptrace=eip")
        .arg(instpath)
        .status()?;

    Ok(())
}
