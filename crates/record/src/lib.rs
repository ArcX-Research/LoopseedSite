//! The record behind loopseed.io.
//!
//! Every number on the site is typed here and quoted from a frozen ruling in the loopseed
//! repository, named in each item's `source`. The statistics that rule on those numbers are
//! reimplemented in [`stats`] so a reader can recompute them from the frozen counts, and the
//! tests in this crate check that the recomputation reproduces the frozen values.

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

/// The repository that holds the sediment, the album and the harnesses. `None` while the study
/// keeps it private; the site then says so instead of linking.
pub const REPOSITORY_URL: Option<&str> = None;

/// A mailbox for evidence requests. `None` routes readers to the organisation site.
pub const CONTACT_EMAIL: Option<&str> = None;

/// Where the record lives inside the loopseed repository, so a path on the site can be found.
pub const ALBUM_PATH: &str = "fish/album/";
pub const LAWS_PATH: &str = "docs/LAWS.md";
