//! Exact McNemar tests and Bonferroni Clopper–Pearson bounds.
//! Matches `fish/lab/claim_evolution.py`, including 100-step bisection.

/// Binomial coefficient as a float.
pub fn choose(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut acc = 1.0_f64;
    for i in 1..=k {
        acc = acc * (n - k + i) as f64 / i as f64;
    }
    acc
}

/// `P[X ≥ successes]` for `X ~ Binomial(trials, probability)`.
pub fn binomial_upper_tail(successes: u64, trials: u64, probability: f64) -> f64 {
    if successes == 0 {
        return 1.0;
    }
    if successes > trials {
        return 0.0;
    }
    (successes..=trials)
        .map(|k| {
            choose(trials, k)
                * probability.powi(k as i32)
                * (1.0 - probability).powi((trials - k) as i32)
        })
        .sum()
}

/// Exact one-sided lower confidence bound on a binomial proportion.
pub fn clopper_pearson_lower(successes: u64, trials: u64, alpha: f64) -> f64 {
    if trials == 0 || successes == 0 || !(0.0 < alpha && alpha < 1.0) {
        return 0.0;
    }
    let successes = successes.min(trials);
    let (mut low, mut high) = (0.0_f64, 1.0_f64);
    for _ in 0..100 {
        let middle = (low + high) / 2.0;
        if binomial_upper_tail(successes, trials, middle) < alpha {
            low = middle;
        } else {
            high = middle;
        }
    }
    (low + high) / 2.0
}

/// Exact one-sided upper confidence bound on a binomial proportion.
pub fn clopper_pearson_upper(successes: u64, trials: u64, alpha: f64) -> f64 {
    if trials == 0 || successes >= trials {
        return 1.0;
    }
    1.0 - clopper_pearson_lower(trials - successes, trials, alpha)
}

/// Two 2.5 % one-sided bounds give a 95 % Bonferroni bound.
pub const ALPHA_ONE_SIDED: f64 = 0.025;

/// Paired statistics over `n` matched tasks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Paired {
    pub n: u64,
    pub gains: u64,
    pub losses: u64,
    pub ties: u64,
    /// One-sided exact McNemar: `P[Binomial(gains + losses, ½) ≥ gains]`.
    pub p_value: f64,
    /// Lower gain-rate bound minus upper loss-rate bound.
    pub lower_difference: f64,
    pub observed_difference: f64,
}

pub fn paired(gains: u64, losses: u64, n: u64) -> Paired {
    let n = n.max(gains + losses).max(1);
    let discordant = gains + losses;
    let p_value = if discordant == 0 {
        1.0
    } else {
        binomial_upper_tail(gains, discordant, 0.5)
    };
    let lower_difference = clopper_pearson_lower(gains, n, ALPHA_ONE_SIDED)
        - clopper_pearson_upper(losses, n, ALPHA_ONE_SIDED);
    Paired {
        n,
        gains,
        losses,
        ties: n - discordant,
        p_value,
        lower_difference,
        observed_difference: (gains as f64 - losses as f64) / n as f64,
    }
}

impl Paired {
    /// Requires `p < 0.05` and a positive lower bound.
    pub fn superior(&self) -> bool {
        self.p_value < 0.05 && self.lower_difference > 0.0
    }
}

/// `4.77 × 10⁻⁷` style rendering for probabilities.
pub fn scientific(value: f64) -> String {
    if value <= 0.0 {
        return "0".to_string();
    }
    if value >= 0.01 {
        return format!("{value:.3}");
    }
    let exponent = value.log10().floor() as i32;
    let mantissa = value / 10f64.powi(exponent);
    format!("{mantissa:.2} × 10{}", superscript(exponent))
}

fn superscript(exponent: i32) -> String {
    const DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let mut out = String::new();
    if exponent < 0 {
        out.push('⁻');
    }
    for ch in exponent.unsigned_abs().to_string().chars() {
        out.push(DIGITS[ch.to_digit(10).unwrap_or(0) as usize]);
    }
    out
}

/// `+0.076` style rendering for differences.
pub fn signed(value: f64, decimals: usize) -> String {
    if value > 0.0 {
        format!("+{value:.decimals$}")
    } else {
        format!("{value:.decimals$}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    #[test]
    fn exact_mcnemar_with_no_losses_is_a_power_of_two() {
        assert_eq!(paired(21, 0, 128).p_value, 2f64.powi(-21));
        assert_eq!(paired(42, 0, 128).p_value, 2f64.powi(-42));
        assert_eq!(paired(25, 0, 64).p_value, 2f64.powi(-25));
        assert_eq!(paired(43, 0, 64).p_value, 2f64.powi(-43));
        assert_eq!(paired(38, 0, 64).p_value, 2f64.powi(-38));
        assert_eq!(paired(0, 0, 10).p_value, 1.0);
        assert_eq!(paired(0, 2, 10).p_value, 1.0);
        assert_eq!(paired(1, 2, 10).p_value, 0.875);
    }

    #[test]
    fn frozen_primary_ruling_reproduces() {
        // fish/album/2026-09-03T0028-EAT-claim-coat-primary-ruling.md
        assert!(close(
            paired(21, 0, 128).lower_difference,
            0.07611204480423511,
            1e-9
        ));
        assert!(close(
            paired(42, 0, 128).lower_difference,
            0.21934735023473845,
            1e-9
        ));
    }

    #[test]
    fn frozen_transfer_rulings_reproduce() {
        // fish/album/2026-09-03T1908-EAT-formal-claim-transfer-stage1.md
        assert!(close(
            paired(25, 0, 64).lower_difference,
            0.2150264321730908,
            1e-9
        ));
        // fish/album/2026-09-04T0043-EAT-clean-formal-claim-transfer.md (quoted to four places)
        assert!(close(paired(43, 0, 64).lower_difference, 0.4871, 5e-5));
        assert!(close(paired(38, 0, 64).lower_difference, 0.4077, 5e-5));
    }

    #[test]
    fn nine_gains_is_the_zero_loss_minimum_at_128() {
        assert!(paired(8, 0, 128).lower_difference < 0.0);
        assert!(paired(9, 0, 128).lower_difference > 0.0);
        assert!(!paired(8, 0, 128).superior());
        assert!(paired(9, 0, 128).superior());
    }

    #[test]
    fn ties_and_losses_are_not_evidence() {
        let tied = paired(0, 0, 32);
        assert_eq!(tied.p_value, 1.0);
        assert!(tied.lower_difference < 0.0);
        let losing = paired(0, 2, 32);
        assert_eq!(losing.p_value, 1.0);
        assert!(losing.lower_difference < 0.0);
    }

    #[test]
    fn rendering() {
        assert_eq!(scientific(4.76837158203125e-7), "4.77 × 10⁻⁷");
        assert_eq!(scientific(2.2737367544323206e-13), "2.27 × 10⁻¹³");
        assert_eq!(scientific(0.85546875), "0.855");
        assert_eq!(signed(0.0761, 3), "+0.076");
        assert_eq!(signed(-0.0287, 3), "-0.029");
    }
}
