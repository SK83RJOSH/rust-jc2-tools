use bevy::{
    asset::{
        io::{AssetSource, AssetSourceId},
        AssetLoadFailedEvent,
    },
    prelude::*,
    utils::HashMap,
};
use jc2_hashing::HashString;
use parking_lot::RwLock;
use std::{path::PathBuf, sync::Arc};

mod archive;
use archive::{Archive, ArchiveLoader};

mod asset_reader;
use asset_reader::FileSystemAssetReader;

#[derive(Default, Debug)]
pub struct FileSystemPlugin;

#[derive(Event, Debug)]
pub enum FileSystemEvent {
    DirectoryMounted { path: PathBuf },
    DirectoryUnmounted { path: PathBuf },
    ArchivePending { path: PathBuf },
    ArchiveMounted { path: PathBuf },
    ArchiveUnmounted { path: PathBuf },
    ArchiveError { path: PathBuf },
}

#[derive(Default, Debug)]
pub struct FileSystemMountsData {
    pub(crate) directories: RwLock<Vec<PathBuf>>,
    pub(crate) archives: RwLock<Vec<Archive>>,
}

#[derive(Resource, Default, Debug)]
pub struct FileSystemMounts {
    pub(crate) mounts: Arc<FileSystemMountsData>,
    pub(crate) pending_archives: HashMap<HashString, Handle<Archive>>,
    pub(crate) pending_events: Vec<FileSystemEvent>,
}

impl FileSystemMounts {
    pub fn mount_directory(&mut self, path: impl Into<PathBuf>) -> &Self {
        let path = path.into();
        {
            let mut directories = self.mounts.directories.write();
            let directory_count = directories.len();
            directories.retain(|directory| *directory != path);
            directories.push(path.clone());
            if directories.len() > directory_count {
                self.pending_events
                    .push(FileSystemEvent::DirectoryMounted { path });
            }
        }
        self
    }

    pub fn unmount_directory(&mut self, path: impl Into<PathBuf>) -> &Self {
        let path: PathBuf = path.into();
        {
            let mut directories = self.mounts.directories.write();
            let directory_count = directories.len();
            directories.retain(|directory| directory != &path);
            if directories.len() < directory_count {
                self.pending_events
                    .push(FileSystemEvent::DirectoryUnmounted { path });
            }
        }
        self
    }

    pub fn mount_archive(&mut self, asset_server: &AssetServer, path: impl Into<PathBuf>) -> &Self {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        for archive in self.mounts.archives.read().iter() {
            if archive.hash == hash {
                return self;
            }
        }
        self.pending_archives
            .insert(hash, asset_server.load(path.clone()));
        self.pending_events
            .push(FileSystemEvent::ArchivePending { path });
        self
    }

    pub fn unmount_archive(&mut self, path: impl Into<PathBuf>) -> &Self {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        {
            let mut archives = self.mounts.archives.write();
            let archive_count = archives.len();
            archives.retain(|archive| archive.hash != hash);
            if archives.len() < archive_count {
                self.pending_events
                    .push(FileSystemEvent::ArchiveUnmounted { path });
            }
        }
        self.pending_archives.remove(&hash);
        self
    }

    pub fn is_mounting_archives(&self) -> bool {
        !self.pending_archives.is_empty()
    }

    pub fn is_mounting_archive(&self, path: impl Into<PathBuf>) -> bool {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        self.pending_archives.contains_key(&hash)
    }

    pub fn has_mounted_archive(&self, path: impl Into<PathBuf>) -> bool {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        self.mounts
            .archives
            .read()
            .iter()
            .any(|archive| archive.hash == hash)
    }
}

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        let mounts = Arc::new(FileSystemMountsData::default());
        app.insert_resource(FileSystemMounts {
            mounts: mounts.clone(),
            pending_archives: HashMap::new(),
            pending_events: Vec::new(),
        })
        .register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || {
                Box::new(FileSystemAssetReader::new(
                    mounts.clone(),
                    AssetSource::get_default_reader("assets".into()),
                ))
            }),
        )
        .add_event::<FileSystemEvent>()
        .add_systems(First, process_archive_events);
    }

    fn finish(&self, app: &mut App) {
        app.init_asset::<Archive>()
            .register_asset_loader(ArchiveLoader);
    }
}

fn process_archive_events(
    mut archives: ResMut<Assets<Archive>>,
    mut load_events: EventReader<AssetEvent<Archive>>,
    mut failed_events: EventReader<AssetLoadFailedEvent<Archive>>,
    mut event_writer: EventWriter<FileSystemEvent>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    // Process pending events
    for event in mounts.pending_events.drain(..) {
        event_writer.send(event);
    }

    // Process loaded archives
    for archive in load_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::LoadedWithDependencies { id } => Some(*id),
            _ => None,
        })
        .filter_map(|h| archives.remove(h))
    {
        event_writer.send(FileSystemEvent::ArchivePending {
            path: archive.source_path.clone(),
        });
        let hash = archive.hash;
        mounts.mounts.archives.write().push(archive);
        mounts.pending_archives.remove(&hash);
    }

    // Process failed archives
    for path in failed_events.read().map(|event| &event.path) {
        event_writer.send(FileSystemEvent::ArchiveError {
            path: path.path().into(),
        });
        mounts
            .pending_archives
            .remove(&HashString::from_str(&path.to_string()));
    }
}
