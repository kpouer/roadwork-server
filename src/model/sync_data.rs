use serde::{Deserialize, Serialize};

use crate::model::status::Status;

#[derive(Deserialize, Serialize)]
pub(crate) struct SyncData {
    #[serde(rename = "localUpdateTime")]
    pub(crate) local_update_time: u64,
    /**
     * Timestamp of the last server change
     */
    #[serde(rename = "serverUpdateTime")]
    pub(crate) server_update_time: u64,
    pub(crate) status: Status,
    pub(crate) dirty: bool,
}

impl Default for SyncData {
    fn default() -> Self {
        SyncData {
            local_update_time: 0,
            server_update_time: 0,
            status: Status::New,
            dirty: false,
        }
    }
}

impl SyncData {
    pub(crate) fn copy(&mut self, other: &SyncData) {
        self.local_update_time = other.local_update_time;
        self.server_update_time = other.server_update_time;
        self.status = other.status;
    }

    pub(crate) fn update_time(&mut self, server_update_time: u64) {
        self.local_update_time = server_update_time;
        self.server_update_time = server_update_time;
    }
}
