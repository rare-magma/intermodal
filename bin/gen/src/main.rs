use crate::common::*;

#[macro_use]
mod cmd;

mod bin;
mod changelog;
mod command_ext;
mod common;
mod config;
mod entry;
mod example;
mod exit_status_ext;
mod introduction;
mod kind;
mod metadata;
mod opt;
mod package;
mod project;
mod readme;
mod release;
mod row;
mod subcommand;
mod summary;
mod table;

#[throws]
fn main() {
  pretty_env_logger::init();

  let project = Project::load()?;

  Opt::from_args().run(&project)?;
}