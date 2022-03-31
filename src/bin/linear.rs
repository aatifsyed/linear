#![feature(rustc_private)]
extern crate rustc_driver;

fn main() -> anyhow::Result<()> {
    rustc_driver::install_ice_hook();
    Ok(())
}
