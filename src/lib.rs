#![allow(unused_assignments)]
pub mod cli;
pub mod fetch;
use crate::fetch::{fetch_lyric, fetch_song_info, Result};
use anyhow::anyhow;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use json::JsonValue;
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

pub async fn run(search_name: String, save_path: String) -> Result<()> {
    /* 获取关键字检索到的歌曲信息 */
    let resp = fetch_song_info(search_name.clone()).await?;
    /* 获取歌曲信息列表 */
    let songs = resp["result"]["songs"].members();
    /* 创建保存信息的列表 */
    let mut songs_name = Vec::new();
    let mut artists = Vec::new();
    let mut ids = Vec::new();
    let mut selections = Vec::new();

    for song in songs {
        if let Some(id) = song["id"].as_i64() {
            songs_name.push(song["name"].to_string());
            artists.push(song["artists"][0]["name"].to_string());
            selections.push(format!(
                "id: {id} | 歌曲名: {} | 艺人: {}",
                song["name"].to_string(),
                song["artists"][0]["name"].to_string()
            ));
            ids.push(id)
        } else {
            eprintln!("获取歌曲id失败");
            return Ok(());
        }
    }

    if selections.len() == 0 {
        eprintln!("获取关键字 `{search_name}` 有关歌曲的信息失败");
        return Ok(());
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择其中一个id进行下载:")
        .items(&selections)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let mut lrc_resp_vec = JsonValue::new_array();
    let mut song_name = String::new();
    let mut artist_name = String::new();
    match selection {
        Some(selection) => {
            lrc_resp_vec = fetch_lyric(ids[selection]).await?;
            /* 获取歌曲的名字*/
            song_name = songs_name[selection].replace(" ", "_");
            /* 获取歌曲的艺术家的名字*/
            artist_name = artists[selection].replace(" ", "_");
        }
        None => {
            eprintln!("获取序号失败");
            return Ok(());
        }
    }

    /* 获取歌曲的歌词 */
    let lrc = lrc_resp_vec["lrc"]["lyric"].to_string();
    /* 歌词保存的路径*/
    let lrc_path = format!("{save_path}/{song_name}-{artist_name}.lrc");

    /* 创建保存歌词的文件夹*/
    if !Path::new(&save_path).exists() {
        create_dir(&save_path).map_err(|err| anyhow!("[ERROR] 原因: {err}"))?;
    }
    /* 创建歌词文件 */
    let mut file = File::create(&lrc_path)?;
    match file.write_all(lrc.as_bytes()) {
        Ok(_) => println!("下载歌词成功， 保存路径: `{lrc_path}`"),
        Err(err) => eprintln!("[ERROR] 下载错误， 原因: `{err}`"),
    }
    Ok(())
}
