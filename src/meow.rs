use std::{collections::HashMap, fs, path::PathBuf};

use regex::Regex;

use crate::{dc_structs::DiscordUser, errors::ErrorThingy};

pub fn analyze(package_path: PathBuf) -> Result<(), ErrorThingy> {
    let account = fs::read_to_string(package_path.join("account/user.json"))
        .expect("Something went wrong reading the file");
    let account: DiscordUser = serde_json::from_str(&account).unwrap();

    println!("[+] analyzing data dump for {}", account.username);

    let mut money_spent: i32 = 0;
    for payment in account.money_wastes {
        let money_change = payment.amount - payment.amount_refunded;

        money_spent += money_change as i32;
    }

    println!(" money wasted on discord nitro: {} USD", money_spent / 100);

    println!("[+] analyzing messages idfk");

    get_message_counts(package_path.join("messages"))?;

    Ok(())
}

fn get_message_counts(data_path: PathBuf) -> Result<(), ErrorThingy> {
    // its very unlikely that the amount of messages will be more than the max of an unsigned 32bit integer
    let mut total_message_count: u32 = 0;
    let mut total_messages_with_attachements: u32 = 0;
    let mut direct_messages: u32 = 0;

    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // i HATE regex!!! but i still need it!!!
    let url_re = Regex::new(r"https?://(www\.)?[-a-zA-Z\d@:%._+~#=]{1,256}\.[a-zA-Z\d()]{1,6}\b([-a-zA-Z\d()!@:%_+.~#?&/=]*)").unwrap();

    data_path.read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.path().is_dir() {
            let message_channel = fs::read_to_string(path.join("channel.json"))
                .expect("Something went wrong reading the file");
            let message_channel_parsed =
                json::parse(&message_channel).expect("Failed to parse JSON");

            let messages_raw = fs::read_to_string(path.join("messages.csv"))
                .expect("Something went wrong reading the file");

            let mut messages_in_channel: u32 = 0;
            let mut messages_attachments_in_channel: u32 = 0;

            // i wish i remembered what this does
            if message_channel_parsed["type"].as_i8().unwrap() == 1 {
                return;
            }

            //println!("ID: {}", message_channel_parsed["id"].as_str().unwrap());
            csv::Reader::from_reader(messages_raw.as_bytes())
                .records()
                .for_each(|record| {
                    messages_in_channel += 1;
                    let unwrapped_record = record.unwrap();
                    if !unwrapped_record[3].is_empty() {
                        messages_attachments_in_channel += 1;
                    }

                    //println!("{}", std::str::from_utf8(unwrapped_record[2].as_bytes()).unwrap());

                    // i reeeaaalllyyy wanna get rid of this regex but i do NOT want to implement url matching :(
                    let content = url_re.replace_all(&unwrapped_record[2], "");

                    // this sucks for performance, but i can't use retain because i need to replace w/ space
                    let content: String = content
                        .chars()
                        .map(|c| if !c.is_ascii_alphanumeric() { ' ' } else { c })
                        .collect();

                    content.to_lowercase().split_whitespace().for_each(|word| {
                        if word.len() > 1 && !word.chars().all(|c| c.is_ascii_digit()) {
                            let count = word_counts.entry(word.into()).or_insert(0);
                            *count += 1;
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
    println!(
        "Messages with attachments: {}",
        total_messages_with_attachements
    );
    println!("Direct messages: {}", direct_messages);

    println!("[+] analyzing word counts ...");
    println!("Total different words: {}", word_counts.len());

    let mut count_vec: Vec<_> = word_counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Most frequent words: ");
    let mut i = 0;
    for (word, count) in count_vec.iter().take(20) {
        i += 1;
        println!("{}: {} ({})", i, word, count);
    }

    Ok(())
}
