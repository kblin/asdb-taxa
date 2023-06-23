// Copyright 2023 Danmarks Tekniske Universitet
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use asdb_taxa::TaxonCache;

#[derive(Debug, Parser)]
#[command(name = "asdb-taxa", about = "Create a taxon cache for ASDB")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "init", about = "Initialise a new cache")]
    Init(InitOpts),

    #[command(name = "add", about = "Add more entries to an existing cache")]
    Add(AddOpts),

    #[command(name = "list", about = "List current cache entries")]
    List(ListOpts),
}

#[derive(Debug, Args)]
struct InitOpts {
    #[arg(short, long, help = "Cache file to use")]
    cache: String,

    #[arg(short, long, help = "ASDB json data directory to determine needed taxids")]
    datadir: String,

    #[arg(short, long, help = "TaxonDB merged ID dump file to load from")]
    mergeddump: String,

    #[arg(short, long, help = "TaxonDB ranked lineage dump file to load from")]
    taxdump: String,
}

#[derive(Debug, Args)]
struct AddOpts {
    #[arg(short, long, help = "Cache file to use")]
    cache: String,

    #[arg(short, long, help = "ASDB json data directory to determine needed taxids")]
    datadir: String,

    #[arg(short, long, help = "TaxonDB merged ID dump file to load from")]
    mergeddump: String,

    #[arg(short, long, help = "TaxonDB ranked lineage dump file to load from")]
    taxdump: String,
}

#[derive(Debug, Args)]
struct ListOpts {
    #[arg(short, long, help = "Cache file to use")]
    cache: String,
}

pub fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Init(cfg) => init(cfg),
        Commands::Add(cfg) => add(cfg),
        Commands::List(cfg) => list(cfg),
    }
}

fn init(args: InitOpts) {
    let mut taxon_cache = TaxonCache::new();

    taxon_cache
        .initialise_from_paths(
            PathBuf::from(args.taxdump),
            PathBuf::from(args.mergeddump),
            PathBuf::from(args.datadir),
        )
        .expect("Failed to initialise cache");

    taxon_cache
        .save_path(&PathBuf::from(args.cache))
        .expect("Failed to save cache");
}

fn add(args: AddOpts) {
    let mut taxon_cache = TaxonCache::new();
    let cache_file = PathBuf::from(args.cache);
    taxon_cache
        .load_path(&cache_file)
        .expect("Failed to load cache file");

    taxon_cache
        .initialise_from_paths(
            PathBuf::from(args.taxdump),
            PathBuf::from(args.mergeddump),
            PathBuf::from(args.datadir),
        )
        .expect("Failed to initialise cache");

    taxon_cache
        .save_path(&cache_file)
        .expect("Failed to save cache");
}

fn list(args: ListOpts) {
    let mut taxon_cache = TaxonCache::new();

    taxon_cache
        .load_path(&PathBuf::from(args.cache))
        .expect("Failed to load cache file");
    for (tax_id, entry) in &taxon_cache.mappings {
        println!("{}: {}", tax_id, entry.name)
    }
    println!("\n{} entries total", taxon_cache.mappings.len())
}
