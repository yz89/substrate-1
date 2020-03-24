// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

mod build_spec_cmd;
mod check_block_cmd;
mod export_blocks_cmd;
mod import_blocks_cmd;
mod purge_chain_cmd;
mod revert_cmd;
mod runcmd;

pub use crate::commands::build_spec_cmd::BuildSpecCmd;
pub use crate::commands::check_block_cmd::CheckBlockCmd;
pub use crate::commands::export_blocks_cmd::ExportBlocksCmd;
pub use crate::commands::import_blocks_cmd::ImportBlocksCmd;
pub use crate::commands::purge_chain_cmd::PurgeChainCmd;
pub use crate::commands::revert_cmd::RevertCmd;
pub use crate::commands::runcmd::RunCmd;
use crate::CliConfiguration;
use crate::Result;
use crate::SubstrateCLI;
use sc_client_api::execution_extensions::ExecutionStrategies;
use sc_network::config::NodeKeyConfig;
use sc_service::{
	config::DatabaseConfig, config::WasmExecutionMethod, PruningMode, Roles,
};
use sc_tracing::TracingReceiver;
use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;

/// All core commands that are provided by default.
///
/// The core commands are split into multiple subcommands and `Run` is the default subcommand. From
/// the CLI user perspective, it is not visible that `Run` is a subcommand. So, all parameters of
/// `Run` are exported as main executable parameters.
#[derive(Debug, Clone, StructOpt)]
pub enum Subcommand {
	/// Build a spec.json file, outputs to stdout.
	BuildSpec(BuildSpecCmd),

	/// Export blocks to a file.
	ExportBlocks(ExportBlocksCmd),

	/// Import blocks from file.
	ImportBlocks(ImportBlocksCmd),

	/// Validate a single block.
	CheckBlock(CheckBlockCmd),

	/// Revert chain to the previous state.
	Revert(RevertCmd),

	/// Remove the whole chain data.
	PurgeChain(PurgeChainCmd),
}

macro_rules! match_and_call {
	(fn $method:ident ( &self $(, $arg:ident : $ty:ty)* ) $(-> $result:ty)?) => {
		fn $method (&self, $($arg : $ty),*) $(-> $result)? {
			match self {
				Subcommand::BuildSpec(cmd) => cmd.$method($($arg),*),
				Subcommand::ExportBlocks(cmd) => cmd.$method($($arg),*),
				Subcommand::ImportBlocks(cmd) => cmd.$method($($arg),*),
				Subcommand::CheckBlock(cmd) => cmd.$method($($arg),*),
				Subcommand::Revert(cmd) => cmd.$method($($arg),*),
				Subcommand::PurgeChain(cmd) => cmd.$method($($arg),*),
			}
		}
	};

	(fn $method:ident <C: SubstrateCLI> ( &self $(, $arg:ident : $ty:ty)* ) $(-> $result:ty)?) => {
		fn $method <C: SubstrateCLI> (&self, $($arg : $ty),*) $(-> $result)? {
			match self {
				Subcommand::BuildSpec(cmd) => cmd.$method::<C>($($arg),*),
				Subcommand::ExportBlocks(cmd) => cmd.$method::<C>($($arg),*),
				Subcommand::ImportBlocks(cmd) => cmd.$method::<C>($($arg),*),
				Subcommand::CheckBlock(cmd) => cmd.$method::<C>($($arg),*),
				Subcommand::Revert(cmd) => cmd.$method::<C>($($arg),*),
				Subcommand::PurgeChain(cmd) => cmd.$method::<C>($($arg),*),
			}
		}
	};
}

impl CliConfiguration for Subcommand {
	match_and_call! { fn base_path(&self) -> Result<Option<&PathBuf>> }

	match_and_call! { fn is_dev(&self) -> Result<bool> }

	match_and_call! { fn database_config(&self, base_path: &PathBuf, cache_size: Option<usize>) -> Result<DatabaseConfig> }

	match_and_call! { fn chain_id(&self, is_dev: bool) -> Result<String> }

	match_and_call! { fn init<C: SubstrateCLI>(&self) -> Result<()> }

	match_and_call! { fn pruning(&self, is_dev: bool, roles: Roles) -> Result<PruningMode> }

	match_and_call! { fn tracing_receiver(&self) -> Result<TracingReceiver> }

	match_and_call! { fn tracing_targets(&self) -> Result<Option<String>> }

	match_and_call! { fn state_cache_size(&self) -> Result<usize> }

	match_and_call! { fn wasm_method(&self) -> Result<WasmExecutionMethod> }

	match_and_call! { fn execution_strategies(&self, is_dev: bool) -> Result<ExecutionStrategies> }

	match_and_call! { fn database_cache_size(&self) -> Result<Option<usize>> }

	match_and_call! { fn node_key(&self, net_config_dir: &PathBuf) -> Result<NodeKeyConfig> }
}
