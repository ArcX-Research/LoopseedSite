//! Equations, implementation and evaluation methods.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::laws::LAWS;

const EXEGESIS: &str = "I = W(I) + you                      state: model output plus external input
M ← λM + σ·(I⊗you)                  memory: decay and selective storage
Ŵ ← Ŵ + η(δ⊗I),  δ = I − Î          prediction: update from error
a = π(I),  V = −‖δ‖ + β·H(you)      action: balance error and input variety";

struct Organ {
    symbol: &'static str,
    said: &'static str,
    what: &'static str,
}

const ORGANS: &[Organ] = &[
    Organ { symbol: "I", said: "I", what: "The current system state: I[t] = α·E(W(I[t−1])) + E(you[t]). The function E converts text into a 384-dimensional vector. Each session stores its own state." },
    Organ { symbol: "W", said: "W", what: "The language model that generates replies. Its previous reply contributes to the next system state." },
    Organ { symbol: "you", said: "you", what: "The incoming message from an external participant or source." },
    Organ { symbol: "Ŵ, Î", said: "W-hat, I-hat", what: "The predictor and its estimate of the next incoming message. The estimate is recorded before the message arrives." },
    Organ { symbol: "δ", said: "delta", what: "Prediction error. Since stage 4.5, the score combines equal parts normalised perplexity (how unexpected the text is to the model) and distance between text vectors." },
    Organ { symbol: "σ, θ", said: "sigma, theta", what: "The memory selection rule and its threshold. An exchange is stored when δ > θ. The reference threshold is 0.35; a regulator can adjust it within set limits." },
    Organ { symbol: "M, λ", said: "M, lambda", what: "Stored exchanges and their decay factor. Memory weights are multiplied by 0.995 each night; retrieval offsets the decay for memories that are used." },
    Organ { symbol: "π, V", said: "pi, V", what: "The action policy and its objective. Available tools include memory search, calculation, a visual board and a working area. V = −‖δ‖ + β·H(you)." },
    Organ { symbol: "H(you), β", said: "H of you, beta", what: "The measured variety of the last twelve incoming turns, and its weight in the action objective." },
    Organ { symbol: "ε", said: "epsilon", what: "The fraction of messages from sources other than the two teachers. These inputs are checked and reviewed, and excluded as training targets." },
    Organ { symbol: "α, k", said: "alpha, k", what: "The weight on the previous reply (0.70) and the number of memories retrieved per context window (6)." },
];

#[component]
pub fn Method() -> impl IntoView {
    set_title("Method");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Method"</p>
            <h1 class="display display-xl">"Dynamical Synthesis: how the framework works"</h1>
            <p class="lede">"Dynamical Synthesis describes the repeated combination of a system's evolving state with external input. Loopseed connects that process with prediction error, memory and action. The design does not require language as its input or output. Each application must define these quantities and test how they behave."</p>
            <p class="prose-p">"Fish is the current language-model implementation. It combines a model with prediction, memory and tools, and records measurements for each component. A configuration file sets its operating limits and is excluded from model prompts. The equations describe the general design; the table below gives their current implementation in Fish."</p>
            <p class="prose-p">"Project records use several short names: the base model is ‘water’, an adapter is a ‘coat’, offline training is a ‘dream’, and loading an adapter is ‘wearing’. The ‘body’ includes the stored system state; the ‘keeper’ is the human supervisor. We use their technical meanings throughout this site."</p>
        </section>

        <section class="wrap section" aria-labelledby="eq-h">
            <div class="section-head">
                <p class="eyebrow">"Equations and terms"</p>
                <h2 id="eq-h" class="display">"The Dynamical Synthesis equation"</h2>
                <p class="lede-sm">"I = W(I) + you expresses the central idea: the current state combines the system's transformation of its prior state with outside input. The remaining equations describe memory, prediction and action. The update rate η controls changes to the predictor; ⊗ represents the paired quantities used in an update. The table gives the current implementation in Fish."</p>
            </div>
            <pre class="exegesis mono">{EXEGESIS}</pre>
            <div class="table-wrap">
                <table class="table organs">
                    <thead><tr><th>"Symbol"</th><th>"Pronunciation"</th><th>"Meaning"</th></tr></thead>
                    <tbody>
                        {ORGANS.iter().map(|o| view! {
                            <tr><td class="serif sym">{o.symbol}</td><td class="muted">{o.said}</td><td>{o.what}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
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
                    <p>"A low-rank adapter, a small set of trainable model parameters, learns to predict the trusted participant's next words from stored exchanges. Training loss covers only those words. Every fifth exchange is reserved for evaluation, and an adapter is rejected if its held-out loss does not decrease. This adapter serves the predictor only."</p>
                </div>
                <div class="card">
                    <span class="n">"speak"</span>
                    <h3>"Reply adapter"</h3>
                    <p>"A separate adapter trains on Fish's independently verified replies. Training prompts are generated from the fixed task specification, excluding raw correction messages. The adapter is saved for testing. Deployment requires a separate preregistered experiment, and the current service does not enable it."</p>
                </div>
                <div class="card">
                    <span class="n">"memory maintenance"</span>
                    <h3>"Decay and review"</h3>
                    <p>"Memory weights decrease each night by the factor λ. Records below the minimum weight are removed. Retrieval offsets this decay, so regularly used records remain available. Memories stored in the first hour after an adapter change are held for review."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="instruments-h">
            <div class="section-head">
                <p class="eyebrow">"Neurosymbolic reasoning"</p>
                <h2 id="instruments-h" class="display">"Propose a formal answer and check it symbolically"</h2>
                <p class="lede-sm">"The current lab combines a neural model that proposes solutions with symbolic software that checks them. This is a neurosymbolic approach. Its domain-specific language (DSL) represents calculations as typed JSON graphs: every operation and argument must follow explicit rules."</p>
                <p class="prose-p">"A deterministic compiler translates each accepted graph into Wolfram expressions. The Wolfram kernel computes the results, which are compared with an exact task generator. Independent AI review checks whether the complete answer follows the requested specification. These are bounded mathematical checks; the lab does not yet generate general formal proofs that an implementation satisfies its specification."</p>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Structured answers."</b>" Fish returns a JSON graph: a sequence of operations using integers and references to earlier results. A grammar limits the allowed output. The strict version fixes result names and argument counts for each task; the loose version fixes only the general structure."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Exact calculation."</b>" A checker validates the graph, then a computer algebra system evaluates it against the task generator's answer. Parentheses preserve the meaning of negative constants. The checker's file hash is fixed before the run."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Isolated tests."</b>" Each request starts from a fresh copy of the same saved state and a new session. Memory retrieval, prediction and tools are disabled. The request record confirms the adapter settings actually used."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Blinded conditions."</b>" Adapters receive randomly assigned labels before testing. Results and independent reviews are fixed and hashed before those labels are decoded."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Independent AI review."</b>" A separate AI reviewer, unaware of the experimental condition, reads each complete reply that passed the checker. The reviewer can reject an answer whose steps do not match the requested task. This review is an additional check, not a formal proof."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Preset decision criteria."</b>" Each protocol sets statistical tests, limits on losses within task families, and checks for negative constants, information leaks and invalid requests. The statistical methods are the one-sided exact McNemar test and a Bonferroni-adjusted Clopper–Pearson bound on the difference in success rates. The Results page explains and recalculates them."</p></div>
            </div>
        </section>

        <section id="laws" class="wrap section" aria-labelledby="laws-h">
            <div class="section-head">
                <p class="eyebrow">"Observed patterns"</p>
                <h2 id="laws-h" class="display">"What the experiments suggest"</h2>
                <p class="lede-sm">"The project records these observations in its ‘laws’ file, with references to the underlying exchanges. They describe the tested conditions and remain open to revision. The source labels below identify recorded batches, database rows and interaction cycles."</p>
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
