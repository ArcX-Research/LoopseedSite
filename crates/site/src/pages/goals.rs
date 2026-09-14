//! Research goals, proposed tests and the limits of current evidence.
use crate::components::page_nav::PageNav;
use crate::components::tables::ScrollTable;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::stages::{Stage, BECOMING_CLOSED, ORGANISM, SCALING};

const SECTIONS: &[(&str, &str)] = &[
    ("#next-h", "Next studies"),
    ("#benefit-h", "Human benefit"),
    ("#claim-h", "Learning outcomes"),
    ("#novelty-h", "New ideas"),
    ("#becoming", "Computational universe"),
    ("#not-h", "Evidence and decisions"),
    ("#tracks", "Development record"),
];

#[component]
pub fn Goals() -> impl IntoView {
    set_title("Research goals");
    view! {
        <div class="goals-page research-page">
        <section class="page-intro page-intro-goals">
            <div class="wrap">
                <p class="eyebrow">"Research goals"</p>
                <h1 class="display display-xl">"Next studies and broader goals"</h1>
                <p class="lede">"The next phase will investigate the gaps identified in "<a href="/papers/dynamical-synthesis.html">"Dynamical Synthesis: Learning through Interaction"</a>": applying learning to unfamiliar tasks, retaining earlier abilities and separating the effects of memory and training. These studies will inform the longer-term work on useful new ideas, computational environments and human control."</p>
                <p class="prose-p">"We plan to report the follow-up studies in a second paper and publish later phases as their findings become available, including unsuccessful tests and remaining questions."</p>
            </div>
        </section>

        <div class="research-layout">
        <PageNav items=SECTIONS show_label=false sidebar=true/>
        <div class="research-sections">
        <section class="wrap section" aria-labelledby="next-h">
            <div class="section-head">
                <p class="eyebrow">"Next research phase"</p>
                <h2 id="next-h" class="display">"Follow-up experiments"</h2>
                <p class="lede-sm">"These priorities follow the paper’s findings and limitations. Each study will set its comparisons, measurements and decision criteria before evaluation."</p>
            </div>
            <ol class="rules research-procedure">
                <li class="rule"><span class="n" aria-hidden="true">"01"</span><p><b>"Transfer to unfamiliar tasks. "</b>"The harder mathematics study found no advantage over the control trained with mismatched answers. The next comparison would reserve entire problem types for evaluation, giving each model the same computing budget and access to tools."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"02"</span><p><b>"Retention across updates. "</b>"Further response training produced more losses than gains. Repeated training rounds would compare ways to preserve earlier abilities, including reuse of earlier examples, with new and previously successful tasks tested after each round."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"03"</span><p><b>"Rule learning and reuse. "</b>"The hidden-rule studies found unreliable learning and use of saved descriptions. Follow-up tests would cover the full sequence from observations to a correct rule and its use on new cases after a reset, counting failed attempts in the overall result."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"04"</span><p><b>"Memory and parameter updates. "</b>"The broader learning comparison did not reach a final result. Varying memory and training separately would help establish whether each contributes beyond additional training alone."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"05"</span><p><b>"Feedback and intervention. "</b>"Controlled tests would measure responses to corrections, misleading feedback and authorised intervention before and after learning. They would include cases in which a system receives copies of its own earlier errors."</p></li>
                <li class="rule"><span class="n" aria-hidden="true">"06"</span><p><b>"Replication and comparison. "</b>"Repeating studies across random seeds and model families would test the consistency of the findings. Comparisons with established learning methods and records for independent replication would help assess their wider relevance."</p></li>
            </ol>
            <p class="more"><a href="/results">"Findings behind these priorities"</a></p>
        </section>

        <section class="wrap section" aria-labelledby="benefit-h">
            <div class="section-head">
                <p class="eyebrow">"Human benefit"</p>
                <h2 id="benefit-h" class="display">"Useful learning under human control"</h2>
                <p class="lede-sm">"Learning can change how a system responds to evidence, instructions and intervention. We want to understand whether useful abilities can accumulate while people retain effective ways to inspect, correct and stop the system."</p>
            </div>
            <h3 class="figure-h">"Benefits and effects on people"</h3>
            <p class="prose-p">"A proposed application should address a specific need, such as checking a scientific analysis or finding software defects. Evaluation would compare its usefulness, errors and resource use with an established method, and consider who benefits and who bears the risks. People affected by its use should have a part in those decisions."</p>
            <h3 class="figure-h">"Correction and intervention"</h3>
            <p class="prose-p">"Tests would assess whether authorised people can correct an error, revise a task, restrict actions and stop operation after successive updates. Success rates and response times would be measured under familiar and unfamiliar conditions. An update that fails the required intervention tests would be rejected even if task performance improved."</p>
            <h3 class="figure-h">"Records and independent review"</h3>
            <p class="prose-p">"Preserving the starting model, learning data and updated version would allow reviewers to trace a change in behaviour to the experience used in training. We plan to share protocols, measured outcomes and unsuccessful tests alongside "<a href="/#live-science">"public research sessions"</a>". The "<a href="/record#where-h">"evidence-access page"</a>" identifies downloadable records and material available on request."</p>
            <details class="study-details">
                <summary>"Related work on human control and beneficial AI"</summary>
                <div class="study-details-content prose">
                    <p>"This direction shares concerns with the "<a href="https://futureoflife.org/our-mission/">"Future of Life Institute’s mission"</a>" and the "<a href="https://futureoflife.org/open-letter/ai-principles/">"Asilomar AI Principles"</a>". Our proposed contribution is to test how repeated learning updates affect usefulness, correction and intervention."</p>
                    <p>"Verification and control are also discussed in "<a href="https://futureoflife.org/data/documents/research_priorities.pdf" rel="external">"Research Priorities for Robust and Beneficial Artificial Intelligence"</a>" by Russell, Dewey and Tegmark (2015)."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="claim-h">
            <div class="section-head">
                <p class="eyebrow">"Evaluation"</p>
                <h2 id="claim-h" class="display">"Learning outcomes"</h2>
                <p class="lede-sm">"Prediction, task performance and retention need separate measurements. Improvement in one does not establish improvement in the others."</p>
            </div>
            <p class="prose-p">"Prediction tests would measure error and uncertainty on observations excluded from training, using a fixed scoring procedure. Task tests would assess accuracy on unfamiliar problem types and distinguish using a supplied rule from learning one through observation. Retention tests would revisit earlier abilities after delays, new sessions and further training."</p>
            <p class="prose-p">"Together, these measurements would show which abilities improve, how long the gains last and where performance declines. Each study would define acceptable losses and required improvements before testing."</p>
        </section>

        <section class="wrap section" aria-labelledby="novelty-h">
            <div class="section-head">
                <p class="eyebrow">"New ideas"</p>
                <h2 id="novelty-h" class="display">"Generation and evaluation of new ideas"</h2>
                <p class="lede-sm">"New ideas are not an explicit term in "<strong class="framework-equation">"I = W(I) + you"</strong>". They may emerge as a system combines external observations with what it has already learned. We will look for this possibility in the hypotheses, methods and designs it proposes."</p>
            </div>
            <h3 class="figure-h">"Novelty, correctness and usefulness"</h3>
            <p class="prose-p">"A proposal would first be compared with specified examples, known methods and prior work. Absence from the supplied material would not establish originality, since the idea might appear in earlier training or existing literature. Scientific discovery would require an independent assessment of both novelty and correctness."</p>
            <p class="prose-p">"Evaluation would then test the proposal: checking predictions against unseen observations, running programs against a specification or seeking counterexamples to a rule. Usefulness would be measured against a stated task and comparison method, with duplicate, incorrect and unsuccessful proposals included in the record."</p>
            <h3 class="figure-h">"Contribution of external observations"</h3>
            <p class="prose-p">"Comparisons would keep the task, starting model and computing budget fixed while changing the observations available to the system. Withheld or shuffled observations would help test whether informative input contributes to useful proposals and whether those proposals can be retained and used later."</p>
            <p class="prose-p">"The existing experiments use tasks with known reference answers. They provide no evidence yet of open-ended scientific discovery."</p>
        </section>

        <section id="becoming" class="wrap section" aria-labelledby="becoming-h">
            <div class="section-head">
                <p class="eyebrow">"Computation beyond language"</p>
                <h2 id="becoming-h" class="display">"Learning in the computational universe"</h2>
                <p class="lede-sm">"The computational universe is the space of possible programs, rules and the behaviours they generate, following "<a href="https://wolframphysics.org/glossary/">"Wolfram’s terminology"</a>". It offers environments whose rules can be specified and whose outcomes can be checked, making them useful for controlled studies of learning."</p>
                <p class="prose-p">"Simple programs can produce complex patterns, as illustrated in "<a href="https://writings.stephenwolfram.com/2019/05/a-world-run-with-code/">"Wolfram’s discussion of the computational universe"</a>". We want to test whether a learner can model such systems, choose informative experiments and adapt when their rules change."</p>
            </div>
            <h3 class="figure-h">"Programs, rules and simulations"</h3>
            <p class="prose-p">"Short programs, symbolic transformations and graph rules would support tests of rule learning, prediction and program construction. Cellular automata and other simulations would allow us to vary initial conditions, available observations and opportunities for action under fixed resource limits."</p>
            <p class="prose-p">"Each experiment would use a finite set of rules known to the evaluator and hidden from the learner when discovery is the task. Several rules may fit the same observations, so evaluation would include new predictions and experiments that distinguish competing explanations. Direct simulation, search and models without a learning update would provide comparisons for accuracy and computing cost."</p>
            <h3 class="figure-h">"Interacting systems"</h3>
            <p class="prose-p">"Agents that observe and influence one another would allow us to trace how information and errors spread. With each agent’s boundary and permitted actions defined, tests could examine whether checked knowledge transfers between systems and remains useful after further learning."</p>
            <p class="prose-p">"The existing "<a href="/results#worlds">"rule-learning studies"</a>" provide a limited starting point for this work."</p>
            <details class="study-details">
                <summary>"Connections to physical theories"</summary>
                <div class="study-details-content prose">
                    <p>"The "<a href="https://www.wolframphysics.org/technical-introduction/">"Wolfram Physics Project"</a>" explores whether physical laws can arise from underlying computational rules. Testing that idea requires predictions about nature, physical observations and comparison with existing theories. Our software experiments do not establish a physical theory."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="not-h">
            <div class="section-head">
                <p class="eyebrow">"Evidence and decisions"</p>
                <h2 id="not-h" class="display">"Interpretation and research decisions"</h2>
                <p class="lede-sm">"These goals extend beyond the present evidence. The reported results concern particular tasks and configurations; they do not establish reliable learning across environments, human control through repeated updates or consciousness."</p>
            </div>
            <p class="prose-p">"Independent replication remains necessary. The "<a href="/results">"Results page"</a>" gives the measured outcomes and limitations, while the "<a href="/method">"Method page"</a>" describes how answers and proposed updates are checked."</p>
            <h3 id="falsify-h" class="figure-h">"Criteria for revising the approach"</h3>
            <p class="prose-p">"The learning procedure would need revision if gains disappear under matched controls, fail consistently on unfamiliar tasks or are outweighed by losses of earlier abilities. If informative observations produce no advantage over the comparison conditions, the proposed contribution of interaction would remain unsupported. Failure to respect authorised intervention would count against suitability for use."</p>
        </section>

        <section id="tracks" class="wrap section" aria-labelledby="tracks-h">
            <div class="section-head">
                <p class="eyebrow">"Development record"</p>
                <h2 id="tracks-h" class="display">"Recorded stages and outstanding checks"</h2>
                <p class="lede-sm">{BECOMING_CLOSED}</p>
            </div>
            <p class="prose-p">"The tables retain the project’s stage identifiers and decisions. Most entries reflect the 7 September review; the paper entry includes the published paper and data. A stage marked ‘closed’ may have been closed by a supervisor’s waiver without meeting every original criterion."</p>
            <details class="study-details">
                <summary>"System components and their recorded checks"</summary>
                <div class="study-details-content">
                    <StageTable stages=ORGANISM/>
                </div>
            </details>
            <details class="study-details">
                <summary>"Evaluation, replication and reporting"</summary>
                <div class="study-details-content">
                    <StageTable stages=SCALING/>
                </div>
            </details>
            <p class="prose-p">"The "<a href="/method#symbols">"symbol guide"</a>" defines the notation used in these records. The "<a href="/record#reports-h">"report list"</a>" provides study outcomes and sources."</p>
        </section>
        </div>
        </div>
        </div>
    }
}

#[component]
fn StageTable(stages: &'static [Stage]) -> impl IntoView {
    view! {
        <ScrollTable label="Research roadmap and recorded stage status">
            <table class="table stages">
                <caption class="sr-only">"Project stages, criteria and recorded outcomes"</caption>
                <thead><tr><th scope="col">"Stage"</th><th scope="col">"Component or study"</th><th scope="col">"Purpose"</th><th scope="col">"Recorded criterion"</th><th scope="col">"Status and qualification"</th></tr></thead>
                <tbody>
                    {stages.iter().map(|s| view! {
                        <tr>
                            <th scope="row" class="mono muted">{s.id}</th>
                            <td><b>{s.name}</b></td>
                            <td>{s.what}</td>
                            <td class="muted">{s.pass_test}</td>
                            <td><span class=format!("st {}", s.status.class())>{s.status.label()}</span><span class="state">{s.state}</span></td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </ScrollTable>
    }
}
