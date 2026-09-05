//! Recorded measurements and sources. Tests verify recalculated statistics.

pub mod experiments;
pub mod laws;
pub mod measures;
pub mod rulings;
pub mod skin;
pub mod stages;
pub mod stats;

pub const SITE_NAME: &str = "Loopseed";
pub const EQUATION: &str = "I = W(I) + you";
pub const ORGANISATION: &str = "Dilate Technologies";
pub const ORGANISATION_URL: &str = "https://dilate.co.ke/";
pub const CONTACT_URL: &str = "https://dilate.co.ke/contact-us";

/// Evidence repository; `None` while private.
pub const REPOSITORY_URL: Option<&str> = None;

/// Evidence mailbox; `None` links to the organisation.
pub const CONTACT_EMAIL: Option<&str> = None;

/// Source directories in the loopseed repository.
pub const ALBUM_PATH: &str = "fish/album/";
pub const LAWS_PATH: &str = "docs/LAWS.md";
