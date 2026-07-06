use std::path::Path;
use std::sync::Mutex;

use local_store::LocalStore;

pub struct AppState {
    pub store: Mutex<LocalStore>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::open("token-tamagotchi.sqlite3")
    }
}

impl AppState {
    pub fn open(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let store = LocalStore::open(path).unwrap_or_else(|error| {
            panic!("failed to open local store at {}: {error}", path.display())
        });

        Self {
            store: Mutex::new(store),
        }
    }
}
