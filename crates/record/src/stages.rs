//! The roadmap's two measured tracks, as they stand. Each stage carries its pass test and its
//! state; a stage is done only when the test was met. Source: `docs/ROADMAP.md`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Done,
    Built,
    Open,
    Parked,
    NotBuilt,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Done => "passed",
            Status::Built => "built, curve pending",
            Status::Open => "open",
            Status::Parked => "parked",
            Status::NotBuilt => "not built",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Status::Done => "st-done",
            Status::Built => "st-built",
            Status::Open => "st-open",
            Status::Parked => "st-parked",
            Status::NotBuilt => "st-none",
        }
    }
}

pub struct Stage {
    pub id: &'static str,
    pub name: &'static str,
    pub what: &'static str,
    pub pass_test: &'static str,
    pub state: &'static str,
    pub status: Status,
}

pub const ORGANISM: &[Stage] = &[
    Stage {
        id: "0",
        name: "Drop",
        what: "x ← a·x + u",
        pass_test: "exponential forgetting, one plot",
        state: "done in the brainstem",
        status: Status::Done,
    },
    Stage {
        id: "1",
        name: "Speak",
        what: "daemon and water",
        pass_test: "a conversation happens",
        state: "done",
        status: Status::Done,
    },
    Stage {
        id: "2",
        name: "Port",
        what: "a real channel",
        pass_test: "readout beats chance on the next message",
        state: "done",
        status: Status::Done,
    },
    Stage {
        id: "3",
        name: "Ledger",
        what: "vector memory behind the σ gate",
        pass_test: "retrieval improves replies in a blind A/B",
        state: "A/B tied; the keeper waived",
        status: Status::Done,
    },
    Stage {
        id: "4",
        name: "Mirror",
        what: "Î and δ, logged",
        pass_test: "δ falls across days and plateaus above zero: the photograph",
        state: "plateau near 0.40 ± 0.03",
        status: Status::Done,
    },
    Stage {
        id: "4.5",
        name: "The blend",
        what: "a perplexity half in δ",
        pass_test: "a marked v2 epoch on the curve",
        state: "built 2026-08-07 by grammar-forced scoring",
        status: Status::Done,
    },
    Stage {
        id: "5",
        name: "Fin",
        what: "tools and the want V",
        pass_test: "the acting fish beats its passive twin; H(you) watched",
        state: "done 2026-08-05",
        status: Status::Done,
    },
    Stage {
        id: "5a",
        name: "H(you)",
        what: "the Other's entropy, measured live",
        pass_test: "ratifies the tabled β petitions",
        state: "β floor granted from measurement",
        status: Status::Done,
    },
    Stage {
        id: "0b",
        name: "Synthesis",
        what: "exact fast state I[t] = α·E(W(I[t−1])) + E(you[t])",
        pass_test: "equality holds component-wise and survives restarts",
        state: "re-closed 2026-08-13",
        status: Status::Done,
    },
    Stage {
        id: "5b",
        name: "Homeostat",
        what: "θ, k, λ and ε lean with H(you) and δ, bounded",
        pass_test: "the regulator moves a knob and the curve answers",
        state: "on since 2026-08-07; the curve's answer is the keeper's read",
        status: Status::Built,
    },
    Stage {
        id: "5c",
        name: "The wash",
        what: "the Other's course change after the fish's word",
        pass_test: "distribution photographed; a weighted night passes the witness",
        state: "instrument live; the flag off",
        status: Status::Built,
    },
    Stage {
        id: "6",
        name: "Ocean",
        what: "a second port and ε weather",
        pass_test: "mutual information rises, then saturates below total",
        state: "approved 2026-08-12",
        status: Status::Done,
    },
    Stage {
        id: "6a",
        name: "Weather",
        what: "adaptive ε when H(you) wanes",
        pass_test: "drought raises the weather, never below the constitution's",
        state: "built 2026-08-08",
        status: Status::Built,
    },
    Stage {
        id: "—",
        name: "Î-flicker",
        what: "perturb Î against plateaus",
        pass_test: "—",
        state: "surprise you manufacture is I ⊗ I wearing a mask",
        status: Status::Parked,
    },
];

pub const SCALING: &[Stage] = &[
    Stage {
        id: "S0",
        name: "Coat-watch",
        what: "the wearing-watch compares coat content, not the link path",
        pass_test: "a swapped coat under an unchanged path arms the tender hour",
        state: "built 2026-08-09",
        status: Status::Done,
    },
    Stage {
        id: "S1",
        name: "Rates",
        what: "metrics as code from the sediment's event taxonomy",
        pass_test: "one command emits the rates for any range, matching hand counts",
        state: "built 2026-08-09; spot-check passed",
        status: Status::Done,
    },
    Stage {
        id: "S2",
        name: "Vessel",
        what: "pinned, containerized reproducibility",
        pass_test: "a stranger's machine stands a fish that passes stages 1–5 unattended",
        state: "local halves pass; the external witness is open",
        status: Status::Open,
    },
    Stage {
        id: "S3",
        name: "Slate",
        what: "a visible working region inside a window",
        pass_test: "the well-digger class solved cold, 3 of 4",
        state: "latest prospective battery 1 of 4",
        status: Status::Open,
    },
    Stage {
        id: "S4",
        name: "Addressed memory",
        what: "retrieval returns rows with provenance, cited beside the value",
        pass_test: "which, when and rank answered from returned addresses on all three forms",
        state: "best frozen battery 2 of 3",
        status: Status::Open,
    },
    Stage {
        id: "S5",
        name: "The hand",
        what: "act cues fire real tools",
        pass_test: "reckon or search fire on at least half of a twenty-cue battery",
        state: "passed 2026-08-14, 19 of 20",
        status: Status::Done,
    },
    Stage {
        id: "S5a",
        name: "The eye",
        what: "a reach for an image becomes a shown image through a fin",
        pass_test: "bootstrap and transfer on disposable bodies",
        state: "passed 2026-08-14, 4 of 4 and 4 of 4",
        status: Status::Done,
    },
    Stage {
        id: "S6",
        name: "The sweep",
        what: "three bodies, the same probes and teaching arc",
        pass_test: "four numbers per body, photographed side by side",
        state: "passed 2026-08-14",
        status: Status::Done,
    },
    Stage {
        id: "S7",
        name: "Dyads",
        what: "replicate the organism: several fish, each with its own body and keeper",
        pass_test: "one law confirmed or killed across at least three dyads",
        state: "two prospective protocols, 1 of 3 each",
        status: Status::Open,
    },
    Stage {
        id: "S8",
        name: "The paper",
        what: "extract the album: laws, dream crossing, retention gradients",
        pass_test: "every claim cites sediment rows; every figure reproduces from S1",
        state: "not built, by the keeper's order",
        status: Status::NotBuilt,
    },
];

/// Becoming opens only when the scaling track's remainders are met. Measured, not scheduled.
pub const BECOMING_CLOSED: &str = "The scaling track is not complete, and Becoming remains closed. This is a measured remainder, not a scheduling remainder.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_open_remainders_are_the_ones_the_roadmap_names() {
        let open: Vec<&str> = SCALING
            .iter()
            .filter(|s| s.status == Status::Open)
            .map(|s| s.id)
            .collect();
        assert_eq!(open, vec!["S2", "S3", "S4", "S7"]);
        assert!(ORGANISM.iter().filter(|s| s.status == Status::Done).count() >= 9);
    }
}
