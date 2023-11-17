use anyhow::Context;
use package_version::{Source, Sources};
use serde::{Deserialize, Serialize};
use std::{
	env,
	fs::{read_dir, read_to_string, write, DirEntry}
};

#[derive(Debug, Deserialize)]
struct Config {
	source: Sources,
	#[serde(default)]
	tags: TagConfig
}

#[derive(Debug, Default, Deserialize)]
struct TagConfig {
	/// use the found verison as tag
	#[serde(default)]
	version: bool
}

/// Github action matrix
#[derive(Debug, Serialize)]
struct Matrix {
	include: Vec<Output>
}

#[derive(Debug, Serialize)]
struct Output {
	version: String,
	path: String,
	name: String,
	platforms: String,
	docker_tags: String,
	index: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Index {
	version: String
}

fn process_dir(dir: &DirEntry) -> anyhow::Result<Option<Output>> {
	let dir = dir.path();
	let dir_name = dir
		.file_name()
		.unwrap_or_else(|| dir.as_os_str())
		.to_string_lossy();
	println!("process {dir:?}", dir.as_path());
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
	let index_path = dir.join("index.json");
	let index: Option<Index> = match read_to_string(&index_path)
		.with_context(|| format!("failed to open {index_path:?}"))
		.and_then(|index| {
			serde_json::from_str(&index)
				.with_context(|| format!("failed to parse {index_path:?}"))
		}) {
		Ok(value) => Some(value),
		Err(err) => {
			let title = "failed to load index. Use `None`";
			let msg = format!("{err:?}"); // print string as single line
			println!("::warning title={dir_name}: {title}::{msg:?}");
			let err = err.context(title);
			eprintln!("{err:?}");
			None
		}
	};
	if Some(&tag.version) != index.as_ref().map(|f| &f.version) {
		println!("found new tag {:?}", tag.version);
		let path = dir
			.to_str()
			.expect("can not convert path to string")
			.to_owned();
		let index = Index {
			version: tag.version.clone()
		};
		let index = serde_json::to_string_pretty(&index).unwrap();
		let mut docker_tags = "latest".to_owned();
		if config.tags.version {
			docker_tags += " ";
			docker_tags += &tag.version;
		}
		return Ok(Some(Output {
			version: tag.version,
			path,
			name: dir_name.into(),
			platforms: "linux/amd64".to_owned(),
			docker_tags,
			index
		}));
	}
	Ok(None)
}

fn main() {
	let mut outputs = Vec::new();
	let dirs = read_dir("./dockerfiles").expect("failed to read dir `dockerfiles`");
	for dir in dirs {
		let dir = dir.expect("failed to access dir");
		println!("::group::{dir:?}");
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
		println!("::endgroup::")
	}
	let matrix = Matrix { include: outputs };
	println!(
		"matrix output: {}",
		serde_json::to_string_pretty(&matrix).unwrap()
	);
	let json = if matrix.include.is_empty() {
		r#"{"include": [{"skip": "true", "name": "no updates available"}]}"#.to_owned()
	} else {
		serde_json::to_string(&matrix).unwrap()
	};
	let json = format!("matrix={json}");
	let github_output = env::var("GITHUB_OUTPUT")
		.expect("GITHUB_OUTPUT environment variabel is not present");
	write(github_output, json).expect("failed to write to output.txt`");
}
