/// Command line interface for this program
use clap::Parser;

#[cfg(debug_assertions)]
use crate::debug::Level;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	// REQUIRED

	/// Path to the csv file to use
	#[clap()]
	pub file: String,

	// OPTIONAL

	/// Sets debug level
	///
	/// N: No info displayed
	///
	/// E: Errors only
	///
	/// W: Warnings and Errors
	///
	/// A: All Debug info shown
	#[cfg(debug_assertions)]
	#[clap(short, long, arg_enum, default_value_t=Level::N)]
	pub debug: Level,

	/// Enable learning mode
	#[clap(short, long)]
	pub learn: bool,

	/// Enable prediction mode
	#[clap(short, long)]
	pub predict: bool,
}
