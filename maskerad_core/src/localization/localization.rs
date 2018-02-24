// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
 LOCALIZATION SYSTEM.

 The localization system allows you to localize your game, without "too-much" hassle.

 A localization file is in the .CSV form, like this:
 ___________________________________________________
 |      | [lang1] |  [lang2] |  [lang3] |  [langN] |
 |______|_________|__________|__________|__________|
 | Key1 |_________|__________|__________|__________|
 | Key2 |_________|__________|__________|__________|
 | Key3 |_________|__________|__________|__________|
 | Key4 |_________|__________|__________|__________|
 | KeyN |_________|__________|__________|__________|

 A tool "CSV_to_JSON" will read the CSV file, and will create a JSON file
 for each language.

 Note that each [lang] id must be a locale id, like 'en' for english, or 'fr' for french.
 See this document from the Godot documentation: https://godot.readthedocs.io/en/latest/tutorials/misc/locales.html#doc-locales

 This tool will place all those file in {Working directory}/localization/{locale id}/localization.json.
 So, if your CSV file has translations for english, french and spanish, this tool will create the following hierarchy:

 WorkingDirectory
 |
 |-------localization
 |       |
 |       |
 |       |-------en
 |       |       |----localization.json
 |       |
 |       |-------fr
 |       |       |----localization.json
 |       |
 |       |-------es
 |       |       |----localization.json

 When the engine runs, it will load a configuration object (manually set, or from a toml file),
 which contain the locale used.

 To create a Localization structure, we pass it a locale as an str. This object, according to the locale,
 will check if the localization.json associated to this locale exists. If it exists, the json file is
 deserialized to a Rust structure holding an Hashmap<Key, String>.

 The programmer can then use the Localization to get the correct string, without having to worry
 about the language. The code will not change.
*/

use std::collections::HashMap;
use serde_json;
use std::io::{Read, Write};
use localization::localization_error::{LocalizationError, LocalizationResult};
use std::path::PathBuf;

pub struct Localization {
    manifest: Manifest,
}

impl Localization {
    pub fn new<R: Read>(reader: R) -> LocalizationResult<Self>
    {
        let manifest = Manifest::from_reader(reader)?;

        Ok(Localization {
            manifest
        })
    }

    pub fn get<S>(&self, id: S) -> Option<&str> where
        S: AsRef<str>
    {
        self.manifest.get(id.as_ref())
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    translation: HashMap<String, String>,
}

impl Manifest {
    pub fn from_reader<R: Read>(reader: R) -> LocalizationResult<Self> {
        serde_json::from_reader(reader).map_err(|json_error| {
            LocalizationError::from(json_error)
        })
    }

    pub fn get<S>(&self, id: S) -> Option<&str> where
        S: AsRef<str>
    {
        match self.translation.get(id.as_ref()) {
            Some(string) => {
                Some(string.as_str())
            },
            None => {
                None
            },
        }
    }
}