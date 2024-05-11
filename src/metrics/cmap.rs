// Cmapmetrics data structure
// 基本功能： inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    // pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow!(e.to_string()))?
    //         .clone())
    // }
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in self.data.iter() {
            writeln!(f, "{}:, {}", e.key(), e.value())?;
        }

        Ok(())
    }
}
