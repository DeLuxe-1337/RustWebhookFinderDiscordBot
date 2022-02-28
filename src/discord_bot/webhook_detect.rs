use rust_strings::{dump_strings, strings, BytesConfig, Encoding, FileConfig};
use std::path::PathBuf;

pub fn get_strings(encode: Encoding, file: String) -> Vec<String> {
    let mut final_strs: Vec<String> = Vec::new();
    let mut config = FileConfig::new(file.as_str())
        .with_min_length(5)
        .with_encoding(encode);

    let strs = strings(&config).unwrap();

    for s in strs {
        final_strs.push(s.0.to_string());
    }

    final_strs
}

pub fn get_webhook_strings(file: String, encode: Encoding) -> Vec<String> {
    let extracted = get_strings(encode, file);
    let mut result: Vec<String> = Vec::new();

    for i in extracted {
        if i.contains("https://") {
            if i.contains("https://discord.com") {
                result.push(format!("- {}", i));
            }
            else {
                result.push(format!("+ {}", i));
            }
        }
    }

    result
}

pub fn get_discord_message(file: String) -> String {
    let mut result = String::new();
    result.push_str("```diff\nFormat: ASCII\n");

    get_webhook_strings(file.clone(), Encoding::ASCII)
        .iter()
        .for_each(|s| {
            result.push_str(s);
            result.push_str("\n");
        });

    result.push_str("Format: UTF16LE\n");

    get_webhook_strings(file.clone(), Encoding::UTF16LE)
        .iter()
        .for_each(|s| {
            result.push_str(s);
            result.push_str("\n");
        });

    result.push_str("Format: UTF16BE\n");

    get_webhook_strings(file.clone(), Encoding::UTF16BE)
        .iter()
        .for_each(|s| {
            result.push_str(s);
            result.push_str("\n");
        });

    result.push_str("```");

    std::fs::remove_file(file).expect("failed to remove file");

    result
}
