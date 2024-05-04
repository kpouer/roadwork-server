use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use log::{error, info};

use crate::model::sync_data::SyncData;

pub(crate) fn set_data(team: &str, opendata_service: &str, mut sync_data_list: HashMap<String, SyncData>) -> HashMap<String, SyncData> {
    info!("set_data");
    let existing_sync_data_list = get_data(team, opendata_service);
    merge(&existing_sync_data_list, &mut sync_data_list);
    save(team, opendata_service, &sync_data_list);
    sync_data_list
}

fn get_data(team: &str, opendata_service: &str) -> HashMap<String, SyncData> {
    let data_path = get_path(team, opendata_service);
    info!("getData path={}", data_path);
    match File::open(data_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(sync_data_list) => {
                    info!("Returning data for {}", opendata_service);
                    return sync_data_list;
                }
                Err(e) => info!("Error reading data: {}", e)
            }
        }
        Err(e) => info!("Error reading data: {}", e)
    }
    info!("Nothing to return for {}", opendata_service);
    HashMap::new()
}

fn save(team: &str, opendata_service: &str, roadwork_data: &HashMap<String, SyncData>) {
    let path = get_path(team, opendata_service);
    info!("save to {}", path);
    let path = Path::new(&path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Ok(_) = fs::create_dir_all(parent) {} else {
                error!("Unable to create directory {}", parent.display());
            }
        }
    }
    serde_json::to_writer(&File::create(path).unwrap(), roadwork_data).unwrap();
}

/**
 * Merge existing data with new data.
 *
 * @param existing_sync_data_list        the existing data
 * @param new_sync_data_list the new data
 */
fn merge(existing_sync_data_list: &HashMap<String, SyncData>, new_sync_data_list: &mut HashMap<String, SyncData>) {
    info!("merge");
    let server_update_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    for (id, existing_sync_data) in existing_sync_data_list {
        if let Some(new_sync_data) = new_sync_data_list.get_mut(id) {
            if new_sync_data.dirty {
                if new_sync_data.server_update_time == existing_sync_data.server_update_time {
                    info!("{} dirty=true server time is identical, update time", id);
                    new_sync_data.update_time(server_update_time);
                } else {
                    // server version is more up to date, but it is also modified by the client, use the greatest status
                    if new_sync_data.status < existing_sync_data.status {
                        info!("{} dirty=true server time is modified but server version is better ({} > {})", id, existing_sync_data.status, new_sync_data.status);
                        // server version is better
                        new_sync_data.copy(existing_sync_data);
                    } else {
                        info!("{} dirty=true server time is modified but server version is lower ({} < {})", id, existing_sync_data.status, new_sync_data.status);
                        new_sync_data.update_time(server_update_time);
                    }
                }
                new_sync_data.dirty = false;
            } else {
                if new_sync_data.server_update_time != existing_sync_data.server_update_time {
                    info!("{} dirty=false server time is different, copying server version", id);
                    new_sync_data.copy(existing_sync_data);
                }
            }
        }
    }
    new_sync_data_list.iter_mut().for_each(|(_, sync_data)| sync_data.dirty = false);
}

fn get_path(team: &str, opendata_service: &str) -> String {
    format!("data/{}/{}.json", team, opendata_service)
}