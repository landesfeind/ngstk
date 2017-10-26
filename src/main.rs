#![allow(dead_code,unused_must_use)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate clap;

mod io;
mod util;
mod model;
mod sequence;
mod alignment;
mod sketch;
mod tool;
use tool::Tool;

fn main() {
    match pretty_env_logger::init() {
        Err(why) => panic!("Can not initialize logging facility: {:?}", why),
        Ok(_) => {}
    }

    let mut app = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        //.setting(AppSettings::SubcommandRequired)
        .arg(clap::Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Sets the level of verbosity"))
        ;
    app = tool::Translate::subcommand("translate", app);
    app = tool::Sketch::subcommand("sketch", app);
    app = tool::FastaFormat::subcommand("fasta-format", app);

    match app.get_matches().subcommand() {
        ("translate", Some(sub_m)) => tool::Translate::run(sub_m),
        ("sketch", Some(sub_m)) => tool::Sketch::run(sub_m),
        ("fasta-format", Some(sub_m)) => tool::FastaFormat::run(sub_m),
        _ => {}
    }
}
