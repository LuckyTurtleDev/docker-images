use anyhow::Context;
use nonempty::NonEmpty;
use package_version::{Source, Sources, Tag};
use serde::{Deserialize, Serialize};
use std::{
	env,
	fs::{read_dir, read_to_string, write},
	path::Path
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
	source: NonEmpty<Sources>,
	#[serde(default)]
	tags: TagConfig,
	#[serde(default)]
	config: ConfigConfig
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct TagConfig {
	/// use the found verison as tag
	#[serde(default)]
	version: bool
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ConfigConfig {
	platforms: Vec<String>
}

impl Default for ConfigConfig {
	fn default() -> Self {
		Self {
			platforms: vec!["linux/amd64".to_owned()]
		}
	}
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

#[derive(Debug, Deserialize, Eq, Serialize, PartialEq)]
struct Index {
	versions: Vec<String>
}

fn get_tag(source: Sources) -> anyhow::Result<Tag> {
	let tags = source.get_tags().with_context(|| "failed to get tags")?;
	let tag = tags
		.into_iter()
		.next()
		.ok_or_else(|| anyhow::Error::msg("No suitable tag aviable at source"))?;
	Ok(tag)
}

fn process_dir(dir: &Path) -> anyhow::Result<Option<Output>> {
	let dir_name = dir.file_name().unwrap_or(dir.as_os_str()).to_string_lossy();
	println!("process {dir:?}");
	let config_path = dir.join("config.toml");
	let config =
		read_to_string(config_path).with_context(|| "Failed to open `config.toml`")?;
	let config: Config =
		basic_toml::from_str(&config).with_context(|| "failed to prase config")?;
	let mut versions = Vec::with_capacity(config.source.len());
	for source in config.source.into_iter() {
		versions.push(get_tag(source)?.version);
	}
	let index_path = dir.join("index.json");
	let old_index: Option<Index> = match read_to_string(&index_path)
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
			println!("{err:?}");
			None
		}
	};
	let new_index = Index { versions };
	if Some(&new_index) != old_index.as_ref() {
		println!("found new versions {:?}", new_index.versions);
		let path = dir
			.to_str()
			.expect("can not convert path to string")
			.to_owned();
		let new_index_str = serde_json::to_string_pretty(&new_index).unwrap();
		let mut docker_tags = "latest".to_owned();
		if config.tags.version {
			docker_tags += "\n";
			docker_tags += new_index.versions.first().unwrap(); //nonempty
		}
		let mut platforms = "".to_owned();
		for platform in config.config.platforms {
			platforms += &platform;
			platforms += ",";
		}
		platforms.pop();
		return Ok(Some(Output {
			version: new_index.versions.into_iter().next().unwrap(),
			path,
			name: dir_name.into(),
			platforms,
			docker_tags,
			index: new_index_str
		}));
	}
	Ok(None)
}

fn main() {
	let mut outputs = Vec::new();
	let dirs = read_dir("./dockerfiles").expect("failed to read dir `dockerfiles`");
	for dir in dirs {
		let dir = dir.expect("failed to access dir").path();
		println!("::group::process {dir:?}");
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
				println!("{err:?}");
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
