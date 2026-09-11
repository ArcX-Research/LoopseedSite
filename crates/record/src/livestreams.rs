//! Featured recordings from the owner's Live Science playlist.
//! Titles, broadcast dates and embedding availability checked on 11 September 2026.
pub const PLAYLIST_URL: &str =
    "https://www.youtube.com/playlist?list=PLX7vesicHA47pr2ex_oCADd00MvaeoPyv";

pub struct Livestream {
    pub video_id: &'static str,
    pub title: &'static str,
    pub theme: &'static str,
    pub description: &'static str,
    pub date: &'static str,
    pub datetime: &'static str,
    pub duration: &'static str,
}

pub const FEATURED: &[Livestream] = &[
    Livestream {
        video_id: "xhSWNRaKHz8",
        title: "Teaching AI to Survive Chaotic Worlds: Adaptive Predictive Coding",
        theme: "Prediction and adaptation",
        description: "Prediction, adaptation and performance measurement in environments whose rules change.",
        date: "1 December 2025",
        datetime: "2025-12-01",
        duration: "1h 25m",
    },
    Livestream {
        video_id: "qhwFHNvMXEA",
        title: "Event-modulated Plasticity: Adaptive Predictive Coding",
        theme: "Learning under change",
        description: "A research session on event-modulated plasticity: how changes in an environment can influence the way a system learns.",
        date: "12 December 2025",
        datetime: "2025-12-12",
        duration: "1h 33m",
    },
    Livestream {
        video_id: "DXFEO41DoNQ",
        title: "Multi-agent Cultural Dynamics: Adaptive Predictive Coding",
        theme: "Interaction between systems",
        description: "A research session on multi-agent cultural dynamics, examining interactions between systems that learn and adapt.",
        date: "23 December 2025",
        datetime: "2025-12-23",
        duration: "1h 46m",
    },
];
