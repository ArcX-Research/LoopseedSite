//! Stages, criteria and recorded status from `docs/ROADMAP.md`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Done,
    Built,
    Open,
    Parked,
    Available,
    NotBuilt,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Done => "closed",
            Status::Built => "built; evaluation pending",
            Status::Open => "open",
            Status::Parked => "paused",
            Status::Available => "paper available",
            Status::NotBuilt => "not built",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Status::Done => "st-done",
            Status::Built | Status::Available => "st-built",
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
        name: "State decay",
        what: "combine a weighted previous state with new input",
        pass_test: "a plot shows exponential decay of past input",
        state: "implemented in the core state update",
        status: Status::Done,
    },
    Stage {
        id: "1",
        name: "Model connection",
        what: "connect the service to the language model",
        pass_test: "the service sends a message and receives a model reply",
        state: "done",
        status: Status::Done,
    },
    Stage {
        id: "2",
        name: "Input channel",
        what: "receive external messages",
        pass_test: "next-message prediction exceeds chance",
        state: "done",
        status: Status::Done,
    },
    Stage {
        id: "3",
        name: "Memory",
        what: "select exchanges for retrieval using prediction error",
        pass_test: "retrieval improves replies in a blinded A/B test",
        state: "A/B test tied; closed by an explicit supervisor waiver",
        status: Status::Done,
    },
    Stage {
        id: "4",
        name: "Prediction",
        what: "record predictions Î and errors δ",
        pass_test: "prediction error falls across days, then levels off above zero",
        state: "plateau near 0.40 ± 0.03",
        status: Status::Done,
    },
    Stage {
        id: "4.5",
        name: "Combined error measure",
        what: "combine token loss and embedding distance in the prediction score",
        pass_test: "mark the change to v2 on the chart",
        state: "implemented 2026-08-07 using constrained scoring output",
        status: Status::Done,
    },
    Stage {
        id: "5",
        name: "Tool use",
        what: "invoke tools and record the score V",
        pass_test:
            "the system with actions outperforms a passive control; input variety is monitored",
        state: "done 2026-08-05",
        status: Status::Done,
    },
    Stage {
        id: "5a",
        name: "Input variety",
        what: "measure dispersion among one source's recent messages, H(you)",
        pass_test: "use measurements to assess proposed changes to β",
        state: "measurements supported a minimum value for β",
        status: Status::Done,
    },
    Stage {
        id: "0b",
        name: "Dynamical Synthesis",
        what: "combine the actual previous reply and current input for memory retrieval",
        pass_test: "each vector component matches the equation, including after restarts",
        state: "verified again on 2026-08-13",
        status: Status::Done,
    },
    Stage {
        id: "5b",
        name: "Parameter regulation",
        what: "adjust θ, k, λ and ε within limits using input variety and prediction error",
        pass_test: "a parameter change produces a measured response",
        state: "implemented 2026-08-07; its effect remains to be evaluated",
        status: Status::Built,
    },
    Stage {
        id: "5c",
        name: "Response effects",
        what: "measure how the next input differs from the source's recent messages",
        pass_test:
            "record the distribution; training weighted by this measure passes held-out evaluation",
        state: "measurement implemented; weighted training disabled in the recorded configuration",
        status: Status::Built,
    },
    Stage {
        id: "6",
        name: "External input",
        what: "a second channel supplies input at rate ε",
        pass_test: "estimated mutual information rises, then levels off below its maximum",
        state: "approved 2026-08-12",
        status: Status::Done,
    },
    Stage {
        id: "6a",
        name: "Adaptive input rate",
        what: "adjust ε as incoming messages become less varied",
        pass_test:
            "lower variety raises external input; the rate stays above its configured minimum",
        state: "built 2026-08-08",
        status: Status::Built,
    },
    Stage {
        id: "—",
        name: "Prediction perturbation",
        what: "alter predictions when error levels off",
        pass_test: "—",
        state: "paused because it would create artificial prediction error",
        status: Status::Parked,
    },
];

pub const SCALING: &[Stage] = &[
    Stage {
        id: "S0",
        name: "Adapter monitoring",
        what: "detect adapter changes by file content",
        pass_test: "replacing an adapter at the same path starts the memory-review period",
        state: "built 2026-08-09",
        status: Status::Done,
    },
    Stage {
        id: "S1",
        name: "Metrics",
        what: "calculate metrics from classified database events",
        pass_test: "one command reports rates for a chosen period and matches manual counts",
        state: "built 2026-08-09; spot-check passed",
        status: Status::Done,
    },
    Stage {
        id: "S2",
        name: "Reproducible setup",
        what: "package exact software versions in a container",
        pass_test: "an independent machine runs Fish and passes stages 1–5 unattended",
        state: "local tests pass; independent setup remains unverified",
        status: Status::Open,
    },
    Stage {
        id: "S3",
        name: "Working area",
        what: "a visible area for intermediate state within a session",
        pass_test: "solve 3 of 4 well-digger state-tracking problems without prior teaching",
        state: "latest prospective test: 1 of 4",
        status: Status::Open,
    },
    Stage {
        id: "S4",
        name: "Memory with source references",
        what: "return source records alongside retrieved values",
        pass_test: "answer which record, when it was stored and its rank in all three test forms",
        state: "best fixed test set: 2 of 3",
        status: Status::Open,
    },
    Stage {
        id: "S5",
        name: "Tool activation",
        what: "action requests invoke the intended tools",
        pass_test: "calculation or search runs for at least half of twenty test requests",
        state: "passed 2026-08-14, 19 of 20",
        status: Status::Done,
    },
    Stage {
        id: "S5a",
        name: "Image display",
        what: "image requests invoke a tool that displays an image",
        pass_test: "initial and transfer tests pass on isolated copies",
        state: "passed 2026-08-14, 4 of 4 and 4 of 4",
        status: Status::Done,
    },
    Stage {
        id: "S6",
        name: "Model comparison",
        what: "three models use the same test questions and teaching sequence",
        pass_test: "record four comparable metrics per model",
        state: "passed 2026-08-14",
        status: Status::Done,
    },
    Stage {
        id: "S7",
        name: "Independent pairs",
        what: "repeat the study with separate model–supervisor pairs",
        pass_test: "test one predicted pattern across at least three pairs",
        state: "two prospective protocols; each confirmed the pattern in 1 of 3 pairs",
        status: Status::Open,
    },
    Stage {
        id: "S8",
        name: "Research paper",
        what: "report observed patterns, training effects and retention",
        pass_test: "every claim cites database records; every figure reproduces using S1 metrics",
        state: "paper, eight figures and supporting data available; full reporting criterion remains unverified",
        status: Status::Available,
    },
];

/// Existing criteria must pass before Becoming experiments begin.
pub const BECOMING_CLOSED: &str = "The next experimental phase has not begun. Independent setup, reliable state tracking, memory references and replication checks remain unresolved.";

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
