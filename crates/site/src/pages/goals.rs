//! Research goals, proposed tests and the limits of current evidence.
use crate::components::page_nav::PageNav;
use crate::components::tables::ScrollTable;
use crate::util::set_title;
use leptos::prelude::*;
use loopseed_record::stages::{Stage, BECOMING_CLOSED, ORGANISM, SCALING};

const SECTIONS: &[(&str, &str)] = &[
    ("#benefit-h", "Human benefit"),
    ("#claim-h", "Learning"),
    ("#novelty-h", "New ideas"),
    ("#becoming", "Computational universe"),
    ("#next-h", "Planned tests"),
    ("#not-h", "Evidence limits"),
    ("#tracks", "Roadmap"),
];

#[component]
pub fn Goals() -> impl IntoView {
    set_title("Research goals");
    view! {
        <section class="page-intro page-intro-goals">
            <div class="wrap">
                <p class="eyebrow">"Research goals"</p>
                <h1 class="display display-xl">"Learning, discovery and reliable human control"</h1>
                <p class="lede">"We aim to develop systems that learn from verified experience, generate useful new ideas and remain open to correction. The long-term objective is broadly capable intelligence that supports human agency and wellbeing."</p>
                <p class="prose-p">"Progress requires evidence that useful abilities persist, new proposals survive testing and people retain effective control over how systems are used. The directions below set out the questions and proposed tests; the "<a href="/results">"Results page"</a>" states what has been measured so far."</p>
            </div>
        </section>

        <PageNav items=SECTIONS/>
        <section class="wrap section" aria-labelledby="benefit-h">
            <div class="section-head">
                <p class="eyebrow">"Research commitments"</p>
                <h2 id="benefit-h" class="display">"Useful learning under human control"</h2>
                <p class="lede-sm">"As a system learns, its response to evidence, instructions and intervention can change. Our central question is whether useful abilities can accumulate while people retain effective ways to inspect, correct and stop the system. The requirements below guide how we intend to evaluate that question."</p>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Define who benefits and who bears the risks."</b>" For each proposed application, identify the people affected and a concrete problem to solve, such as checking scientific analyses or finding software defects. Compare usefulness, errors and resource use with an established method. Assess how benefits and harms are distributed, with input from affected people."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Test human control after learning."</b>" Test whether authorised people can correct an error, revise the task, restrict actions and stop operation before and after each update. Record success rates and response times across repeated updates and unfamiliar conditions. Failure to respect an authorised intervention should disqualify an update even if its task scores improve."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Check the evidence behind an update."</b>" Record which observations, corrections and checks produced each update. Compare independently checked feedback with false or conflicting feedback, including repeated copies of the same claim. Measure whether the system corrects errors and retains the correction when tested on new cases."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Make each change inspectable."</b>" Preserve the starting version, learning data and resulting version so gains and failures can be traced. Share protocols, comparisons and negative results alongside "<a href="/#live-science">"public research sessions"</a>". Make "<a href="/record#where-h">"data and source records"</a>" available for review and replication, with privacy protections and access limits stated explicitly."</p></div>
            </div>
            <div class="prose">
                <p>"For proposed learning studies, we would specify the required improvement, permitted losses on earlier tasks and intervention tests before evaluation. An update would qualify only if it met all three requirements. The "<a href="/results">"current results"</a>" do not yet establish reliable human control through repeated learning."</p>
            </div>
            <details class="study-details">
                <summary>"Related work: human control and beneficial AI"</summary>
                <div class="study-details-content prose">
                    <p>"These commitments share concerns with the "<a href="https://futureoflife.org/about-us/">"Future of Life Institute’s mission"</a>" and the "<a href="https://futureoflife.org/open-letter/ai-principles/">"Asilomar AI Principles"</a>". Our proposed contribution is to test how successive learning updates affect usefulness, correction and the ability to intervene."</p>
                    <p>"For the broader research context, see "<a href="https://futureoflife.org/data/documents/research_priorities.pdf" rel="external">"Research Priorities for Robust and Beneficial Artificial Intelligence"</a>" (Russell, Dewey and Tegmark, 2015), which examines verification and control among the requirements for beneficial AI."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="claim-h">
            <div class="section-head">
                <p class="eyebrow">"Learning objectives"</p>
                <h2 id="claim-h" class="display">"Improvement must extend beyond the training examples"</h2>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Predict new observations"</h3>
                    <p>"Test whether experience improves predictions on observations excluded from training. Keep the measurement procedure fixed and compare with a system that did not receive the update. Assess uncertainty as well as average error."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Apply knowledge to unfamiliar tasks"</h3>
                    <p>"Reserve entire problem types and environments for evaluation. Compare systems with the same computational budget and access to tools. Distinguish applying a supplied rule from inferring a rule through experience."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Retain earlier abilities"</h3>
                    <p>"Revisit previously successful tasks after delays, new sessions and further training. Record declines alongside gains. Repeated evaluation should establish how long an improvement lasts and which conditions it depends on."</p>
                </div>
            </div>
            <p class="prose-p">"The broader ambition includes general intelligence: useful learning across a wide range of tasks and environments. Progress toward that ambition must be assessed together with the human-benefit and oversight requirements above."</p>
        </section>

        <section class="section section-feature" aria-labelledby="novelty-h">
            <div class="wrap">
                <div class="section-head">
                    <p class="eyebrow">"Proposed research · generation and evaluation"</p>
                    <h2 id="novelty-h" class="display">"Generating and testing new ideas"</h2>
                    <p class="lede-sm">"We aim to test whether interaction with external systems and environments helps a system generate new hypotheses, methods or designs. Such proposals would be evaluated for novelty against specified reference material, and tested separately for correctness and usefulness."</p>
                    <p class="prose-p">"The equation I = W(I) + you does not explicitly represent novelty. We treat new ideas as a possible emergent outcome of the interaction between internal computation and external input: the system may revise and combine what it has learned to propose something new. We will look for evidence of such outcomes, with claims of discovery requiring independent checks of novelty and validity."</p>
                </div>
                <div class="cols-3">
                    <div class="card">
                        <span class="n">"01"</span>
                        <h3>"Define what counts as new"</h3>
                        <p>"Specify the reference examples, known methods and prior work used for comparison. A proposal absent from the supplied material may still be familiar from earlier training or existing literature. A claim of scientific originality would need a separate review of prior work and expert assessment."</p>
                    </div>
                    <div class="card">
                        <span class="n">"02"</span>
                        <h3>"Test the proposal"</h3>
                        <p>"Evaluate predictions on unseen observations, execute proposed programs against a specification, or search for counterexamples to a proposed rule. Measure usefulness against a stated task and baseline. Record unsuccessful, duplicate and invalid proposals as well as accepted ones."</p>
                    </div>
                    <div class="card">
                        <span class="n">"03"</span>
                        <h3>"Identify what interaction contributes"</h3>
                        <p>"Hold the task, starting system and computational budget fixed while varying the additional observations it receives. Compare informative input with withheld or shuffled observations. Test whether useful proposals can be retained and applied in later settings."</p>
                    </div>
                </div>
                <p class="prose-p">"This direction is proposed. The existing mathematics and rule-learning studies concern tasks with known reference answers; they have not demonstrated open-ended scientific discovery."</p>
            </div>
        </section>

        <section id="becoming" class="wrap section" aria-labelledby="becoming-h">
            <div class="section-head">
                <p class="eyebrow">"Computation beyond language"</p>
                <h2 id="becoming-h" class="display">"Explore the computational universe"</h2>
                <p class="lede-sm">"Here, the computational universe means the space of possible programs, rules and the behaviours they generate. This use follows "<a href="https://wolframphysics.org/glossary/">"Wolfram’s terminology"</a>" for computational systems. It gives us a way to study learning in worlds whose rules and observations can be specified and checked."</p>
                <p class="prose-p">"Simple programs can produce complex patterns, as illustrated in "<a href="https://writings.stephenwolfram.com/2019/05/a-world-run-with-code/">"Wolfram’s discussion of the computational universe"</a>". For our research, the question is whether a learner can build useful models of such systems, choose informative experiments and adapt when the rules change."</p>
            </div>
            <div class="cols-3">
                <div class="card">
                    <span class="n">"01"</span>
                    <h3>"Programs and symbolic rules"</h3>
                    <p>"Study short programs, rules that transform symbol sequences and rules that change graph connections. Test whether a system can infer a useful rule, predict unseen cases or construct a program that satisfies a specification."</p>
                </div>
                <div class="card">
                    <span class="n">"02"</span>
                    <h3>"Simulated environments"</h3>
                    <p>"Use cellular automata—grids updated by local rules—and other dynamical simulations to study prediction, state estimation and action. Vary the rules, initial conditions and available observations, and compare performance under fixed resource limits."</p>
                </div>
                <div class="card">
                    <span class="n">"03"</span>
                    <h3>"Interacting systems"</h3>
                    <p>"Study agents that observe and influence one another. Trace how information moves between them, whether errors are repeated and whether checked knowledge can transfer without erasing earlier abilities. Define each system’s boundary and permitted actions."</p>
                </div>
            </div>
            <p class="prose-p">"Each experiment would select a finite set of rules, inputs and evaluation conditions. The underlying rule would be available to the evaluator and withheld from the learner when rule discovery is the task. Because several rules may fit the same limited observations, tests should assess new predictions and experiments that distinguish competing explanations."</p>
            <p class="prose-p">"The existing "<a href="/results#worlds">"rule-learning studies"</a>" provide a limited starting point. Proposed extensions would compare learned models with direct simulation, search and systems that receive no learning update, measuring accuracy, computational cost and failures on unfamiliar rules."</p>
            <details class="study-details">
                <summary>"The broader question about physical reality"</summary>
                <div class="study-details-content prose">
                    <p>"A separate line of inquiry, explored in the "<a href="https://www.wolframphysics.org/technical-introduction/">"Wolfram Physics Project"</a>", asks whether physical laws can be described by underlying computational rules. That is a hypothesis about nature, requiring specific predictions, comparison with physical observations and assessment against existing theories. Our software experiments do not establish that the physical universe is a computer."</p>
                </div>
            </details>
        </section>

        <section class="wrap section" aria-labelledby="next-h">
            <div class="section-head">
                <p class="eyebrow">"Proposed experiments"</p>
                <h2 id="next-h" class="display">"What the next tests need to establish"</h2>
                <p class="lede-sm">"Each proposal needs a protocol specifying the comparison, measurements and failure criteria before evaluation."</p>
            </div>
            <div class="rules">
                <div class="rule"><span class="n">"01"</span><p><b>"Learning through successive updates."</b>" Compare an updated system with its previous version on new tasks and a fixed set of earlier tasks. Keep prediction and task-success measures separate. Preserve the original evaluation sets and report the effect of each update."</p></div>
                <div class="rule"><span class="n">"02"</span><p><b>"Generation of useful proposals."</b>" Define novelty criteria before collection, freeze the reference material and keep the final evaluation examples out of the feedback loop. Compare valid, useful proposals per unit of computation under different input conditions."</p></div>
                <div class="rule"><span class="n">"03"</span><p><b>"Correction and human intervention."</b>" In bounded simulations, test returned copies of incorrect claims against fresh observations or exact checks. Separately test authorised interruption and changes to permissions before and after learning. Record failures to correct or comply with the permitted intervention."</p></div>
                <div class="rule"><span class="n">"04"</span><p><b>"Learning in computational worlds."</b>" Reserve whole rule classes and initial conditions from training. Compare prediction and action with direct simulation, search and unchanged-model baselines. Record which observations were available and how much computation each method used."</p></div>
                <div class="rule"><span class="n">"05"</span><p><b>"Verification of reusable programs."</b>" Extend checking from individual answers to programs with explicit specifications. Where feasible, require a machine-checkable proof covering the permitted inputs. Review the specification itself to establish whether it represents the intended task."</p></div>
                <div class="rule"><span class="n">"06"</span><p><b>"Independent reproduction."</b>" Package the software, settings, protocols and necessary data so another team can repeat a study. Reproducing a figure from supplied counts and independently rerunning the experiment should be reported as separate achievements."</p></div>
            </div>
        </section>

        <section class="wrap section" aria-labelledby="not-h">
            <div class="section-head">
                <p class="eyebrow">"Evidence limits"</p>
                <h2 id="not-h" class="display">"What remains unestablished"</h2>
            </div>
            <ul class="plain">
                <li>"The reported learning gains concern specific tasks and configurations. Reliable transfer to unfamiliar task families and sustained retention through repeated updates remain open."</li>
                <li>"No reported experiment demonstrates general intelligence, consciousness or open-ended scientific discovery."</li>
                <li>"The present results do not establish alignment with human values, reliable acceptance of intervention after learning, or resistance to misleading feedback."</li>
                <li>"Rule discovery in a simulated world is not evidence for a physical theory. Claims about nature require observations of the physical world."</li>
                <li>"The studies were conducted within the project. Separate AI review is fallible, and independent replication remains necessary. Detailed findings and qualifications are on the Results page."</li>
            </ul>
            <h3 id="falsify-h" class="figure-h">"Findings that would change the research direction"</h3>
            <p class="prose-p">"If gains disappear under matched controls, repeatedly fail on unfamiliar tasks or are offset by losses of earlier abilities, the proposed learning mechanism would need revision. If informative observations do not improve valid proposals over comparison conditions, the claimed contribution of interaction would lack support. Failure to respect authorised intervention would count against suitability for use even if task scores improved."</p>
        </section>

        <section id="tracks" class="wrap section" aria-labelledby="tracks-h">
            <div class="section-head">
                <p class="eyebrow">"Recorded development status"</p>
                <h2 id="tracks-h" class="display">"Roadmap and unresolved criteria"</h2>
                <p class="lede-sm">{BECOMING_CLOSED}</p>
            </div>
            <p class="prose-p">"These tables preserve the project’s stage identifiers and recorded decisions. Most statuses reflect the 7 September review; the paper entry now records the available working draft. ‘Closed’ means the project closed a stage, sometimes through an explicit waiver. It does not mean every original scientific criterion passed."</p>
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
            <p class="prose-p">"Symbols used in the roadmap are defined in the "<a href="/method#symbols">"Method page’s guide"</a>". The report list preserves the "<a href="/record#reports-h">"study outcomes and source records"</a>"."</p>
        </section>
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
