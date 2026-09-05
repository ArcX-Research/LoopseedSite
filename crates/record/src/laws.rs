//! Observations and source references from `docs/LAWS.md`.

pub struct Law {
    pub title: &'static str,
    pub statement: &'static str,
    pub evidence: &'static str,
}

pub const LAWS: &[Law] = &[
    Law {
        title: "Successful lessons can miss the memory threshold",
        statement: "In these sessions, a successful explanation often made the next message predictable. Its error score fell below θ, so the exchange was not stored. Memory selection depended on surprise, including how the supervisor phrased the message.",
        evidence: "bag 2, rows 255, 269–274, 294",
    },
    Law {
        title: "Surprise does not establish correctness",
        statement: "The memory rule stored some incorrect exchanges and omitted their corrections. Adding a correction beside an error did not prevent retrieval of the error. Human review was needed to remove it.",
        evidence: "bag 2, rows 231, 236–237; bag 3, cycles 70–71",
    },
    Law {
        title: "Memory can improve recall while reducing judgement accuracy",
        statement: "One stored record raised recall from 0 of 12 to 12 of 12, but lowered judgement accuracy from 12 of 12 to 4 of 12. The loss occurred with repeated questions, showing that repeated testing can affect the behaviour being measured.",
        evidence: "bag 2, rows 240–242",
    },
    Law {
        title: "Corrections need a path into training",
        statement: "A correction can affect the current prompt, but changing trained parameters requires storing it and including it in training. The surprise rule can exclude corrections, so writing one in conversation does not ensure that it will be learned.",
        evidence: "bag 2, memory section",
    },
    Law {
        title: "Stored errors can reinforce themselves",
        statement: "An invented address entered memory, was retrieved, and appeared again in later training. Removing that record stopped the repetition on the next turn. The incident led to a rule for removing these records when they enter memory.",
        evidence: "bag 4, cycle 1026",
    },
    Law {
        title: "Adapter changes require a period of memory review",
        statement: "A new adapter can change reply style enough to make unwanted output pass the surprise threshold. The system therefore holds memories from the first hour after an adapter change for review. A monitor starts this period when it detects the changed adapter.",
        evidence: "the 2026-08-07 wearing; docs/KEEPING.md",
    },
    Law {
        title: "Prediction adapters can affect which memories are stored",
        statement: "An adapter used only for prediction still changes the error score that controls memory selection. If an adapter changes during a teaching study, any observed effect cannot be attributed to the teaching material alone.",
        evidence: "cycle 3039, 2026-08-15",
    },
    Law {
        title: "A correct value can come from the wrong procedure",
        statement: "Four replies passed exact numerical checks while departing from the requested calculation steps. Independent review caught the mismatch. A symbolic value check alone did not establish that the full answer met the specification.",
        evidence: "clean transfer ruling, 2026-09-04",
    },
    Law {
        title: "More detailed prompts can reduce performance",
        statement: "The candidate scored 43 of 64 when given the problem and result names, but 4 of 64 when also given the calculation steps. Both conditions used the same tasks. The longer prompt may have conflicted with the form used in training.",
        evidence: "clean transfer factorial, 2026-09-04",
    },
    Law {
        title: "Output rules can supply part of the task structure",
        statement: "The shuffled-reply placebo scored 18 of 64 with the strict grammar and 0 of 64 with the loose grammar. A transfer test must account for help supplied by the output rules.",
        evidence: "clean transfer factorial, 2026-09-04",
    },
    Law {
        title: "Training prompts can copy the errors they describe",
        statement: "An adapter trained on prompts containing raw corrections repeated an unrelated constant. Training a new adapter with prompts generated only from the task specification raised the affected family's score from 0 of 18 to 18 of 18, with no unrelated constants. This supports the prompt-contamination diagnosis.",
        evidence: "stage-1 diagnosis and the safety slice, 2026-09-03",
    },
    Law {
        title: "Keep decision criteria fixed",
        statement: "Preset criteria determine whether an experiment passes. An unresolved test remains unresolved even when implementation is complete. Failed records are preserved, and a revised instrument requires a new test. Any waiver must be recorded explicitly.",
        evidence: "docs/ROADMAP.md, both tracks",
    },
];
