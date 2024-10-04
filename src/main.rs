#![cfg(unix)]

mod checker;
mod cli_opt;
mod locale_file_parser;
mod locale_key_collector;
mod rules;

use crate::checker::Checker;
use crate::cli_opt::Cli;
use crate::locale_file_parser::LocalizedTexts;
use crate::locale_key_collector::LocaleKeyCollector;
use crate::rules::key_and_eng_matches::KeyEngMatches;
use crate::rules::missing_translations::MissingTranslations;
use crate::rules::use_of_keys_do_not_exist::UseOfKeysDoNotExist;
use clap::Parser;
use serde_yaml_ng::from_reader;
use serde_yaml_ng::Value as Yaml;
use std::fs::File;

const EXIT_CODE_ON_ERROR: i32 = 1;

fn main() {
    let cli = Cli::parse();

    let locale_file = File::open(cli.locale_file()).unwrap_or_else(|e| {
        panic!(
            "Error: cannot open the specified file {} due to error {:?}",
            cli.locale_file().display(),
            e
        )
    });

    let contents: Yaml = from_reader(&locale_file).unwrap();
    let localized_texts = LocalizedTexts::new(contents);

    let rust_files_to_check = cli.rust_src_to_check();
    let mut collector = LocaleKeyCollector::new();
    collector.collect(&rust_files_to_check);

    let mut checker = Checker::new();
    checker.register_rule(MissingTranslations);
    checker.register_rule(KeyEngMatches);
    checker.register_rule(UseOfKeysDoNotExist);

    checker.check(&localized_texts, collector.locale_keys());

    checker.report_to_user();

    if checker.has_error() {
        std::process::exit(EXIT_CODE_ON_ERROR);
    }
}
