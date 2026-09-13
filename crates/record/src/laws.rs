//! Observations that inform the methods, with references to the original records.

pub struct Law {
    pub title: &'static str,
    pub statement: &'static str,
    pub evidence: &'static str,
}

pub const LAWS: &[Law] = &[
    Law {
        title: "Memory selection and correctness",
        statement: "Low prediction error excluded some helpful explanations from active memory, while incorrect exchanges could pass the selection rule. In one incident, an invented address was retrieved and repeated until the memory was removed; repetition stopped on the next turn. These observations support checking correctness separately from prediction error.",
        evidence: "Archive batch 2, rows 231, 236–237, 255, 269–274, 294; batch 3, cycles 70–71; batch 4, cycle 1026",
    },
    Law {
        title: "Recall and task performance",
        statement: "A stored record improved recall but reduced accuracy on judgement questions in a small comparison. Repeated questions may also have influenced the answers. This distinction supports separate tests of retrieving information and using it to solve a task.",
        evidence: "Archive batch 2, rows 240–242",
    },
    Law {
        title: "Adapter changes during interaction",
        statement: "Changing the prediction adapter altered the score used to select memories, despite leaving the response adapter unchanged. Later behaviour could therefore reflect both the teaching material and the adapter change. These observations also informed the review period for new memories after an update.",
        evidence: "Cycle 3039, 15 August 2026; adapter change of 7 August 2026; docs/KEEPING.md",
    },
    Law {
        title: "Numerical accuracy and solution validity",
        statement: "Some answers in the 4 September comparison passed numerical checks after changing the requested calculation. AI review rejected them, showing why the complete solution needed assessment as well as its final values.",
        evidence: "Transfer-study review and decision, 4 September 2026",
    },
    Law {
        title: "Prompts and output requirements",
        statement: "Supplied calculation steps reduced the response adapter’s score in one comparison, while stricter output rules helped the adapter trained on mismatched answers. Both effects supported including prompt and output conditions in the study design.",
        evidence: "Prompt and output-format comparison, 4 September 2026",
    },
    Law {
        title: "Corrections in training prompts",
        statement: "Training prompts containing corrections were linked to repetition of a number unrelated to new questions. A follow-up using prompts generated from problem specifications produced correct answers on the tested problem type without that number. The result supported reviewing the prompts as well as the target answers.",
        evidence: "Training-prompt diagnosis and 18-question follow-up, 3 September 2026",
    },
];
