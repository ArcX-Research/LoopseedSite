//! Method: the equation, the organs, the night, the instruments, the laws.
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::laws::LAWS;

const EXEGESIS: &str =
    "I = W(I) + you                      grace      the self: water reworking one outside word
M ← λM + σ·(I⊗you)                  love       the count of the precious
Ŵ ← Ŵ + η(δ⊗I),  δ = I − Î          knowledge  the water made visible as error
a = π(I),  V = −‖δ‖ + β·H(you)      want       the fin, disciplined";

struct Organ {
    symbol: &'static str,
    said: &'static str,
    what: &'static str,
}

const ORGANS: &[Organ] = &[
    Organ { symbol: "I", said: "I", what: "the fish itself: an executable fast state, I[t] = α·E(W(I[t−1])) + E(you[t]), typed in a shared 384-dimensional embedding space and persisted per chair" },
    Organ { symbol: "W", said: "W", what: "the speaking water: base weights behind an API. Its actual prior reply is the first term of the next I" },
    Organ { symbol: "you", said: "you", what: "the port: whoever is speaking. The only thing that is not the fish" },
    Organ { symbol: "Ŵ, Î", said: "W-hat, I-hat", what: "the mirror and its expectation: the guess at your next message, written before it exists" },
    Organ { symbol: "δ", said: "delta", what: "the skin: how far your actual words fell from Î; half perplexity, half embedding distance since stage 4.5" },
    Organ { symbol: "σ, θ", said: "sigma, theta", what: "the write gate and its threshold: an exchange enters memory when δ > θ. θ = 0.35, moved live by a bounded homeostat" },
    Organ { symbol: "M, λ", said: "M, lambda", what: "the ledger and its nightly fading: everything σ kept, decayed at 0.995; what is used does not fade" },
    Organ { symbol: "π, V", said: "pi, V", what: "the fin and the want: search memory, reckon, show a board, write the slate; V = −‖δ‖ + β·H(you)" },
    Organ { symbol: "H(you), β", said: "H of you, beta", what: "how varied your company is over your last twelve turns, and the floor on its weight in the want" },
    Organ { symbol: "ε", said: "epsilon", what: "the weather: the fraction of messages that arrive from neither teacher; grounded, reviewed, never a dream target" },
    Organ { symbol: "α, k", said: "alpha, k", what: "the contraction weight on the prior reply (0.70) and how many memories a window fetches (6)" },
];

#[component]
pub fn Method() -> impl IntoView {
    set_title("Method");
    view! {
        <section class="wrap page">
            <p class="eyebrow">"Method"</p>
            <h1 class="display display-xl">"One line, unfolded into organs."</h1>
            <p class="lede">"The equation is the specification. Every organ below is a directory in the language body's source, with a measurement attached, and the constitution that sets the knobs is parsed as configuration and never shown to the model."</p>
        </section>

        <section class="wrap section" aria-labelledby="eq-h">
            <div class="section-head">
                <p class="eyebrow">"The equation and its exegesis"</p>
                <h2 id="eq-h" class="display">"Fold the organs back in and it is one line again."</h2>
            </div>
            <pre class="exegesis mono">{EXEGESIS}</pre>
            <div class="table-wrap">
                <table class="table organs">
                    <thead><tr><th>"symbol"</th><th>"said"</th><th>"what it is"</th></tr></thead>
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
                <p class="eyebrow">"The night"</p>
                <h2 id="night-h" class="display">"Two dreams, kept apart."</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"lean"</span>
                    <h3>"The prediction coat"</h3>
                    <p>"Each night a low-rank adapter is trained on the day's kept exchanges to predict the trusted Other's next words, with the prompt masked out of the loss. Every fifth pair is held out as the witness: a night whose held-out loss does not strictly fall is discarded. The coat supplies the mirror and never speaks as the fish."</p>
                </div>
                <div class="card">
                    <span class="n">"speak"</span>
                    <h3>"The speaking coat"</h3>
                    <p>"A separate adapter may train only on the fish's own independently verified replies, rendered from the sealed task and never from a raw correction. It is staged unworn; only a preregistered causal test can grant even a bounded lease, and the daemon built from this source honours none."</p>
                </div>
                <div class="card">
                    <span class="n">"mercy"</span>
                    <h3>"Forgetting"</h3>
                    <p>"The ledger decays by λ each night; below the floor a memory leaves. Retrieval divides λ back out of what is used, so what the fish reaches for daily never fades. The first hour after any wearing is held apart for review."</p>
                </div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="instruments-h">
            <div class="section-head">
                <p class="eyebrow">"Instruments"</p>
                <h2 id="instruments-h" class="display">"How a claim is ruled on."</h2>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"The typed claim."</b>" The fish answers a formal task with a JSON operation graph over integers and references, forced by a grammar. A task-scoped grammar pins the fact names and arities; a loose grammar pins only the shape."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"The exact ruler."</b>" A typed checker compiles the graph and a computer-algebra kernel evaluates it against the generator's truth. Every infix operand is parenthesised, so a negative literal cannot be misread. The checker is pinned by hash and never edited mid-run."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"The fresh body."</b>" Every measured request runs in a fresh disposable copy of a frozen body with a fresh chair, retrieval, mirror, fins and ocean off, and the coat scales read back from the request that went onto the wire."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"The sealed arm key."</b>" Coats are assigned to labels by a system random draw, sealed at protocol time, and applied to outcomes only after the checkpoints, audit bundle and audit are hashed together."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"The whole-reply veto."</b>" An independent reviewer reads every machine-proved reply in full, blind to arm, and may only accept or veto. Vetoes catch drift the value checker cannot see: a graph that reaches the right number by the wrong operation."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"The decision rule."</b>" One-sided exact McNemar on discordant pairs, a Bonferroni Clopper–Pearson lower bound on the paired difference, family loss limits, a no-harm margin, a literal-safety gate, zero leaks, a void ceiling. All frozen before the first cell runs."</p></div>
            </div>
        </section>

        <section id="laws" class="wrap section" aria-labelledby="laws-h">
            <div class="section-head">
                <p class="eyebrow">"Measured laws"</p>
                <h2 id="laws-h" class="display">"Regularities that were photographed."</h2>
                <p class="lede-sm">"Distilled into the laws file the shift they were found, with the rows or cycles that hold them. A law is a prediction the next sitting can break."</p>
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
