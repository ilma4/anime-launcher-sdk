pub mod wine_lang;
pub mod wine_sync;
pub mod virtual_desktop;
pub mod shared_libraries;

pub mod prelude {
    pub use super::wine_lang::WineLang;
    pub use super::wine_sync::WineSync;
    pub use super::virtual_desktop::VirtualDesktop;
    pub use super::shared_libraries::SharedLibraries;
}

#[macro_export]
macro_rules! config_impl_wine_schema {
    ($launcher_dir:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Wine {
            pub prefix: PathBuf,
            pub builds: PathBuf,
            pub selected: Option<String>,
            pub sync: WineSync,
            pub language: WineLang,
            pub borderless: bool,
            pub virtual_desktop: VirtualDesktop,
            pub shared_libraries: SharedLibraries
        }

        impl Default for Wine {
            #[inline]
            fn default() -> Self {
                let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

                Self {
                    prefix: launcher_dir.join("prefix"),
                    builds: launcher_dir.join("runners"),
                    selected: None,
                    sync: WineSync::default(),
                    language: WineLang::default(),
                    borderless: false,
                    virtual_desktop: VirtualDesktop::default(),
                    shared_libraries: SharedLibraries::default()
                }
            }
        }

        impl From<&JsonValue> for Wine {
            fn from(value: &JsonValue) -> Self {
                let default = Self::default();

                Self {
                    prefix: match value.get("prefix") {
                        Some(value) => match value.as_str() {
                            Some(value) => PathBuf::from(value),
                            None => default.prefix
                        },
                        None => default.prefix
                    },

                    builds: match value.get("builds") {
                        Some(value) => match value.as_str() {
                            Some(value) => PathBuf::from(value),
                            None => default.builds
                        },
                        None => default.builds
                    },

                    selected: match value.get("selected") {
                        Some(value) => {
                            if value.is_null() {
                                None
                            } else {
                                match value.as_str() {
                                    Some(value) => Some(value.to_string()),
                                    None => default.selected
                                }
                            }
                        },
                        None => default.selected
                    },

                    sync: match value.get("sync") {
                        Some(value) => WineSync::from(value),
                        None => default.sync
                    },

                    language: match value.get("language") {
                        Some(value) => WineLang::from(value),
                        None => default.language
                    },

                    borderless: match value.get("borderless") {
                        Some(value) => value.as_bool().unwrap_or(default.borderless),
                        None => default.borderless
                    },

                    virtual_desktop: match value.get("virtual_desktop") {
                        Some(value) => VirtualDesktop::from(value),
                        None => default.virtual_desktop
                    },

                    shared_libraries: match value.get("shared_libraries") {
                        Some(value) => SharedLibraries::from(value),
                        None => default.shared_libraries
                    },
                }
            }
        }
    };
}
