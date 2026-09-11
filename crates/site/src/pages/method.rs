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
    Symbol { symbol: "I, W", said: "I, W", what: "In the design shorthand, I denotes system activity and W its internal processing. In the implemented recurrence, I[t] has the narrower meaning of a text vector used for memory retrieval; the previous reply supplies the contribution from internal processing. It is only one part of the system state." },
    Symbol { symbol: "you, u[t]", said: "you, u at t", what: "Input from outside the chosen system boundary. The general equations represent it as u[t]. It may come from an environment, a person or another system. In the text implementation, you[t] is the incoming message." },
    Symbol { symbol: "s[t], a[t]", said: "s at t, a at t", what: "The complete state and the action at step t in the general formulation. The state includes whatever information the specified update needs, such as memory, internal variables and the current model configuration." },
    Symbol { symbol: "F, π, θ[t]", said: "F, pi, theta at t", what: "F updates the state; π selects an action; θ[t] denotes the learned parameters used at step t. These functions and parameters must be specified for a particular implementation. The notation does not prescribe a learning algorithm." },
    Symbol { symbol: "E, d, r, α", said: "E, d, r, alpha", what: "E converts text to an embedding, a numeric vector. d is the vector's dimension, determined by the chosen encoder. r[t−1] is the actual previous reply. α weights that reply's embedding in the retrieval vector; its reference setting is 0.70." },
    Symbol { symbol: "Ŵ → Î", said: "W-hat produces I-hat", what: "Ŵ is the predictor; Î is its predicted next incoming message. The arrow means ‘produces’. This prediction is recorded before the actual message arrives." },
    Symbol { symbol: "δ, L", said: "delta, L", what: "δ is the prediction-error score, with the two versions defined above. L is the mean negative log probability of the observed message's tokens under the predictor's prior context. Lower L means those tokens were assigned higher probability." },
    Symbol { symbol: "σ, θₘ", said: "sigma, theta subscript m", what: "σ is the memory selection rule. An error score above the threshold θₘ can select an exchange for active memory, subject to source and review rules. Records abbreviate this threshold as θ. Its reference value is 0.35, with bounded adjustment by the regulator." },
    Symbol { symbol: "M, λ, k", said: "M, lambda, k", what: "M is the subset of stored exchanges available to active retrieval. λ multiplies their weights during maintenance; its default is 0.995. k sets how many memories retrieval requests; its reference value is 6. Individual experiments record their own settings." },
    Symbol { symbol: "LoRA", said: "low-rank adaptation", what: "A method that trains small matrices added to a model while keeping its base weights fixed. These learned updates are called adapters here. Prediction and answer generation use separate adapters." },
];

#[component]
pub fn Method() -> impl IntoView {
    set_title("Method");
    view! {
        <section class="page-intro page-intro-method">
            <div class="wrap">
                <p class="eyebrow">"Method"</p>
                <h1 class="display display-xl">"How learning is implemented and tested"</h1>
                <p class="lede">"The controlled studies compare model configurations on the same tasks and measure changes in prediction, answer accuracy and retention of earlier abilities. Each comparison specifies what changes in the system, what stays fixed and which checks determine the outcome."</p>
                <p class="prose-p">"The general formulation below describes a proposed framework. The state update, memory rules and training procedures describe the current implementation. "<a href="/results">"The Results page"</a>" gives the settings, findings and limitations of each study."</p>
            </div>
        </section>

        <PageNav items=SECTIONS/>
        <section class="wrap section" aria-labelledby="boundary-h">
            <div class="section-head">
                <p class="eyebrow">"General formulation"</p>
                <h2 id="boundary-h" class="display">"Define the system, its inputs and its actions"</h2>
                <p class="lede-sm">"The shorthand I = W(I) + you expresses the organising idea of Dynamical Synthesis: system activity combines internal processing with external input. A computational model must specify what the system contains, how input is represented and how the state changes over time."</p>
            </div>
            <p class="prose-p">"The system boundary may enclose a single model, an agent with memory and tools, or several interacting systems. Input from outside that boundary may be a sensor measurement, a control signal or a message. One possible formulation is:"</p>
            <pre class="exegesis mono" aria-label="General state and action equations">"s[t+1] = F(s[t], u[t]; θ[t])\na[t]   = π(s[t], u[t]; θ[t])"</pre>
            <p class="prose-p">"At step t, s[t] is the system state, u[t] is the external input and a[t] is an action. F updates the state and π selects the action, using learned parameters θ[t]. A complete implementation must define these functions and any rule for changing the parameters. Input may depend on earlier actions, so an external source is not necessarily statistically independent of the system."</p>
            <ResearchFigure name="framework"
                alt="Two panels show interaction and parameter learning. Inputs and actions cross the system boundary. Current parameters govern state updates and actions. Interaction records supply training examples. A passing candidate replaces the current parameters; a failed candidate leaves them in use."
                caption="Interaction and parameter learning. Records supply selected training examples, and a trained copy is compared with the current version on reserved tests, including earlier skills. A candidate replaces the current parameters only if it meets the specified criteria. This is the proposed design; the reported studies test individual parts."/>
            <div class="cols-2">
                <div class="card"><h3>"Trace feedback to its evidence"</h3><p>"A system may receive an answer that another system copied from its own earlier output. The new sender has added no measurement or calculation that could confirm the answer. Proposed feedback tests would record the original source, any transformations and any missing source information, then test whether new observations or checked calculations lead to correction."</p></div>
                <div class="card"><h3>"Specify who can authorise a change"</h3><p>"Observations supply information; permission to change a goal or an operating limit requires a separately defined authority. Tests of human oversight would need to check whether authorised corrections and interventions change behaviour as intended. The general equations do not establish that ability."</p></div>
            </div>
            <p class="prose-p">"Reusing verified training examples may support retention without adding new evidence. The proposed feedback tests ask whether a system treats a repeated claim as new support. These tests, and tests of human oversight, remain "<a href="/goals#next-h">"research goals"</a>". The "<a href="/record#paper-h">"paper"</a>" sets out the assumptions."</p>
        </section>
        <section class="wrap section" aria-labelledby="eq-h">
            <div class="section-head">
                <p class="eyebrow">"Implemented measurements"</p>
                <h2 id="eq-h" class="display">"Represent input and measure prediction error"</h2>
                <p class="lede-sm">"Fish represents text as numeric vectors called embeddings. At turn t, its retrieval vector I[t] combines the new message, you[t], with the actual previous reply, r[t−1]. E maps each text to a d-dimensional vector, with d determined by the chosen encoder. α sets the weight of the previous reply."</p>
            </div>
            <pre class="exegesis mono" aria-label="Implemented retrieval-state equation">{STATE_UPDATE}</pre>
            <p class="prose-p">"The resulting vector is normalised before memory search. The previous reply's embedding is saved across session restarts; when none is available, its contribution is zero. The sum is recomputed for each input. This vector represents one part of the system state, alongside the stored history, prompt context and model parameters."</p>
            <h3 class="figure-h">"Two versions of the prediction score"</h3>
            <p class="prose-p">"The predictor records its expected next message, Î, before the actual message arrives. The first score, δ₁, measures the cosine distance between their embeddings. The second, δ₂, also uses L: the average negative log probability of the observed tokens under the context used for prediction."</p>
            <pre class="exegesis mono" aria-label="Two versions of the prediction-error score">{PREDICTION_SCORES}</pre>
            <p class="prose-p">"Lower values indicate closer predictions under the stated measure. The record identifies the version used, including fallback to δ₁ when token scoring is unavailable. The versions must be analysed separately. Neither score measures whether the system's answer is correct."</p>
            <h3 id="symbols" class="figure-h">"Symbols and abbreviations"</h3>
            <p class="prose-p">"The home-page learning cycle uses the symbols defined here. The memory threshold is written θₘ below to distinguish it from the learned parameters θ[t] in the general equations; implementation records abbreviate the threshold as θ."</p>
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
                    <p>"H(you) measures the variety of one speaker's recent messages: the mean pairwise embedding distance among up to twelve messages, with at least four required. It is a measure of dispersion, rather than Shannon entropy. The weight β varies between a configured floor and twice that floor. V = −|δ| + β·H(you) combines prediction error and input variety in a recorded score; the model is not trained to maximise V."</p>
                    <p>"ε denotes the configured fraction of messages supplied by the additional external-input channel. Source and review rules determine whether those messages may enter active memory or training. The term ‘normalised perplexity’ in source records refers to 1 − exp(−L), the bounded transform of token loss used in δ₂."</p>
                    <p>"The encoder and dimension used for the interaction measurements are recorded with the "<a href="/results#prediction-settings">"results"</a>". The retrieval calculation adds a fixed search instruction before embedding each text; the prediction score embeds the messages without that prefix. A coefficient α below one in the retrieval equation does not establish stability of the complete system. A contraction claim would require a defined state space, a distance measure and a bound on how the full update changes distances between states."</p>
                    <p class="source mono">"Source files: fish/daemon/src/fast/state.rs · fish/daemon/src/ledger/embed/mod.rs · fish/daemon/src/skin/mod.rs · fish/daemon/src/want/mod.rs · SOUL.md"</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="night-h">
            <div class="section-head">
                <p class="eyebrow">"Training and memory"</p>
                <h2 id="night-h" class="display">"Separate adapters for prediction and replies"</h2>
                <p class="lede-sm">"Training uses low-rank adaptation (LoRA): small learned matrices modify the model while its base weights remain fixed. One adapter is trained to predict the next input and another to generate answers. They have different training targets and are evaluated separately."</p>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Predict the next input"</h3>
                    <p>"Training examples pair an eligible exchange with the next incoming message in the same session. Only the target message contributes to the training loss. Evaluation measures token loss on examples reserved from training, including earlier examples when testing retention."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Generate answers"</h3>
                    <p>"The mathematical studies trained on checked model answers. Some initial prompts contained unrelated corrections, so later studies generated prompts directly from the problem specifications. The private continuation study used worked solutions supplied by a separate teaching procedure. Answer tests measure whether the trained adapter solves new questions and preserves earlier abilities."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Maintain stored experience"</h3>
                    <p>"Nightly maintenance reduces memory weights. Records below the configured minimum leave active retrieval, while their original exchange records remain in the archive. Retrieval can increase a record's weight. New memories selected during the first hour after an adapter change are withheld from retrieval and training until reviewed."</p>
                </div>
            </div>
            <p class="prose-p">"A trained adapter is a candidate update until it passes the required evaluation. The private continuation protocol permits activation only on its isolated experimental copy after the preset checks pass, with the previous configuration retained for recovery. "<a href="/results#current">"The recorded comparison"</a>" reports the training losses, answer tests and activation decisions."</p>
            <details class="study-details">
                <summary>"Data selection, validation and source files"</summary>
                <div class="study-details-content prose">
                    <p>"Prediction examples must satisfy the source, session and review rules in the data builder. Records awaiting review are excluded. For each identified speaker, the standard split reserves every fifth eligible pair for validation. A group with two to four pairs reserves its last pair; a speaker with no validation example is flagged by a separate coverage check."</p>
                    <p>"The private continuation procedure reuses eligible earlier training examples alongside new ones. Earlier validation examples remain reserved, and additional examples assess prediction of new inputs. Answer training has its own data split. Lower validation loss therefore needs to be considered alongside direct tests of new questions and earlier abilities."</p>
                    <p class="source mono">"Source files: fish/night/lean.py · fish/night/mercy.py · fish/lab/private_dream.py · fish/lab/private_dream_data.py · docs/KEEPING.md"</p>
                    <p>"Original records call offline training a ‘dream’ and an adapter a ‘coat’; ‘wearing’ means activating that adapter. These names describe software operations."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="instruments-h">
            <div class="section-head">
                <p class="eyebrow">"Experimental verification"</p>
                <h2 id="instruments-h" class="display">"Check the calculation and the complete answer"</h2>
                <p class="lede-sm">"The mathematical adapter studies combine model-generated solutions with exact symbolic calculation. The Wolfram kernel, called through WolframScript, evaluates the submitted calculations against reference answers. The main comparisons also use a separate AI reviewer to assess whether the complete solution answers the question."</p>
                <p class="prose-p">"Controls include the base model without additional training and an adapter trained on shuffled question–answer pairings. Comparing the same questions across configurations helps distinguish the effect of correct training examples from effects of additional training or the output format."</p>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Record the protocol before evaluation."</b>" The main comparisons specify the tasks, model configurations and pass criteria in local protocol files. Hashes identify the versions of software, data and adapters used. These are locally recorded plans, rather than public preregistrations."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Control the test conditions."</b>" Each request starts from a fresh copy of the same saved state and a new session. Memory retrieval, prediction and model tool use are disabled for the adapter comparison. Request records confirm the adapter configuration used; the external checker runs separately."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Validate the answer structure."</b>" Models return a JSON graph: a list of calculation steps containing operations, integers and references to earlier results. A grammar constrains this output. The strict version specifies result names and argument counts for each task; the less restrictive version specifies only the general structure."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Recompute the mathematics."</b>" A compiler translates valid graphs into Wolfram expressions, preserving the signs of negative constants. The Wolfram kernel evaluates those expressions, and the checker compares them with exact answers computed by the task generator. Matching a value alone does not establish that every step follows the requested method."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Review the full solution with model identities hidden."</b>" Adapters receive coded labels. A separate AI reviewer reads answers that passed the checker and can reject incorrect or irrelevant steps. Results and reviews are saved and hashed before the labels are revealed. This review is part of the project and remains fallible; independent replication is still needed."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Apply the stated decision criteria."</b>" The main comparisons assess paired gains and losses: the same question succeeds with one configuration and fails with the other. They use a one-sided exact McNemar test and a conservative confidence bound, alongside protocol-specific checks for regressions and invalid requests. A higher total score alone may not satisfy those criteria. "<a href="/results#explorer-h">"Statistical calculations and assumptions"</a>"."</p></div>
            </div>
            <p class="prose-p">"The 18-question place-value follow-up used exact checking without the separate AI review. Later studies of plain-text answers used their own scoring procedures. The "<a href="/results#coats">"study descriptions"</a>" identify which checks apply. These procedures assess specified mathematical tasks; they do not constitute a general proof that the software meets every requirement."</p>
            <details class="study-details">
                <summary>"Checker and analysis references"</summary>
                <div class="study-details-content prose">
                    <p>"The 4 September comparison records WolframScript version 1.13.0. Its confidence calculation combines one-sided Clopper–Pearson bounds for paired gains and losses using a Bonferroni adjustment. The Results page reproduces the calculation and discusses the assumption that task pairs are independent."</p>
                    <p class="source mono">"Source files: fish/lab/claim_probe.py · fish/lab/typed_ast_probe.py · fish/lab/typed_ast_probe_v2.py"</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="context-h">
            <div class="section-head">
                <p class="eyebrow">"Scientific context"</p>
                <h2 id="context-h" class="display">"Methods and related research"</h2>
                <p class="lede-sm">"The training procedure draws on low-rank adaptation. Retention tests address a problem studied in continual learning, and the paired comparisons use established statistical methods. Research on generated training data also informs the proposed feedback tests."</p>
            </div>
            <div class="prose">
                <p><b>"Parameter-efficient training. "</b><a href="https://arxiv.org/abs/2106.09685">"Hu and colleagues, LoRA (2021)"</a>" introduced the low-rank parameter updates used here. The contribution of an adapter in these experiments is assessed through its predictions and answers on the specified tests."</p>
                <p><b>"Retention after further learning. "</b><a href="https://doi.org/10.1073/pnas.1611835114">"Kirkpatrick and colleagues (2017)"</a>" address loss of earlier abilities by limiting changes to parameters important for previous tasks, a method called elastic weight consolidation. The private continuation study uses replay of earlier examples and explicit retention tests; it does not implement elastic weight consolidation."</p>
                <p><b>"Paired outcome comparisons. "</b><a href="https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/mcnemar.htm">"NIST's description of McNemar's test"</a>" gives its assumptions, including independence between pairs. Questions generated from a shared template may be related, which limits interpretation of the reported significance tests."</p>
                <p><b>"Repeated use of generated data. "</b><a href="https://www.nature.com/articles/s41586-024-07566-y">"Shumailov and colleagues (2024)"</a>" found degradation across generations of models trained on generated data. "<a href="https://arxiv.org/abs/2404.01413">"Gerstgrasser and colleagues (2024)"</a>" found that retaining original data while accumulating generated data avoided collapse in the settings they studied. These results depend on the training regime. They concern changes across successive training runs; treating a copied claim as fresh evidence during interaction is a separate question. Neither phenomenon has been directly tested in the studies reported here."</p>
            </div>
        </section>

        <section id="laws" class="wrap section" aria-labelledby="laws-h">
            <div class="section-head">
                <p class="eyebrow">"Method development"</p>
                <h2 id="laws-h" class="display">"Observations behind the experimental controls"</h2>
                <p class="lede-sm">"Interaction logs and diagnostic tests exposed problems that informed these procedures. The observations below explain why particular controls are needed. Several come from small, repeated tests and do not establish how common the failures are."</p>
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
            <p class="prose-p">"The identifiers refer to archive batches, database rows and interaction cycles recorded in docs/LAWS.md. "<a href="/record#where-h">"Source records and access"</a>" explains how to locate and request the detailed evidence."</p>
        </section>
    }
}
