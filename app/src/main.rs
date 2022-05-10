//
mod http;
use anyhow::Result;
use structopt::{clap::{self}, StructOpt};
use http::HttpModule;

#[derive(Debug, StructOpt)]
#[structopt(name = "create_email")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(name = "file_path")]
    pub file_path: String,
    #[structopt(name = "credential")]
    pub credential: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut module = HttpModule::new(opt);

    module.run().unwrap();
    module.print_user().unwrap();

    Ok(())
}
