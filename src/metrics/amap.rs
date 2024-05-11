use anyhow::{anyhow, Result};
use std::sync::atomic::Ordering;
use std::{
    collections::HashMap,
    fmt,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMertrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMertrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMertrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();

        let counter = self.data.get(key).ok_or_else(|| anyhow!("key not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMertrics {
    fn clone(&self) -> Self {
        AmapMertrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl fmt::Display for AmapMertrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.data.iter() {
            writeln!(f, "{}:{}", k, v.load(Ordering::Relaxed))?;
        }

        Ok(())
    }
}
