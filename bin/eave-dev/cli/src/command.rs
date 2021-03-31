// Copyright (C) 2020-2021 Acala Foundation.
// Modifications Copyright (c) 2021 Eave Protocol
// 2021-03: Update pallet names and tokens for Eave Protocol

// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Disable the following lints
#![allow(clippy::borrowed_box)]

use crate::cli::{Cli, Subcommand};
use sc_cli::{Role, RuntimeVersion, SubstrateCli};
use sc_service::ChainType;
use service::{chain_spec, IdentifyVariant};

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Eave Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		"Eave Developers".into()
	}

	fn support_url() -> String {
		"https://github.com/eaveprotocol/eave/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2021
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id.is_empty() {
			// The binary prefix is always eave.
			// Make Steam the default chain spec.
			"steam"
		} else {
			id
		};

		Ok(match id {
			#[cfg(feature = "with-steam-runtime")]
			"dev" => Box::new(chain_spec::steam::development_testnet_config()?),
			#[cfg(feature = "with-steam-runtime")]
			"local" => Box::new(chain_spec::steam::local_testnet_config()?),
			#[cfg(feature = "with-steam-runtime")]
			"steam" => Box::new(chain_spec::steam::steam_testnet_config()?),
			#[cfg(feature = "with-steam-runtime")]
			"steam-latest" => Box::new(chain_spec::steam::latest_steam_testnet_config()?),
			#[cfg(feature = "with-eave-runtime")]
			"eave" => Box::new(chain_spec::eave::eave_config()?),
			#[cfg(feature = "with-eave-runtime")]
			"eave-latest" => Box::new(chain_spec::eave::latest_eave_config()?),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name()
						.map(|f| f.to_str().map(|s| s.starts_with(&prefix)))
						.flatten()
						.unwrap_or(false)
				};

				if starts_with("eave") {
					#[cfg(feature = "with-eave-runtime")]
					{
						Box::new(chain_spec::eave::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-eave-runtime"))]
					return Err("Eave runtime is not available. Please compile the node with `--features with-eave-runtime` to enable it.".into());
				} else {
					#[cfg(feature = "with-steam-runtime")]
					{
						Box::new(chain_spec::steam::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-steam-runtime"))]
					return Err("Steam runtime is not available. Please compile the node with `--features with-steam-runtime` to enable it.".into());
				}
			}
		})
	}

	fn native_runtime_version(spec: &Box<dyn sc_service::ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_eave() {
			#[cfg(feature = "with-eave-runtime")]
			return &service::eave_runtime::VERSION;
			#[cfg(not(feature = "with-eave-runtime"))]
			panic!("Eave runtime is not available. Please compile the node with `--features with-eave-runtime` to enable it.");
		} else {
			#[cfg(feature = "with-steam-runtime")]
			return &service::steam_runtime::VERSION;
			#[cfg(not(feature = "with-steam-runtime"))]
			panic!("Steam runtime is not available. Please compile the node with `--features with-steam-runtime` to enable it.");
		}
	}
}

fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
	use sp_core::crypto::Ss58AddressFormat;
	let ss58_version = Ss58AddressFormat::SubstrateAccount;

	/* TODO add Eave and Steam into crypto
	let ss58_version = if spec.is_karura() {
		Ss58AddressFormat::KaruraAccount
	} else if spec.is_eave() {
		Ss58AddressFormat::EaveAccount
	} else {
		Ss58AddressFormat::SubstrateAccount
	};
	*/
	sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Parses eave specific CLI arguments and run the service.
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if cli.instant_sealing && chain_spec.chain_type() != ChainType::Development {
				return Err("Instant sealing can be turned on only in `--dev` mode".into());
			}

			runner
				.run_node_until_exit(|config| async move {
					match config.role {
						Role::Light => service::build_light(config),
						_ => service::build_full(config, cli.instant_sealing, false)
							.map(|(_, _, task_manager)| task_manager),
					}
				})
				.map_err(sc_cli::Error::Service)
		}

		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| {
				let (client, _, _) = service::build_full(config, false, false)?;
				cmd.run(client)
			})
		}

		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			#[cfg(feature = "with-eave-runtime")]
			return runner.sync_run(|config| cmd.run::<service::eave_runtime::Block, service::EaveExecutor>(config));

			#[cfg(feature = "with-steam-runtime")]
			return runner
				.sync_run(|config| cmd.run::<service::steam_runtime::Block, service::SteamExecutor>(config));
		}

		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),

		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}

		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}

		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}

		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}

		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}

		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		}

		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		}
	}
}
