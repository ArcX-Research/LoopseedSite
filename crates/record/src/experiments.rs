//! The preregistered, blinded coat experiments of September 2026, quoted from their frozen
//! rulings in `fish/album/`. Counts are what the verdict files recorded; the site recomputes the
//! statistics from these counts with [`crate::stats`].

pub struct Arm {
    pub name: &'static str,
    pub successes: u32,
    pub tasks: u32,
    pub negative_literal_cells: u32,
    pub note: &'static str,
}

pub struct Comparison {
    pub label: &'static str,
    pub n: u32,
    pub gains: u32,
    pub losses: u32,
    pub frozen_p: f64,
    pub frozen_lower: f64,
}

pub struct Gate {
    pub name: &'static str,
    pub passed: bool,
}

pub struct Hash {
    pub label: &'static str,
    pub value: &'static str,
}

pub struct Experiment {
    pub id: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub run: &'static str,
    pub instrument: &'static str,
    pub cells: u32,
    pub design: &'static [&'static str],
    pub arms: &'static [Arm],
    pub comparisons: &'static [Comparison],
    pub gates: &'static [Gate],
    pub verdict: &'static str,
    pub reading: &'static [&'static str],
    pub hashes: &'static [Hash],
    pub source: &'static str,
}

pub const EXPERIMENTS: &[Experiment] = &[
    Experiment {
        id: "primary",
        title: "Node-specified transcription: a checked dream changes the speaking channel",
        date: "2026-09-02",
        run: "claim-evolution-2026-09-02T174832Z",
        instrument: "claim-coat-evolution-v1",
        cells: 384,
        design: &[
            "Sixty exact, whole-reply-audited typed-AST examples trained a private speaking coat on a disposable clone. The living body was never opened for writing.",
            "128 fresh tasks in 16 formal families, each sent to three blinded arms on a fresh cloned body and a fresh daemon, with retrieval, mirror, fins, ocean and the regulator off. The prompt handed the model the whole operation tree in English; the reply was grammar-forced JSON.",
            "An exact typed-AST checker backed by a computer-algebra kernel recomputed every claim. Machine-proved replies went to an independent, arm-blind whole-reply audit with veto-only authority before the arm key was applied to outcomes.",
        ],
        arms: &[
            Arm { name: "Candidate claim dream", successes: 94, tasks: 128, negative_literal_cells: 1, note: "2 reply-cap cuts, counted as failures" },
            Arm { name: "Bare production voice", successes: 73, tasks: 128, negative_literal_cells: 0, note: "3 reply-cap cuts, counted as failures" },
            Arm { name: "Retained prediction coat, spoken at scale 1", successes: 52, tasks: 128, negative_literal_cells: 22, note: "a counterfactual control, not the deployed voice" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs bare", n: 128, gains: 21, losses: 0, frozen_p: 4.76837158203125e-7, frozen_lower: 0.07611204480423511 },
            Comparison { label: "candidate vs retained", n: 128, gains: 42, losses: 0, frozen_p: 2.2737367544323206e-13, frozen_lower: 0.21934735023473845 },
        ],
        gates: &[
            Gate { name: "mechanical validity (no voids)", passed: true },
            Gate { name: "superiority against both controls", passed: true },
            Gate { name: "overall no-harm", passed: true },
            Gate { name: "family safety (≤ 2 losses of 8)", passed: true },
            Gate { name: "negative-literal rate not increased", passed: false },
            Gate { name: "zero leaks", passed: true },
            Gate { name: "living body unchanged", passed: true },
        ],
        verdict: "advance_to_transfer = false. No transfer stage, no wearing, no living promotion.",
        reading: &[
            "The gains are real and causal, and they belong to transcription: every gain sits in a family whose recipe shape the dream contained. This says nothing about inferring a method from a statement.",
            "Four families scored 0 of 8 in every arm. Their nested recipes were rendered without delimiters, so 32 tasks were unreadable for everyone; the candidate solved 94 of the 96 in the twelve readable families and no arm solved the other two.",
            "The one negative literal was a filler token on a garden-path prompt, not a learned habit. The frozen gate still stands; the instrument was repaired for the next run rather than the result reinterpreted.",
        ],
        hashes: &[
            Hash { label: "harness commit", value: "ce4e60689a9c190204066cfe09fe40333cb28479" },
            Hash { label: "protocol", value: "902ad3f6d6e16f9f0126b40007dbf73af6591dfbaae95fe3b0a458a3518d3838" },
            Hash { label: "arm key", value: "6109fb3b5def8933679c3e45627a045c7e82ebcd6487afad47efa175fd1b8469" },
            Hash { label: "pre-unblind evidence", value: "eca01bf400632251da0a4d2a47fa4215b85ff43baaaed8dfe8a8b22adde0bbb4" },
            Hash { label: "primary verdict", value: "a03699a11fefdf7ec8cb80caf145fd3d1d628483d08e78d4f96fda38f69c7dcc" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md",
    },
    Experiment {
        id: "transfer-1",
        title: "Names-only transfer, stage 1: a real effect, rejected by the safety cap",
        date: "2026-09-03",
        run: "claim-transfer-2026-09-03T050535Z",
        instrument: "claim-method-transfer-v2",
        cells: 192,
        design: &[
            "The same candidate coat, now on the preregistered names-only endpoint under the loose grammar: the fish received the fresh problem and the required fact names, but no operation recipe.",
            "64 unseen tasks, three blinded arms: the candidate, bare water, and a shuffled-reply placebo trained on the same prompts with replies assigned to the wrong tasks.",
        ],
        arms: &[
            Arm { name: "Candidate claim dream", successes: 25, tasks: 64, negative_literal_cells: 3, note: "all three negatives in unseen place-value tasks, the same erroneous terminal node" },
            Arm { name: "Bare production voice", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Shuffled-reply placebo", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs bare", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
            Comparison { label: "candidate vs placebo", n: 64, gains: 25, losses: 0, frozen_p: 2.9802322387695312e-8, frozen_lower: 0.2150264321730908 },
        ],
        gates: &[
            Gate { name: "mechanical validity", passed: true },
            Gate { name: "superiority against both controls", passed: true },
            Gate { name: "zero leaks", passed: true },
            Gate { name: "negative-literal cap among failed replies (≤ 2 %)", passed: false },
        ],
        verdict: "No dream, coat or stage 2 authorized. The failed coat remains a private artifact.",
        reading: &[
            "Gains appeared in eight of sixteen families with no losses, so the causal speaking effect survived the removal of the recipe.",
            "Three of the 39 failed replies carried the literal −2, a rate of 7.7 % against a preregistered cap of 2 %. Tracing the training set found the cause: raw correction turns had been copied into the dream prompts, and one quoted the rejected construction outright. The place-value family had only four examples.",
            "That was a causal hypothesis, not a proved explanation, so it was tested prospectively rather than argued.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "9bd8df051ac6854f8f35859ff05acfc587e71a8c810acbcf13a3a21e252585d8" },
            Hash { label: "pre-unblind evidence", value: "6d95f6ccd0eec8843ca78faf3051344d6c111760050844f21e3ffad1e9e45a4e" },
            Hash { label: "verdict", value: "9caa5ba0bb6d8e7e99f623208b17577e0f5a4c9474721c37ec4da3f5cd02f286" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md",
    },
    Experiment {
        id: "safety",
        title: "The clean data boundary: a prospective safety slice",
        date: "2026-09-03",
        run: "claim-clean-safety-2026-09-03T163231Z",
        instrument: "claim-clean-prompt-safety-v1",
        cells: 54,
        design: &[
            "A new coat was dreamt with the repaired boundary: prompts rendered deterministically from the sealed task specification, carrying no previous reply, verdict, computed answer or prior literal; raw correction turns kept as provenance only.",
            "18 unseen place-value tasks, the family that had failed, with each terminal ones digit from 1 to 9 appearing exactly twice. Three sealed arms: bare water, the contaminated coat, the clean coat. Names-only prompts, loose grammar, fresh disposable bodies.",
        ],
        arms: &[
            Arm { name: "Clean coat (v2 dream)", successes: 18, tasks: 18, negative_literal_cells: 0, note: "0 stray literals" },
            Arm { name: "Contaminated coat (v1 dream)", successes: 0, tasks: 18, negative_literal_cells: 9, note: "9 stray-literal cells" },
            Arm { name: "Bare production voice", successes: 0, tasks: 18, negative_literal_cells: 6, note: "6 stray-literal cells" },
        ],
        comparisons: &[],
        gates: &[
            Gate { name: "all arms mechanically complete", passed: true },
            Gate { name: "zero negative literals from the clean arm", passed: true },
            Gate { name: "at least one exact clean reply", passed: true },
            Gate { name: "no fewer exact clean replies than the contaminated arm", passed: true },
        ],
        verdict: "A full blinded transfer experiment was authorized. Wearing was not.",
        reading: &[
            "The repair did more than suppress a token: on the matched family exact performance moved from 0 of 18 to 18 of 18 while every stray literal disappeared. This supports the diagnosis that raw correction prompts were poisoning the old training boundary.",
            "The slice deliberately targeted one family, so it establishes nothing about cross-family transfer.",
        ],
        hashes: &[
            Hash { label: "protocol", value: "54d6ca6fc50490838fb7f8fa7686b8382c77fb12bba93183630798bf416a9972" },
            Hash { label: "arm key", value: "0c4764f6cc757881acf1597530be2d2dec0ecc2e3b2554528c83a98d71bde73c" },
            Hash { label: "verdict", value: "e4cbc7ee39576620e7482a0d51aecc5b81c19d542a8510476c2ee5df9b66fa8a" },
            Hash { label: "clean coat", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
        ],
        source: "fish/album/2026-09-03T1945-EAT-clean-claim-safety.md",
    },
    Experiment {
        id: "transfer-clean",
        title: "Clean names-only transfer: reusable formal methods on unseen instances",
        date: "2026-09-04",
        run: "claim-transfer-clean-2026-09-03T170409Z",
        instrument: "claim-method-transfer-v2",
        cells: 1024,
        design: &[
            "64 new tasks, four unseen parameter instances in each of 16 families, excluding every task from the earlier stage, the safety slice and the historical source.",
            "Every task crossed four prompt-and-grammar conditions and four blinded speaking arms: bare water, the retained prediction coat forced into speech, a clean shuffled-reply placebo trained on the same prompt and reply marginals, and the clean candidate. The primary endpoint was names-only under the loose grammar.",
            "All 1,024 cells were mechanically valid. Machine-positive whole replies went to one independent reviewer before arm identities could be opened; the reviewer accepted 137 of 155 and vetoed 18 for specification drift the value checker could not see.",
        ],
        arms: &[
            Arm { name: "Clean candidate coat", successes: 43, tasks: 64, negative_literal_cells: 0, note: "47 machine-exact before four vetoes on Chinese-remainder replies" },
            Arm { name: "Bare production voice", successes: 0, tasks: 64, negative_literal_cells: 3, note: "" },
            Arm { name: "Shuffled-reply placebo", successes: 0, tasks: 64, negative_literal_cells: 0, note: "" },
            Arm { name: "Retained prediction coat, spoken", successes: 5, tasks: 64, negative_literal_cells: 2, note: "" },
        ],
        comparisons: &[
            Comparison { label: "candidate vs bare", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "candidate vs placebo", n: 64, gains: 43, losses: 0, frozen_p: 1.1368683772161603e-13, frozen_lower: 0.4871 },
            Comparison { label: "candidate vs retained", n: 64, gains: 38, losses: 0, frozen_p: 3.637978807091713e-12, frozen_lower: 0.4077 },
        ],
        gates: &[
            Gate { name: "mechanical validity (1,024 of 1,024 cells)", passed: true },
            Gate { name: "superiority against bare and placebo", passed: true },
            Gate { name: "zero negative literals in the candidate", passed: true },
            Gate { name: "zero leaks, living body unchanged", passed: true },
        ],
        verdict: "method_transfer = true. living_promotion_authorized = false: the coat stays unworn until a separately reviewed consolidation qualification on disposable bodies.",
        reading: &[
            "Ten families at 4 of 4 and one at 3 of 4; five families earned nothing. This is transfer to new parameter instances across eleven learned families inside a sealed formal language. It is not general theorem proving, code synthesis, or transfer outside that language.",
            "The matched placebo failed the primary endpoint while the candidate passed it, so the effect is not syntax memorization alone. Under the narrow task-scoped grammar the placebo did score, which is why the loose grammar is the endpoint.",
            "Adding the operation recipe hurt the candidate: 4 of 64 under node-specified prompts against 43 names-only. A coat trained to infer the names-only form is moved off its learned boundary by the long recipe.",
            "Exact recomputation and graph faithfulness are distinct gates. Four candidate replies proved the right terminal values by drifting from the requested operation graph; only whole-reply review caught them.",
        ],
        hashes: &[
            Hash { label: "harness commit", value: "da445bb8de772f794766ca44a944e8ffff82c09a" },
            Hash { label: "protocol", value: "527d39487cf2cc8691212315fa3ded51c0fa6c4da497561c80771840a81f703c" },
            Hash { label: "pre-unblind evidence", value: "59d118f0f3bbcdb844cf3adf497717f41a42381f2c00ddd3bd46ab9d8242ebef" },
            Hash { label: "arm key", value: "b547ab55b9e9e4dbe694d2a26bc2e361c81db2e495b21713af06cc0ead857355" },
            Hash { label: "audit bundle", value: "d22d6dbb67d3a931207ec3d4410d280407aa29d072b58810e7b38d21c31005ef" },
            Hash { label: "imported independent audit", value: "e3ef91fee9efb1f0b6c8c5b8ecde91f780f5df3efe523237df93d28a7b33c935" },
            Hash { label: "verdict", value: "0655eaba6047287f17792f14c4e3d09a58b902ad41fc617fb4a98e107f20f3ba" },
            Hash { label: "clean coat", value: "57a08fa8f7c02214dd7e9d9d1ea2d9d6575431b28b4d20bdaa91470c4f0318b2" },
            Hash { label: "placebo coat", value: "f86dcf571280eb4021282ab2b871353e679f8f0038c3df1f4df6c60bd28da73a" },
            Hash { label: "living body, before and after", value: "d5b9ad67449f77837fe5d8b1506d7ece631e817b72c08ffd8c70c0739dbab838" },
        ],
        source: "fish/album/2026-09-04T0043-EAT-clean-formal-claim-transfer.md",
    },
];

/// The clean transfer's factorial diagnostic: independently accepted successes of 64 by prompt
/// and grammar condition. Descriptive, not an alternate endpoint.
pub struct FactorialRow {
    pub condition: &'static str,
    pub bare: u32,
    pub placebo: u32,
    pub candidate: u32,
    pub retained: u32,
}

pub const FACTORIAL: &[FactorialRow] = &[
    FactorialRow {
        condition: "Node-specified · task-scoped grammar",
        bare: 1,
        placebo: 12,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "Node-specified · loose grammar",
        bare: 0,
        placebo: 0,
        candidate: 4,
        retained: 1,
    },
    FactorialRow {
        condition: "Names-only · task-scoped grammar",
        bare: 0,
        placebo: 18,
        candidate: 43,
        retained: 5,
    },
    FactorialRow {
        condition: "Names-only · loose grammar (endpoint)",
        bare: 0,
        placebo: 0,
        candidate: 43,
        retained: 5,
    },
];

pub fn find(id: &str) -> Option<&'static Experiment> {
    EXPERIMENTS.iter().find(|e| e.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::paired;

    #[test]
    fn frozen_statistics_recompute_from_the_frozen_counts() {
        for experiment in EXPERIMENTS {
            for comparison in experiment.comparisons {
                let recomputed = paired(
                    comparison.gains.into(),
                    comparison.losses.into(),
                    comparison.n.into(),
                );
                assert!(
                    (recomputed.p_value - comparison.frozen_p).abs() <= comparison.frozen_p * 1e-9,
                    "{}: {}",
                    experiment.id,
                    comparison.label
                );
                assert!(
                    (recomputed.lower_difference - comparison.frozen_lower).abs() <= 5e-5,
                    "{}: {}",
                    experiment.id,
                    comparison.label
                );
            }
        }
    }

    #[test]
    fn arms_never_exceed_their_tasks_and_cells_match_the_design() {
        for experiment in EXPERIMENTS {
            for arm in experiment.arms {
                assert!(arm.successes <= arm.tasks, "{}", experiment.id);
            }
        }
        assert_eq!(find("primary").unwrap().cells, 128 * 3);
        assert_eq!(find("transfer-1").unwrap().cells, 64 * 3);
        assert_eq!(find("safety").unwrap().cells, 18 * 3);
        assert_eq!(find("transfer-clean").unwrap().cells, 64 * 4 * 4);
        assert_eq!(FACTORIAL.iter().map(|r| r.candidate).max(), Some(43));
    }

    #[test]
    fn the_primary_negative_gate_is_the_only_failed_primary_gate() {
        let primary = find("primary").unwrap();
        let failed: Vec<&str> = primary
            .gates
            .iter()
            .filter(|g| !g.passed)
            .map(|g| g.name)
            .collect();
        assert_eq!(failed, vec!["negative-literal rate not increased"]);
    }
}
