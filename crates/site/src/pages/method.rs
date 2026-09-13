//! Equations, implementation and evaluation methods.
use crate::components::page_nav::PageNav;
use crate::components::research_figure::ResearchFigure;
use crate::components::tables::ScrollTable;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::laws::LAWS;

const SECTIONS: &[(&str, &str)] = &[
    ("#boundary-h", "System boundary"),
    ("#eq-h", "State and prediction"),
    ("#symbols", "Symbol guide"),
    ("#night-h", "Training and memory"),
    ("#instruments-h", "Verification"),
    ("#context-h", "Scientific context"),
    ("#laws", "Observations"),
];

const STATE_UPDATE: &str = "I[t] = α · E(r[t−1]) + E(you[t])";
const PREDICTION_SCORES: &str = "δ₁ = 1 − cos(E(Î), E(you))
δ₂ = ½ · (1 − exp(−L)) + ½ · δ₁";

struct Symbol {
    symbol: &'static str,
    said: &'static str,
    what: &'static str,
}

const SYMBOLS: &[Symbol] = &[
    Symbol { symbol: "I, W", said: "I, W", what: "In the shorthand, I denotes system activity and W its internal processing. In the text implementation, I[t] denotes the retrieval vector, which is only one part of the system state." },
    Symbol { symbol: "you, u[t]", said: "you, u at t", what: "External input. The general equations use u[t]; the text implementation uses you[t] for the incoming message." },
    Symbol { symbol: "s[t], a[t]", said: "s at t, a at t", what: "The complete state and action at step t. The state contains the information needed by the specified update, including memory and internal variables where applicable." },
    Symbol { symbol: "F, π, θ[t]", said: "F, pi, theta at t", what: "F updates the state, π selects an action and θ[t] denotes the learned parameters at step t." },
    Symbol { symbol: "E, d, r, α", said: "E, d, r, alpha", what: "E converts text to a vector of dimension d. r[t−1] is the previous reply, whose vector is weighted by α in retrieval. The reference setting for α is 0.70." },
    Symbol { symbol: "Ŵ → Î", said: "W-hat produces I-hat", what: "The predictor Ŵ produces Î, the expected next incoming message." },
    Symbol { symbol: "δ, L", said: "delta, L", what: "δ denotes prediction error. L is the mean negative log probability of the observed tokens under the predictor’s prior context." },
    Symbol { symbol: "σ, θₘ", said: "sigma, theta subscript m", what: "σ selects exchanges for active memory when their error score exceeds θₘ, subject to source and review rules. The threshold has a reference value of 0.35 and can be adjusted within configured bounds." },
    Symbol { symbol: "M, λ, k", said: "M, lambda, k", what: "M contains exchanges available to active retrieval. Maintenance multiplies their weights by λ, whose default is 0.995. Retrieval requests k memories, with a reference value of 6." },
    Symbol { symbol: "LoRA", said: "low-rank adaptation", what: "Training of small additional matrices, called adapters, while the base model’s weights remain fixed." },
];

#[component]
pub fn Method() -> impl IntoView {
    set_title("Method");
    view! {
        <div class="method-page research-page">
        <section class="page-intro page-intro-method">
            <div class="wrap">
                <p class="eyebrow">"Method"</p>
                <h1 class="display display-xl">"System design and evaluation"</h1>
                <p class="lede">"The experiments compare model configurations on the same tasks to measure changes in prediction, answer accuracy and retention. This page describes how inputs are represented, how experience is selected for memory and training, and how proposed updates are evaluated."</p>
                <p class="prose-p">"The general equations describe the Dynamical Synthesis framework. The text-processing and training procedures describe its current implementation in Fish. Individual settings and outcomes are reported with the "<a href="/results">"experimental results"</a>"."</p>
            </div>
        </section>

        <div class="research-layout">
        <PageNav items=SECTIONS show_label=false sidebar=true/>
        <div class="research-sections">
        <section class="wrap section" aria-labelledby="boundary-h">
            <div class="section-head">
                <p class="eyebrow">"General formulation"</p>
                <h2 id="boundary-h" class="display">"System boundary and state"</h2>
                <p class="lede-sm">"Dynamical Synthesis describes system activity as a combination of internal processing and external input, written in shorthand as "<strong class="framework-equation">"I = W(I) + you"</strong>". To make this idea testable, an implementation must define the system boundary and the rules governing its state, actions and parameter changes."</p>
            </div>
            <p class="prose-p">"The boundary may enclose one model, an agent with memory and tools, or several interacting systems. External input can then include measurements, control signals or messages. A general formulation is:"</p>
            <pre class="exegesis mono" aria-label="General state and action equations">"s[t+1] = F(s[t], u[t]; θ[t])\na[t]   = π(s[t], u[t]; θ[t])"</pre>
            <p class="prose-p">"At step t, F updates the state s[t] and π selects an action a[t], using input u[t] and learned parameters θ[t]. The notation leaves the choice of functions and learning algorithm to the implementation. Incoming information may depend on earlier actions, even when it comes from outside the boundary."</p>
            <ResearchFigure name="framework"
                alt="Two panels show interaction and parameter learning. Inputs and actions cross the system boundary. Current parameters govern state updates and actions. Interaction records supply training examples. A passing candidate replaces the current parameters; a failed candidate leaves them in use."
                caption="Interaction and parameter learning in the proposed design. Selected records supply training examples. A trained copy replaces the current parameters only after passing reserved tests, including tests of earlier abilities. The reported studies evaluate individual parts of this design."/>
            <h3 class="figure-h">"Feedback sources"</h3>
            <p class="prose-p">"An external message may repeat the system’s own answer without adding a measurement or calculation that could confirm it. Proposed tests would trace the source and any changes to the feedback, then examine whether new observations or checked calculations lead the system to correct an error. Reusing verified examples for retention serves a different purpose from treating repeated claims as new evidence."</p>
            <h3 class="figure-h">"Authority and intervention"</h3>
            <p class="prose-p">"Input can provide information without granting permission to change a goal or an operating limit. A system must separately define who can authorise those changes. Tests of oversight would assess whether authorised corrections and interventions change behaviour as intended."</p>
            <p class="prose-p">"Feedback and oversight tests remain "<a href="/goals#next-h">"research goals"</a>". The "<a href="/papers/dynamical-synthesis.html" rel="external">"paper"</a>" sets out the framework’s assumptions."</p>
        </section>
        <section class="wrap section" aria-labelledby="eq-h">
            <div class="section-head">
                <p class="eyebrow">"Implemented measurements"</p>
                <h2 id="eq-h" class="display">"Text representation and prediction"</h2>
                <p class="lede-sm">"Fish uses numeric representations of text, called embeddings, to retrieve stored experience. At turn t, the retrieval vector I[t] combines the incoming message, you[t], with the actual previous reply, r[t−1]. The encoder E maps each text to a vector of dimension d, and α weights the previous reply."</p>
            </div>
            <pre class="exegesis mono" aria-label="Implemented retrieval-state equation">{STATE_UPDATE}</pre>
            <p class="prose-p">"The sum is recomputed and normalised before each memory search. The previous reply’s embedding is saved across restarts; when none is available, its contribution is zero. The retrieval vector is one part of the state, alongside stored history, prompt context and model parameters. Its dimension depends on the chosen encoder."</p>
            <h3 class="figure-h">"Prediction-error scores"</h3>
            <p class="prose-p">"The predictor records its expected next message, Î, before the actual message arrives. The first score, δ₁, measures the cosine distance between their embeddings. The second, δ₂, also uses L: the average negative log probability of the observed tokens under the context used for prediction."</p>
            <pre class="exegesis mono" aria-label="Two versions of the prediction-error score">{PREDICTION_SCORES}</pre>
            <p class="prose-p">"Lower values indicate a closer prediction under the chosen measure. Each record identifies the score version, including fallback to δ₁ when token scoring is unavailable, so the versions can be analysed separately. Answer correctness is evaluated with task-specific checks."</p>
        </section>

        <section class="wrap section" aria-labelledby="symbols">
            <div class="section-head">
                <h2 id="symbols" class="display">"Symbols and reference settings"</h2>
                <p class="prose-p">"This guide covers the equations and the symbols beside the home-page learning loop. The memory threshold is written θₘ to distinguish it from learned parameters θ[t]; source records abbreviate the threshold as θ. Reference settings below may differ from those used in a particular experiment."</p>
            </div>
            <ScrollTable label="Symbols and measurement definitions">
                <table class="table organs">
                    <caption class="sr-only">"Notation for the general formulation and the implemented learning cycle"</caption>
                    <thead><tr><th scope="col">"Symbol"</th><th scope="col">"Read as"</th><th scope="col">"Meaning"</th></tr></thead>
                    <tbody>
                        {SYMBOLS.iter().map(|s| view! {
                            <tr><th scope="row" class="serif sym">{s.symbol}</th><td class="muted">{s.said}</td><td>{s.what}</td></tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </ScrollTable>
            <details class="study-details">
                <summary>"Additional recorded quantities and implementation notes"</summary>
                <div class="study-details-content prose">
                    <p>"H(you) measures variation in one speaker’s recent messages using the mean pairwise embedding distance. It uses up to twelve messages and requires at least four; it is not Shannon entropy. The recorded score V = −|δ| + β·H(you) combines prediction error and input variety, with β ranging from a configured floor to twice that floor. V is not a training objective."</p>
                    <p>"ε denotes the configured fraction of messages supplied by the additional external-input channel. Source and review rules determine whether those messages may enter active memory or training. The term ‘normalised perplexity’ in source records refers to 1 − exp(−L), the bounded transform of token loss used in δ₂."</p>
                    <p>"The "<a href="/results#prediction-settings">"interaction results"</a>" record the encoder and dimension used. Retrieval adds a fixed search instruction before embedding each text; prediction scores use the messages without that prefix."</p>
                    <p>"Setting α below one does not establish stability of the complete system. That claim would require a defined state space, a distance measure and a bound on how the full update changes distances between states."</p>
                    <p class="source mono">"Source files: fish/daemon/src/fast/state.rs · fish/daemon/src/ledger/embed/mod.rs · fish/daemon/src/skin/mod.rs · fish/daemon/src/want/mod.rs · SOUL.md"</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="night-h">
            <div class="section-head">
                <p class="eyebrow">"Learning procedures"</p>
                <h2 id="night-h" class="display">"Training and memory"</h2>
                <p class="lede-sm">"Prediction and answer generation use separate LoRA adapters with different training targets. Their base model stays fixed, and each adapter is evaluated against the task it was trained to perform. Stored experience is maintained separately for retrieval and possible reuse in training."</p>
            </div>
            <h3 class="figure-h">"Prediction training"</h3>
            <p class="prose-p">"Each eligible exchange is paired with the next incoming message in the same session. Only that target message contributes to the training loss. Evaluation uses examples excluded from training, including older examples when assessing retention."</p>
            <h3 class="figure-h">"Response training"</h3>
            <p class="prose-p">"The mathematics studies trained on checked answers, while the continuation study used worked solutions from a separate teaching procedure. Later mathematics studies generated prompts directly from problem specifications after earlier prompts containing corrections were linked to repeated errors. Evaluation checks answers to new questions and tasks on which the system previously succeeded."</p>
            <h3 class="figure-h">"Memory maintenance"</h3>
            <p class="prose-p">"Nightly maintenance reduces memory weights, removing records below the configured minimum from active retrieval while preserving the original exchanges in the archive. Retrieval can increase a record’s weight. Memories selected during the first hour after an adapter change are withheld from retrieval and training until reviewed."</p>
            <h3 class="figure-h">"Update evaluation"</h3>
            <p class="prose-p">"A trained adapter remains a candidate until it passes the required tests. The continuation protocol permits activation only on the isolated experimental copy, with the previous configuration retained for recovery. Its "<a href="/results#current">"results"</a>" report the training losses, answer comparisons and adoption decisions."</p>
            <details class="study-details">
                <summary>"Data selection, validation and source files"</summary>
                <div class="study-details-content prose">
                    <p>"The data builder applies source, session and review rules, excluding records that await review. For each identified speaker, it normally reserves every fifth eligible pair for validation. Groups of two to four pairs reserve the last pair; a separate coverage check flags speakers without a validation example."</p>
                    <p>"Continuation training reuses eligible earlier examples alongside new ones, while keeping earlier validation examples reserved. Additional test examples assess prediction of new inputs. Response training uses a separate data split."</p>
                    <p class="source mono">"Source files: fish/night/lean.py · fish/night/mercy.py · fish/lab/private_dream.py · fish/lab/private_dream_data.py · docs/KEEPING.md"</p>
                    <p>"Original records call offline training a ‘dream’ and an adapter a ‘coat’; ‘wearing’ means activating that adapter. These names describe software operations."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="instruments-h">
            <div class="section-head">
                <p class="eyebrow">"Experimental verification"</p>
                <h2 id="instruments-h" class="display">"Mathematical checks and answer review"</h2>
                <p class="lede-sm">"A correct numerical result can still answer the wrong question. The main mathematics comparisons therefore combine checks of answer structure, exact calculations and review of the complete solution. Each stage can reject an answer before it counts as a success."</p>
                <p class="prose-p">"The controls use the base model without additional training and an adapter trained on the same questions with answers taken from other questions. They help assess the contribution of correct training examples. Separate comparisons vary prompts and output rules to examine their effects."</p>
            </div>
            <ol class="rules research-procedure">
                <li class="rule"><span class="n" aria-hidden="true">"01"</span><p><b>"Protocol and version records. "</b>"Before evaluation, local protocol files specify tasks, model configurations and pass criteria. Hashes identify the software, data and adapter versions. These plans were recorded locally without public preregistration."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"02"</span><p><b>"Test conditions. "</b>"Each request starts from a fresh copy of the same saved state and a new session, with memory retrieval, prediction and model tool use disabled. Request records identify the active adapter; the external checker runs separately."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"03"</span><p><b>"Answer structure. "</b>"The model returns a JSON graph of operations, integers and references to earlier results. Output rules either constrain task-specific result names and argument counts or only the general structure."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"04"</span><p><b>"Exact calculation. "</b>"A compiler translates valid graphs into Wolfram expressions, preserving negative signs. The Wolfram kernel, called locally through WolframScript, evaluates them for comparison with exact reference answers from the task generator."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"05"</span><p><b>"Review with model identities hidden. "</b>"A separate AI reviewer reads answers that passed calculation checks and can reject incorrect or irrelevant steps. Models receive coded labels, revealed only after results and reviews are saved and hashed. This internal review can itself make errors; independent replication is still needed."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"06"</span><p><b>"Decision criteria. "</b>"Paired gains and losses are evaluated with a one-sided exact McNemar test and a conservative confidence bound. Study-specific checks also cover losses in performance and invalid requests, so a higher score alone may not satisfy the protocol. The "<a href="/results#explorer-h">"statistical methods"</a>" give the calculation and its assumptions."</p></li>
            </ol>
            <p class="prose-p">"The 18-question place-value follow-up used exact checking without separate AI review, while later plain-text studies used their own scoring procedures. The "<a href="/results#coats">"study descriptions"</a>" identify the applicable checks. Their scope is the specified tasks and requirements."</p>
            <details class="study-details">
                <summary>"Checker and analysis references"</summary>
                <div class="study-details-content prose">
                    <p>"The 4 September comparison used WolframScript 1.13.0. Its confidence calculation combines one-sided Clopper–Pearson bounds for gains and losses using a Bonferroni adjustment."</p>
                    <p class="source mono">"Source files: fish/lab/claim_probe.py · fish/lab/typed_ast_probe.py · fish/lab/typed_ast_probe_v2.py"</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="context-h">
            <div class="section-head">
                <p class="eyebrow">"Scientific context"</p>
                <h2 id="context-h" class="display">"Related methods and research"</h2>
                <p class="lede-sm">"The implementation draws on established training and evaluation methods. Research on continual learning and generated data helps frame the questions about retention and feedback."</p>
            </div>
            <div class="prose">
                <p>"Training uses the low-rank updates introduced by "<a href="https://arxiv.org/abs/2106.09685">"Hu and colleagues (2021)"</a>". For retention, "<a href="https://doi.org/10.1073/pnas.1611835114">"Kirkpatrick and colleagues (2017)"</a>" studied limits on changes to parameters important for earlier tasks, known as elastic weight consolidation. The continuation procedure here instead reuses earlier examples and tests retention directly."</p>
                <p><a href="https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/mcnemar.htm">"NIST’s account of McNemar’s test"</a>" describes the assumptions behind the paired comparisons. Questions generated from a shared template may not meet the assumption of independence between pairs, limiting interpretation of the significance tests."</p>
                <p>"Research on generated data shows why the training procedure matters. "<a href="https://www.nature.com/articles/s41586-024-07566-y">"Shumailov and colleagues (2024)"</a>" found degradation across generations trained on generated data, while "<a href="https://arxiv.org/abs/2404.01413">"Gerstgrasser and colleagues (2024)"</a>" avoided collapse in their tested settings by retaining original data as generated data accumulated. These studies concern successive training runs. The proposed feedback tests ask a different question: whether a system treats a copied claim as new evidence. Neither effect has been directly tested here."</p>
            </div>
        </section>

        <section id="laws" class="wrap section" aria-labelledby="laws-h">
            <div class="section-head">
                <p class="eyebrow">"Method development"</p>
                <h2 id="laws-h" class="display">"Observations informing the controls"</h2>
                <p class="lede-sm">"Interaction logs and diagnostic tests helped identify where memory selection, training data and answer checks could fail. Several observations came from small or repeated tests, so they do not establish how often these problems occur."</p>
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
            <p class="prose-p">"The source identifiers locate archive batches, database rows and interaction cycles in docs/LAWS.md. The "<a href="/record#where-h">"evidence-access page"</a>" explains how to obtain the underlying records."</p>
        </section>
        </div>
        </div>
        </div>
    }
}
