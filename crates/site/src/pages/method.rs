//! Equations, implementation and evaluation methods.
use crate::components::page_nav::PageNav;
use crate::components::tables::ScrollTable;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::laws::LAWS;

const SECTIONS: &[(&str, &str)] = &[
    ("#eq-h", "Measurements"),
    ("#night-h", "Training"),
    ("#instruments-h", "Verification"),
    ("#context-h", "Scientific context"),
    ("#laws", "Observations"),
];

const EXEGESIS: &str = "I[t] = α · E(r[t−1]) + E(you[t])
δ_v1 = 1 − cos(E(Î), E(you))
δ_v2 = ½ · (1 − exp(−L)) + ½ · δ_v1";

struct Organ {
    symbol: &'static str,
    said: &'static str,
    what: &'static str,
}

const ORGANS: &[Organ] = &[
    Organ { symbol: "I, E, r", said: "I, E, r", what: "I is the conversation vector used for retrieval. E embeds text in 384 dimensions; r is an actual generated reply. The prior reply's embedding is retained across room restarts. This vector is one component of Fish's state, alongside its context, database and model parameters." },
    Organ { symbol: "W", said: "W", what: "The model's response-generating transformation in the design notation. Actual generation also depends on the prompt, retrieved context, adapter and sampling settings." },
    Organ { symbol: "you", said: "you", what: "The incoming message from an external participant or source." },
    Organ { symbol: "Ŵ, Î", said: "W-hat, I-hat", what: "The predictor and its estimate of the next incoming message. The estimate is recorded before the message arrives." },
    Organ { symbol: "δ, L", said: "delta, L", what: "δ is a dimensionless prediction-discrepancy score. L is the mean negative log probability of the incoming message's tokens, measured from the predictor's prior context. The project calls 1 − exp(−L) normalised perplexity; it is a bounded transform of token loss, not a probability that the reply is incorrect." },
    Organ { symbol: "σ, θ", said: "sigma, theta", what: "The surprise-based memory selection rule and its threshold. δ > θ selects an exchange for active memory, subject to admission and review rules. The reference θ is 0.35; regulation can adjust it within limits. Surprise does not certify truth." },
    Organ { symbol: "M, λ", said: "M, lambda", what: "The active memory subset and its weight-decay factor. The default nightly multiplier is 0.995; regulation can slow decay and retrieval can reinforce a record. Eviction from active memory preserves the underlying exchange history." },
    Organ { symbol: "π, V", said: "pi, V", what: "π names action selection in the design. V = −|δ| + β·H(you) is a recorded score balancing prediction discrepancy and input variety. The language model is not trained by reinforcement learning to maximise this score. Tools have separate operating rules." },
    Organ { symbol: "H(you), β", said: "H of you, beta", what: "H(you) is the mean pairwise embedding distance among one speaker's last twelve messages, with at least four needed. It measures dispersion, not Shannon entropy. β varies between its configured floor and twice that floor in the recorded V score; this does not itself implement an exploration policy." },
    Organ { symbol: "ε", said: "epsilon", what: "The configured rate of the additional external-input channel. Input provenance and admission rules determine which sources may enter memory or training." },
    Organ { symbol: "α, k", said: "alpha, k", what: "α weights the previous reply embedding (reference value 0.70). k sets the retrieval count (reference value 6). Protocols record the settings used; a coefficient below one alone does not prove that the complete system is a contraction." },
];

#[component]
pub fn Method() -> impl IntoView {
    set_title("Method");
    view! {
        <section class="page-intro page-intro-method">
            <div class="wrap">
                <p class="eyebrow">"Method"</p>
                <h1 class="display display-xl">"From a design idea to a testable learning system"</h1>
                <p class="lede">"Dynamical Synthesis is Loopseed's name for organising a feedback loop between system activity and external input. Fish makes parts of that idea operational through a conversation state, prediction scores, selective memory, tools and offline adapter training. Each mechanism needs its own comparison and measure of success."</p>
                <p class="prose-p">"The shorthand I = W(I) + you expresses the design idea. It does not by itself specify an algorithm, establish stability, or demonstrate intelligence. The equations below describe the implemented conversation vector and prediction instruments. Learning changes model parameters through gradient-based training, using separate objectives for prediction and replies."</p>
                <p class="prose-p">"Project records use several short names: the base model is ‘water’, an adapter is a ‘coat’, offline training is a ‘dream’, and loading an adapter is ‘wearing’. The ‘body’ includes the stored system state; the ‘keeper’ is the human supervisor. We use their technical meanings throughout this site."</p>
            </div>
        </section>

        <PageNav items=SECTIONS/>
        <section class="wrap section" aria-labelledby="eq-h">
            <div class="section-head">
                <p class="eyebrow">"Equations and terms"</p>
                <h2 id="eq-h" class="display">"What the implementation measures"</h2>
                <p class="lede-sm">"At turn t, the conversation vector combines the embedded previous reply r with the new incoming message, you. Î is a predicted next message, recorded before the observation. The first error instrument compares embeddings; the second also includes token loss L. These quantities are defined for this implementation and are not interchangeable with task accuracy."</p>
            </div>
            <p class="source mono">"implementation: fish/daemon/src/fast/state.rs · fish/daemon/src/skin/mod.rs · fish/daemon/src/want/mod.rs · SOUL.md"</p>
            <p class="prose-p">"The two δ versions are recorded separately, including fallback to v1 when token scoring is unavailable. Held-out token loss used to assess training is a different measurement from the δ time series. A reduction in either does not prove a contraction bound for the complete system. That would require a specified state space and a bound on how the update changes distances between states."</p>
            <pre class="exegesis mono">{EXEGESIS}</pre>
            <ScrollTable label="Symbols and measurement definitions">
                <table class="table organs">
                    <thead><tr><th>"Symbol"</th><th>"Pronunciation"</th><th>"Meaning"</th></tr></thead>
                    <tbody>
                        {ORGANS.iter().map(|o| view! {
                            <tr><td class="serif sym">{o.symbol}</td><td class="muted">{o.said}</td><td>{o.what}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
        </section>

        <section class="wrap section" aria-labelledby="night-h">
            <div class="section-head">
                <p class="eyebrow">"Offline training"</p>
                <h2 id="night-h" class="display">"Separate adapters for prediction and replies"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"lean"</span>
                    <h3>"Prediction adapter"</h3>
                    <p>"A low-rank adapter learns the next incoming message from admitted exchanges; the base model stays fixed. Only the target message contributes to training loss. The standard process holds out every fifth eligible pair within each attributed speaker, with a fallback for small groups. Continuation studies retain their earlier held-out sets and specify additional retention checks. This adapter serves the predictor."</p>
                </div>
                <div class="card">
                    <span class="n">"speak"</span>
                    <h3>"Reply adapter"</h3>
                    <p>"A separate adapter learns to produce answers. The September formal studies used checked model replies and prompts generated from task specifications. The later private study uses explicitly identified, teacher-derived worked solutions. Both require direct answer tests: lower validation loss alone does not establish useful transfer."</p>
                </div>
                <div class="card">
                    <span class="n">"memory maintenance"</span>
                    <h3>"Decay and review"</h3>
                    <p>"Memory weights decrease during nightly maintenance. Records below the weight floor leave the active retrieval index, while the original exchange rows remain in the archive. Retrieval can reinforce a record's weight. Memories admitted in the first hour after an adapter change are held for review."</p>
                </div>
            </div>
            <p class="prose-p">"The continuing private study archives experience and trains the two channels separately. A candidate may be activated on that clone only after its preset checks pass; the previous selection remains recoverable. This is distinct from authorising a change to the main live system. "<a href="/results#current">"Current study and evaluation scope"</a>"."</p>
            <p class="source mono">"implementation: fish/night/lean.py · fish/night/mercy.py · fish/lab/private_dream.py · fish/lab/private_dream_data.py"</p>
        </section>

        <section class="wrap section" aria-labelledby="instruments-h">
            <div class="section-head">
                <p class="eyebrow">"Neurosymbolic reasoning"</p>
                <h2 id="instruments-h" class="display">"Propose a formal answer and check it symbolically"</h2>
                <p class="lede-sm">"The formal adapter studies combine a neural model that proposes solutions with symbolic software that checks them. This is a neurosymbolic approach. Their domain-specific language (DSL) represents calculations as typed JSON graphs: every operation and argument must follow explicit rules. Later plain-text studies use separately specified scorers."</p>
                <p class="prose-p">"A deterministic compiler translates each accepted graph into Wolfram expressions. The Wolfram kernel computes the results, which are compared with an exact task generator. Independent AI review checks whether the complete answer follows the requested specification. These are bounded mathematical checks; the lab does not yet generate general formal proofs that an implementation satisfies its specification."</p>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Structured answers."</b>" Fish returns a JSON graph: a sequence of operations using integers and references to earlier results. A grammar limits the allowed output. The strict version fixes result names and argument counts for each task; the loose version fixes only the general structure."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Exact calculation."</b>" A checker validates the graph, then a computer algebra system evaluates it against the task generator's answer. Parentheses preserve the meaning of negative constants. The checker's file hash is fixed before the run."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Isolated tests."</b>" Each request starts from a fresh copy of the same saved state and a new session. Memory retrieval, prediction and tools are disabled. The request record confirms the adapter settings actually used."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Blinded conditions."</b>" Adapters receive randomly assigned labels before testing. Results and independent reviews are fixed and hashed before those labels are decoded."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Separate AI review."</b>" A reviewer with condition labels withheld reads each complete reply that passed the checker. It can reject an answer whose steps do not match the task. The reviewer can also make mistakes; this assessment within the project does not replace external replication or a formal proof."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Preset decision criteria."</b>" Each protocol sets statistical tests, limits on losses within task families, and checks for negative constants, information leaks and invalid requests. The statistical methods are the one-sided exact McNemar test and a Bonferroni-adjusted Clopper–Pearson bound on the difference in success rates. The Results page explains and recalculates them."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="context-h">
            <div class="section-head">
                <p class="eyebrow">"Scientific context"</p>
                <h2 id="context-h" class="display">"Established methods and the question we add"</h2>
                <p class="lede-sm">"Loopseed combines existing techniques in a particular experimental system. Its research question is whether verified interaction can yield durable, transferable improvements under explicit tests for regressions. The architecture's name does not establish novelty or effectiveness."</p>
            </div>
            <div class="prose">
                <p>"Adapters use low-rank parameter updates with frozen base weights, following the approach introduced by Hu and colleagues in "<a href="https://arxiv.org/abs/2106.09685">"LoRA (2021)"</a>". LoRA provides a training mechanism; it does not guarantee useful or lasting learning."</p>
                <p>"Learning new tasks while preserving prior performance is a central continual-learning problem. "<a href="https://doi.org/10.1073/pnas.1611835114">"Kirkpatrick and colleagues (2017)"</a>" study this problem through constraints on parameter updates. Loopseed's current private study instead uses replay and explicit retention tests; it does not implement their elastic weight consolidation method."</p>
                <p>"The formal studies use paired binary outcomes. The "<a href="https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/mcnemar.htm">"NIST description of McNemar's test"</a>" explains the need for mutually independent pairs. Shared task templates can undermine that assumption. "<a href="/results#explorer-h">"Our statistical scope and limitations"</a>" accompany the original calculations."</p>
            </div>
        </section>

        <section id="laws" class="wrap section" aria-labelledby="laws-h">
            <div class="section-head">
                <p class="eyebrow">"Observed patterns"</p>
                <h2 id="laws-h" class="display">"What the experiments suggest"</h2>
                <p class="lede-sm">"The project records these observations in its ‘laws’ file. Several come from small, repeated interaction tests rather than independent samples. They motivate hypotheses and design changes; they are not universal laws. The source labels identify recorded batches, database rows and interaction cycles."</p>
            </div>
            <ol class="laws">
                {LAWS.iter().map(|l| view! {
                    <li class="law">
                        <h3>{l.title}</h3>
                        <p>{l.statement}</p>
                        <span class="mono meta">{l.evidence}</span>
                    </li>
                }).collect_view()}
            </ol>
        </section>
    }
}
