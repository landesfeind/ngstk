extern crate clap;

mod translate;
pub use self::translate::Translate;
mod sketch;
pub use self::sketch::Sketch;
mod fasta_format;
pub use self::fasta_format::FastaFormat;

pub trait Tool {

	fn subcommand<'a, 'b>(name: &str, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
		app.subcommand( 
			Self::args( clap::SubCommand::with_name(name))
		)
	}

	fn args<'a, 'b>(subapp: clap::App<'a, 'b>) -> clap::App<'a, 'b>;

	fn run(args: &clap::ArgMatches);

}