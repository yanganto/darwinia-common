// --- crates ---
use structopt::StructOpt;
// --- substrate ---
use sc_cli::RunCmd;

#[derive(Debug, StructOpt)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub run: RunCmd,
}

#[derive(Clone, Debug, StructOpt)]
pub enum Subcommand {
	/// A set of base subcommands handled by `sc_cli`.
	#[structopt(flatten)]
	Base(sc_cli::Subcommand),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
