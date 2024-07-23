mod checker;
mod parse;
mod rules;

use crate::checker::Checker;
use crate::parse::LocalizedTexts;
use crate::rules::key_and_eng_matches::KeyEngMatches;
use crate::rules::missing_translations::MissingTranslations;
use serde_yaml_ng::from_reader;
use serde_yaml_ng::Value as Yaml;
use std::env::args;
use std::fs::File;

fn main() {
    let file_name = args()
        .nth(1)
        .expect("Error: a yaml file should be specified");
    let file = File::open(&file_name).unwrap_or_else(|e| {
        panic!(
            "Error: cannot open the specified file {file_name} due to error {:?}",
            e
        )
    });

    let contents: Yaml = from_reader(&file).unwrap();

    let localized_texts = LocalizedTexts::new(contents);

    let mut checker = Checker::new();
    checker.register_rule(MissingTranslations);
    checker.register_rule(KeyEngMatches);

    checker.check(&localized_texts);

    checker.report_to_user();
}
