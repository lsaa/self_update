// fuckign windles

use anyhow::Result;
use self_update::cargo_crate_version;

fn main() -> Result<()>{
    println!("this is {}", cargo_crate_version!());
    let _e = update()?;
    Ok(())
}

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
