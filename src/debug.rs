#[cfg(debug_assertions)]
use clap::ArgEnum;
#[cfg(debug_assertions)]
use colored::Colorize;

/// Debugging module
/// Enumerates possible debug levels
#[cfg(debug_assertions)]
#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum Level {
	N, // No debug information shown
	E, // Only show errors
	W, // Only show errors and warnings
	A, // Show all debug info
}

/// Handles debug messages
#[cfg(debug_assertions)]


#[derive(Clone, Copy)]
pub struct Messages {
	debug_level: Level
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
impl Messages {
	/// Creates a new Messages object.
	/// Manages which messages will be shown
	///
	/// * `debug_level` - which level of verbosity should be used
	pub fn new(debug_level: Level) -> Messages {
		Messages{debug_level}
	}

	/// Error debug message
	pub fn error(&self, msg: &str) {
		match self.debug_level {
			Level::E | Level::W | Level::A => {
				let err = "Error: ".red().bold();
				let str = msg.red();
				println!("{err}{str}");
			},
			_ => ()
		}
	}

	/// Warning debug message
	pub fn warning(&self, msg: &str) {
		match self.debug_level {
			Level::W | Level::A => {
				let warn = "Warning: ".yellow().bold();
				let str = msg.yellow();
				println!("{warn}{str}");
			},
			_ => ()
		}
	}

	/// Info debug message
	pub fn info(&self, msg: &str) {
		match self.debug_level {
			Level::A => {
				let info = "Info: ".purple().bold();
				let str = msg.purple();
				println!("{info}{str}");
			},
			_ => ()
		}
	}
}

/// Set debug level
/// Especially useful for a global debug::Messages object
#[macro_export]
macro_rules! debug_level {
	($dbg: expr, $level: expr) => {
		#[cfg(debug_assertions)]
		unsafe {
			$dbg = Some(debug::Messages::new($level));
		}
	};
}

/// Macro for calling error method with a guard for debug builds
#[macro_export]
macro_rules! err {
	($dbg: expr, $msg: expr) => {
		#[cfg(debug_assertions)]
		unsafe {
			$dbg.expect("No debug::Messages object set").error($msg);
		}
	};
}

/// Macro for calling warning method with a guard for debug builds
#[macro_export]
macro_rules! warn {
	($dbg: expr, $msg: expr) => {
		#[cfg(debug_assertions)]
		unsafe {
			$dbg.expect("No debug::Messages object set").warning($msg);
		}
	};
}

/// Macro for calling info method with a guard for debug builds
#[macro_export]
macro_rules! info {
	($dbg: expr, $msg: expr) => {
		#[cfg(debug_assertions)]
		unsafe {
			$dbg.expect("No debug::Messages object set").info($msg);
		}
	};
}
