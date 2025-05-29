use chrono::{DateTime, Utc};

use crate::{
    OtpKey,
    one_time_pad::{ArchivedKeys, errors::*, types},
};
//=============================================================================================
pub struct KeyManager {
    archived_keys: ArchivedKeys,
    config: KeyManagerConfig,
    stats: KeyManagerStats,
}

impl KeyManager {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> KeyManagerBuilder {
        KeyManagerBuilder::new()
    }

    pub fn generate_key_for(&mut self, text: &str) -> Result<OtpKey> {
        let key = OtpKey::generate_for_text(text)?;

        if self.archived_keys.is_used(&key) {
            log::warn!("Collision detected. New generation started...");
            return self.generate_key_for(text);
        }

        self.update_stats_generation();
        Ok(key)
    }

    pub fn generate_key_for_length(&mut self, length: usize) -> Result<OtpKey> {
        let key = OtpKey::generate_for_length(length)?;

        if self.archived_keys.is_used(&key) {
            log::warn!("Collision detected. New generation started...");
            return self.generate_key_for_length(length);
        }

        self.update_stats_generation();
        Ok(key)
    }

    pub fn archive_key(&mut self, key: OtpKey) -> Result<()> {
        if self.archived_keys.count() >= self.config.max_archived_keys {
            if self.config.auto_cleanup {
                self.cleanup_old_keys()?;
            } else {
                return Err(ArcanumErr::FullArchives);
            }
        }

        self.archived_keys.archive(key)?;
        self.update_stats_archives();

        if self.config.enable_audit_log {
            log::info!("Archived key - Total count: {}", self.archived_keys.count());
        }

        Ok(())
    }

    pub fn is_key_used(&self, key: &OtpKey) -> bool {
        self.archived_keys.is_used(key)
    }

    pub fn stats(&self) -> &KeyManagerStats {
        &self.stats
    }

    pub fn archived_count(&self) -> usize {
        self.archived_keys.count()
    }

    fn cleanup_old_keys(&mut self) -> Result<()> {
        log::info!("Cleaning old keys started...");
        // to implement - useless today
        Ok(())
    }

    fn update_stats_generation(&mut self) {
        self.stats.keys_generated += 1;
        self.stats.last_generation = Some(Utc::now());
    }

    fn update_stats_archives(&mut self) {
        self.stats.keys_archived += 1;
        self.stats.memory_usage_estimate = self.archived_keys.count() * 64;
    }

    pub fn print_stats(&self) {
        let human_memory: f64 = self.stats.memory_usage_estimate as f64 / 1_048_576.0;

        println!("Generated keys: {}", self.stats.keys_generated);
        println!("Archived keys: {}", self.stats.keys_archived);
        println!("Last generation: {:?}", self.stats.last_generation);
        println!("Memory usage: {:.03} Mio", human_memory);
    }
}
//=============================================================================================
#[derive(Debug, Clone)]
pub struct KeyManagerConfig {
    pub max_archived_keys: usize,
    pub enable_audit_log: bool,
    pub auto_cleanup: bool,
}
//=============================================================================================
#[derive(Default)]
pub struct KeyManagerStats {
    pub keys_generated: u64,
    pub keys_archived: u64,
    pub last_generation: Option<DateTime<Utc>>,
    pub memory_usage_estimate: usize,
}

//=============================================================================================
#[derive(Debug, Default)]
pub struct KeyManagerBuilder {
    max_archived_keys: Option<usize>,
    enable_audit_log: bool,
    auto_cleanup: bool,
}

impl KeyManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_archived_keys(mut self, limit: usize) -> Self {
        self.max_archived_keys = Some(limit);
        self
    }

    pub fn enable_audit_log(mut self) -> Self {
        self.enable_audit_log = true;
        self
    }

    pub fn auto_cleanup(mut self) -> Self {
        self.auto_cleanup = true;
        self
    }

    pub fn build(self) -> KeyManager {
        let config = KeyManagerConfig {
            max_archived_keys: self.max_archived_keys.unwrap_or(10_000),
            enable_audit_log: self.enable_audit_log,
            auto_cleanup: self.auto_cleanup,
        };

        KeyManager {
            archived_keys: ArchivedKeys::new(),
            config,
            stats: KeyManagerStats::default(),
        }
    }
}

impl Default for KeyManagerConfig {
    fn default() -> Self {
        Self {
            max_archived_keys: 10_000,
            enable_audit_log: false,
            auto_cleanup: true,
        }
    }
}
