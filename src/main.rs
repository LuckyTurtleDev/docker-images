use anyhow::Context;
use package_version::{Source, Sources};
use serde::{Deserialize, Serialize};
use std::{
	env,
	fs::{read_dir, read_to_string, write, DirEntry}
};

#[derive(Debug, Deserialize)]
struct Config {
	source: Sources
}

/// Github action matrix
#[derive(Debug, Deserialize, Serialize)]
struct Matrix {
	include: Vec<Output>
}

#[derive(Debug, Deserialize, Serialize)]
struct Output {
	tag: String,
	path: String
}

fn process_dir(dir: &DirEntry) -> anyhow::Result<Option<Output>> {
	let dir = dir.path();
	let dir_name = dir
		.file_name()
		.unwrap_or_else(|| dir.as_os_str())
		.to_string_lossy();
	println!("process {dir:?}");
	let config_path = dir.join("config.toml");
	let config =
		read_to_string(config_path).with_context(|| "Failed to open `config.toml`")?;
	let config: Config =
		basic_toml::from_str(&config).with_context(|| "failed to prase config")?;
	let tags = config
		.source
		.get_tags()
		.with_context(|| "failed to get tags")?;
	let tag = tags
		.into_iter()
		.next()
		.ok_or_else(|| anyhow::Error::msg("No suitable tag aviable at source"))?;
	let old_tag_path = dir.join("last_tag.txt");
	let old_tag = match read_to_string(&old_tag_path)
		.with_context(|| format!("failed to open {old_tag_path:?}"))
	{
		Ok(value) => Some(value),
		Err(err) => {
			let title = "failed to load last tag. Use `None`";
			let msg = format!("{err:?}"); // print string as single line
			println!("::warning title={dir_name}: {title}::{msg:?}");
			let err = err.context(title);
			eprintln!("{err:?}");
			None
		}
	};
	if Some(&tag.version) != old_tag.as_ref() {
		println!("found new tag {:?}", tag.version);
		let path = dir
			.to_str()
			.expect("can not convert path to string")
			.to_owned();
		return Ok(Some(Output {
			tag: tag.version,
			path
		}));
	}
	Ok(None)
}

fn main() {
	let mut outputs = Vec::new();
	let dirs = read_dir("./dockerfiles").expect("failed to read dir `dockerfiles`");
	for dir in dirs {
		let dir = dir.expect("failed to access dir");
		match process_dir(&dir) {
			Ok(output) => {
				if let Some(output) = output {
					outputs.push(output);
				}
			},
			Err(err) => {
				let title = format!("failed to process {dir:?}");
				println!("::error title={title}::{err:?}");
				let err = err.context(title);
				eprintln!("{err:?}");
			}
		}
	}
	let matrix = Matrix { include: outputs };
	println!(
		"matrix output: {}",
		serde_json::to_string_pretty(&matrix).unwrap()
	);
	let json = serde_json::to_string(&matrix).unwrap();
	let json = format!("matrix={json}");
	write("output.txt", &json).expect("failed to write to output.txt`");
	env::set_var("GITHUB_OUTPUT", json);
}
