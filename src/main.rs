use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let package_path:&Path = Path::new("package");

    let account = fs::read_to_string(package_path.join("account/user.json")).expect("Something went wrong reading the file");
    let account_parsed = json::parse(&account).expect("Failed to parse JSON");

    println!("Analyzing data dump for {}#{}", account_parsed["username"].as_str().unwrap(), account_parsed["discriminator"].as_i16().unwrap());

    get_message_counts(package_path.join("messages"));
}

fn get_message_counts(data_path:PathBuf) {
    // its very unlikely that the amount of messages will be more than the max of an unsigned 32bit integer
    let mut total_message_count:u32 = 0;
    let mut total_messages_with_attachements:u32 = 0;
    let mut direct_messages:u32 = 0;

    let mut word_counts:HashMap<String, u32> = HashMap::new();

    let re = Regex::new(r"[^A-Za-z\d]").unwrap();
    let url_re = Regex::new(r"https?://(www\.)?[-a-zA-Z\d@:%._+~#=]{1,256}\.[a-zA-Z\d()]{1,6}\b([-a-zA-Z\d()!@:%_+.~#?&/=]*)").unwrap();

    data_path.read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.path().is_dir() {
            let message_channel = fs::read_to_string(path.join("channel.json")).expect("Something went wrong reading the file");
            let message_channel_parsed = json::parse(&message_channel).expect("Failed to parse JSON");

            let messages_raw = fs::read_to_string(path.join("messages.csv")).expect("Something went wrong reading the file");

            let mut messages_in_channel:u32 = 0;
            let mut messages_attachments_in_channel:u32 = 0;

            //println!("ID: {}", message_channel_parsed["id"].as_str().unwrap());
            csv::Reader::from_reader(messages_raw.as_bytes()).records().for_each(|record| {
                messages_in_channel += 1;
                let unwrapped_record = record.unwrap();
                if unwrapped_record[3].as_bytes().len() > 0 {
                    messages_attachments_in_channel += 1;
                }

                println!("{}", std::str::from_utf8(unwrapped_record[2].as_bytes()).unwrap());

                let mut content = url_re.replace_all(std::str::from_utf8(unwrapped_record[2].as_bytes()).unwrap(), "").to_string();
                content = re.replace_all(&*content, " ").to_string();

                while content.contains("  ") {
                    content = content.replace("  ", " ");
                }

                if content.starts_with(" ") {
                    content.remove(0).to_string();
                }

                content = content.to_lowercase();

                content.split(" ").for_each(|word| {
                    if word.len() > 1 {
                        if !word.chars().all(|c| c.is_numeric()) {
                            let count = word_counts.entry(word.to_string()).or_insert(0);
                            *count += 1;
                        }
                    }
                });
            });

            total_message_count += messages_in_channel;
            total_messages_with_attachements += messages_attachments_in_channel;

            if message_channel_parsed["type"].as_i8().unwrap() == 1 {
                direct_messages += messages_in_channel;
            }
        }
    });

    println!("Total messages: {}", total_message_count);
    println!("Messages with attachments: {}", total_messages_with_attachements);
    println!("Direct messages: {}", direct_messages);

    println!("Analyzing word counts ...");
    println!("Total different words: {}", word_counts.len());

    let mut count_vec: Vec<_> = word_counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Most frequent words: ");
    let mut i = 0;
    for (word, count) in count_vec.iter().take(200) {
        i += 1;
        println!("{}: {} ({})", i, word, count);
    }
}
