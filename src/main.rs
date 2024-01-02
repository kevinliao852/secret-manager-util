use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager;
use clap::{App, Arg};
use serde_json;
use std::collections::HashMap;

#[::tokio::main]
async fn main() -> Result<(), aws_sdk_secretsmanager::Error> {
    let matches = App::new("aws secrets manager diff")
        .version("1.0")
        .author("Kevin Liao")
        .about("A simple Rust CLI")
        .arg(
            Arg::with_name("secret_name")
                .short("f")
                .long("secret_name")
                .value_name("SECRET_NAME")
                .help("Sets the secret name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("secret_name_v2")
                .short("s")
                .long("secret_name_v2")
                .value_name("SECRET_NAME_V2")
                .help("Sets the secret name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("region")
                .short("region")
                .long("region")
                .value_name("REGION")
                .help("Sets the region")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    // Extract values from the command-line arguments
    let secret_name = matches.value_of("secret_name").unwrap();
    let secret_name_v2 = matches.value_of("secret_name_v2").unwrap();
    let _region_str = matches.value_of("region").unwrap_or("ap-northeast-1");

    let region = Region::new("ap-northeast-1");

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm.get_secret_value().secret_id(secret_name).send().await?;
    let response_2 = asm
        .get_secret_value()
        .secret_id(secret_name_v2)
        .send()
        .await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    let secret_string = response.secret_string();
    let secret_string_2 = response_2.secret_string();

    if secret_string.is_none() {
        panic!("Secret string is empty");
    }

    if secret_string_2.is_none() {
        panic!("Secret string is empty");
    }

    let secret_string_result = secret_string.unwrap();
    let secret_string_result_2 = secret_string_2.unwrap();

    // store the secrets in a hashmap
    let secrets_map = store_secrets_in_hashmap(secret_string_result.to_string());
    let secrets_map_2 = store_secrets_in_hashmap(secret_string_result_2.to_string());

    // diff the two hashmaps
    let diff_hashmap = find_diff_keys(&secrets_map, &secrets_map_2);

    // print the diff hash map
    for (key, _) in diff_hashmap {
        println!("{}", key);
    }

    Ok(())
}

fn store_secrets_in_hashmap(secret_string: String) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // parse the json string into a hashmap
    let json: serde_json::Value = serde_json::from_str(&secret_string).unwrap();

    //turn it to hashmap
    let json_map = json.as_object().unwrap();

    // iterate over the hashmap and store the values in a new hash map
    for (key, value) in json_map {
        map.insert(key.to_string(), value.to_string());
    }

    map
}

fn find_diff_keys(
    old_hashmap: &HashMap<String, String>,
    new_hashmap: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut diff_keys: HashMap<String, String> = HashMap::new();
    // Find keys in old_hashmap but not in new_hashmap
    for (key, value) in old_hashmap {
        if !new_hashmap.contains_key(key) {
            diff_keys.insert(key.clone(), value.clone());
        }
    }

    // Find keys in new_hashmap but not in old_hashmap
    for (key, value) in new_hashmap {
        if !old_hashmap.contains_key(key) {
            diff_keys.insert(key.clone(), value.clone());
        }
    }

    diff_keys
}
