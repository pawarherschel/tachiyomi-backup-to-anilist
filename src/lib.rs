use tachiyomi::backup;

pub mod tachiyomi {
    pub mod backup {
        include!(concat!(env!("OUT_DIR"), "/tachiyomi.backup.rs"));
    }
}

pub fn asdf() -> backup::Backup {
    Default::default()
}
