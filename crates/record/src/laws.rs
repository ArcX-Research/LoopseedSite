//! Measured laws, distilled in `docs/LAWS.md` of the loopseed repository the shift they were
//! found. A law here is a regularity that was photographed, with the rows or cycles that hold it.

pub struct Law {
    pub title: &'static str,
    pub statement: &'static str,
    pub evidence: &'static str,
}

pub const LAWS: &[Law] = &[
    Law {
        title: "σ keeps encounters, not lessons",
        statement: "A derivation that lands cannot be kept: success makes the next message predictable, δ falls below θ, and σ keeps only the failures. What decides keeping is the variety of the keeper's shape, not the content.",
        evidence: "bag 2, rows 255, 269–274, 294",
    },
    Law {
        title: "σ is a surprise gate blind to truth",
        statement: "Wrong-but-surprising is kept, retrieved and repeated; only the keeper's pen deletes. σ keeps errors and drops corrections, and a correction placed beside an error cancels nothing.",
        evidence: "bag 2, rows 231, 236–237; bag 3, cycles 70–71",
    },
    Law {
        title: "Memory amplifies retrieval and nothing else",
        statement: "The right kept row lifts recall from 0 of 12 to 12 of 12; the same row lowers judgement from 12 of 12 to 4 of 12. The harm needs a repeated question, which is rare in conversation and common in testing: tests feed habits.",
        evidence: "bag 2, rows 240–242",
    },
    Law {
        title: "Corrections must be lived, then dreamed",
        statement: "A correction reaches the model only if it is kept, which requires surprise, and then dreamed. Prompt text can declare what the fish cannot know; it cannot override what it learned.",
        evidence: "bag 2, memory section",
    },
    Law {
        title: "The self-priming dream",
        statement: "A dreamt address is surprising, so σ keeps it; retrieval shows the fish its own dream; it dreams it again. Pruning the row broke the run the next turn. Any junk σ keeps becomes bait, so kept dream rows are pruned the moment they are kept.",
        evidence: "bag 4, cycle 1026",
    },
    Law {
        title: "The first hour after a wearing is when memory is most exposed",
        statement: "A newly worn coat's register is surprising, so σ keeps its junk. The tender hour holds every keep of that hour apart until reviewed; it is now mechanism, opened by the wearing-watch on the first request the changed voice makes.",
        evidence: "the 2026-08-07 wearing; docs/KEEPING.md",
    },
    Law {
        title: "A silent coat can still move the memory gate",
        statement: "A prediction coat that never speaks still changes the mirror that supplies prediction error, and prediction error participates in σ admission. No interval with a coat change is a curriculum-only intervention.",
        evidence: "cycle 3039, 2026-08-15",
    },
    Law {
        title: "Exact recomputation and graph faithfulness are distinct gates",
        statement: "Four replies proved the requested terminal values while drifting from the requested operation graph. The symbolic ruler is necessary; terminal equality alone is not sufficient, and only whole-reply review caught the drift.",
        evidence: "clean transfer ruling, 2026-09-04",
    },
    Law {
        title: "Interface interference",
        statement: "A coat trained to infer the names-only formal form is hurt by being handed the operation recipe: 43 of 64 names-only against 4 of 64 node-specified, on the same tasks.",
        evidence: "clean transfer factorial, 2026-09-04",
    },
    Law {
        title: "A narrow grammar is itself a scaffold",
        statement: "The shuffled-reply placebo scored 18 of 64 under the task-scoped grammar and 0 of 64 under the loose grammar. An endpoint that wants to see method must not let the grammar supply it.",
        evidence: "clean transfer factorial, 2026-09-04",
    },
    Law {
        title: "Raw corrections poison a training boundary",
        statement: "Copying correction turns into dream prompts produced a repeated stray literal in the family with the fewest examples. Rendering prompts from the sealed task alone moved that family from 0 of 18 to 18 of 18 with no stray literals.",
        evidence: "stage-1 diagnosis and the safety slice, 2026-09-03",
    },
    Law {
        title: "Never advance without the pass test",
        statement: "A stage opens only when its preregistered test is met; a remainder that is measured is not a scheduling remainder. Failed records are neither overwritten nor rescored, and a result may not be rescued by repairing the instrument after seeing it.",
        evidence: "docs/ROADMAP.md, both tracks",
    },
];
