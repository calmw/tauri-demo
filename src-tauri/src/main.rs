// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::HashMap;
// use serde::de::IntoDeserializer;
use libsecp256k1;
use hex::*;
use sha3::*;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    code: i32,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SysConfig {
    magic_crypt_key: String,
    magic_crypt_pwd: String,
    magic_crypt_private_key: HashMap<String, String>,
}

// 解锁
#[tauri::command]
fn unlock(password: &str) -> bool {
    let config = read_config();
    let mc = new_magic_crypt!(config.magic_crypt_key+"cisco", 256);

    let base64 = mc.encrypt_str_to_base64(password);
    if base64 != config.magic_crypt_pwd {
        false
    } else {
        true
    }
}

// 检测是否设置密码
#[tauri::command]
fn check_pwd() -> bool {
    let config = read_config();
    if config.magic_crypt_pwd == "" {
        false
    } else {
        true
    }
}

// 设置解锁密码
#[tauri::command]
fn set_pwd(pwd: &str, re_pwd: &str) -> bool {
    if pwd == "" || pwd != re_pwd {
        false
    } else {
        let config = read_config();
        let mc = new_magic_crypt!(config.magic_crypt_key.clone()+"cisco", 256);
        let base64_pwd = mc.encrypt_str_to_base64(pwd);
        let data_write_to_config = SysConfig {
            magic_crypt_key: config.magic_crypt_key.clone(),
            magic_crypt_pwd: base64_pwd,
            magic_crypt_private_key: config.magic_crypt_private_key.clone(),
        };
        fs::write(
            "./config.json",
            serde_json::to_string_pretty(&data_write_to_config).expect("error parsing to JSON"),
        ).expect("error writing to file");
        true
    }
}

// 确认无法解锁弹窗
#[tauri::command]
fn reset_pwd() -> bool {
    let config = read_config();
    let data_write_to_config = SysConfig {
        magic_crypt_key: config.magic_crypt_key.clone(),
        magic_crypt_pwd: "".to_string(),
        magic_crypt_private_key: HashMap::new(),
    };
    fs::write(
        "./config.json",
        serde_json::to_string_pretty(&data_write_to_config).expect("error parsing to JSON"),
    ).expect("error writing to file");
    true
}

fn rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

fn read_config() -> SysConfig {
    let json_string = fs::read_to_string("./config.json").expect("TODO: panic message");
    let deserialized_person: SysConfig = serde_json::from_str(&json_string).expect("Unable to parse");

    deserialized_person
}

fn check_config() {
    let path = env::current_dir().unwrap();
    let dir = path.as_path().read_dir().unwrap();
    let mut exits: bool = false;
    for x in dir {
        if let Ok(path) = x {
            // 是否存在某个文件
            if path.file_name().eq("config.json") {
                exits = true;
            }
        }
    }
    if !exits {
        let magic_crypt_key = rand_string(30);
        // 初始化 config.json
        let data_write_to_config = SysConfig {
            magic_crypt_key,
            magic_crypt_pwd: "".to_string(),
            magic_crypt_private_key: HashMap::new(),
        };

        fs::write(
            "./config.json",
            serde_json::to_string_pretty(&data_write_to_config).expect("error parsing to JSON"),
        ).expect("error writing to file");
    }
}

#[tauri::command]
fn add_private_key_to_config(key: &str) -> i32 {
    let res: i32 = 0;
    let private_key_str: &str;
    if key.starts_with("0x") {
        private_key_str = &key[2..];
    } else {
        private_key_str = key;
    }
    let mut config = read_config();

    let buffer_private = <[u8; 32]>::from_hex(private_key_str).unwrap();
    let keypair = libsecp256k1::SecretKey::parse(&buffer_private).unwrap();
    let private_key = format!("0x{}", libsecp256k1::SecretKey::serialize(&keypair).encode_hex::<String>());
    println!("private_key:{:?}", private_key);
    let public_key = libsecp256k1::PublicKey::from_secret_key(&keypair);
    println!("public_key:{:?}", libsecp256k1::PublicKey::serialize(&public_key).encode_hex::<String>());

    let p = &libsecp256k1::PublicKey::serialize(&public_key)[1..];
    let mut hasher = Keccak256::new();
    hasher.update(p);
    let result = hasher.finalize();
    let address = format!("0x{}", (&result[12..]).encode_hex::<String>());
    let has_key = config.magic_crypt_private_key.contains_key(&address); // 返回布尔值
    if has_key {
        return res + 1; // 已经存在
    }
    let mc = new_magic_crypt!(config.magic_crypt_key.clone()+"private_key", 256);
    let base64_private_key = mc.encrypt_str_to_base64(private_key);
    config.magic_crypt_private_key.insert(address, base64_private_key);
    let data_write_to_config = SysConfig {
        magic_crypt_key: config.magic_crypt_key,
        magic_crypt_pwd: config.magic_crypt_pwd,
        magic_crypt_private_key: config.magic_crypt_private_key,
    };

    fs::write(
        "./config.json",
        serde_json::to_string_pretty(&data_write_to_config).expect("error parsing to JSON"),
    ).expect("error writing to file");

    res + 1
}

#[tauri::command]
fn private_key_exits() -> bool {
    let config = read_config();
    if config.magic_crypt_private_key.len() > 0 {
        return true;
    }
    return false;
}


fn main() {
    check_config();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_pwd,set_pwd,reset_pwd,unlock,add_private_key_to_config,private_key_exits])
        .run(tauri::generate_context!())
        .expect("server error");
}
