//! Observations that inform the methods, with references to the original records.

pub struct Law {
    pub title: &'static str,
    pub statement: &'static str,
    pub evidence: &'static str,
}

pub const LAWS: &[Law] = &[
    Law {
        title: "Review the content selected for memory",
        statement: "Some successful explanations were omitted from active memory because the next message produced a low prediction-error score. Incorrect exchanges could pass the same selection rule. In one recorded incident, an invented address was retrieved and repeated; removing the memory stopped its repetition on the next turn. These observations motivate reviewing correctness separately from prediction error.",
        evidence: "Archive batch 2, rows 231, 236–237, 255, 269–274, 294; batch 3, cycles 70–71; batch 4, cycle 1026",
    },
    Law {
        title: "Test recall and reasoning separately",
        statement: "A stored record improved factual recall in one small comparison while reducing accuracy on judgement questions. Repeated questions may also have influenced the responses. Counting successful retrievals would therefore miss an important part of the outcome: whether the retrieved material helps the system solve the current task.",
        evidence: "Archive batch 2, rows 240–242",
    },
    Law {
        title: "Account for model changes during interaction",
        statement: "Changing an adapter used only for prediction altered the score governing memory selection, even though it was not used to generate replies. A study spanning that change could not attribute subsequent behaviour to the teaching material alone. Observations after adapter changes also motivated the period of memory review described above.",
        evidence: "Cycle 3039, 15 August 2026; adapter change of 7 August 2026; docs/KEEPING.md",
    },
    Law {
        title: "Check the method as well as the final value",
        statement: "Some answers in the 4 September comparison passed exact numerical checks despite changing the requested calculation. A separate AI review rejected them. This motivated retaining both the numerical check and a review of the complete solution, with the limitations of each stated explicitly.",
        evidence: "Transfer-study review and decision, 4 September 2026",
    },
    Law {
        title: "Compare prompts and output constraints explicitly",
        statement: "Adding calculation steps to the prompt reduced the trained adapter's score in one comparison. A more restrictive output grammar also helped the adapter trained on shuffled answers. Both observations show that prompts and format constraints can affect the measured outcome and need to be included in the comparison design.",
        evidence: "Prompt and output-format comparison, 4 September 2026",
    },
    Law {
        title: "Keep unrelated corrections out of training prompts",
        statement: "An adapter trained on prompts containing raw corrections repeated a number unrelated to the new questions. A follow-up regenerated prompts from the problem specifications and obtained correct answers on the tested problem type without that number appearing. This supports checking the origin and content of training prompts as well as their target answers.",
        evidence: "Training-prompt diagnosis and 18-question follow-up, 3 September 2026",
    },
];
