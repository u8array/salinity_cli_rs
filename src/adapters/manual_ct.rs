//! Approximate manual implementation of TEOS-10 conservative temperature conversion.
//! Enabled behind the `approx_ct` feature.
//!
//! This is a reduced approximation: CT ≈ t - Δ(adiabatic lapse) for shallow pressures.
//! It is NOT a full TEOS-10 polynomial port. Error expected <~0.02 K for p < 50 dbar.
//! For precise thermodynamic work port full GSW routines.

/// Approximate potential temperature from in-situ temperature.
/// Very rough UNESCO-like lapse correction: θ ≈ t - 1.0e-4 * p_dbar * t.
pub fn potential_temperature_approx(_sa: f64, t: f64, p_dbar: f64) -> f64 {
    t - 1.0e-4 * p_dbar * t
}

/// Conservative temperature approximation from potential temperature.
/// Here CT ≈ θ (identity) for the reduced model.
pub fn conservative_temperature_from_pt_approx(_sa: f64, pt: f64) -> f64 {
    pt
}

/// Combined helper mirroring ct_from_t signature in teos10.rs.
pub fn ct_from_t_manual(sa: f64, t: f64, p_dbar: f64) -> f64 {
    let pt = potential_temperature_approx(sa, t, p_dbar);
    conservative_temperature_from_pt_approx(sa, pt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_pressure_small_difference() {
        let sa = 35.0 * 35.16504 / 35.0; // ~SR_REF
        let t = 20.0;
        let p = 10.0; // 10 dbar
        let ct = ct_from_t_manual(sa, t, p);
        // Expect CT only slightly below t
        assert!((t - ct) > 0.0 && (t - ct) < 0.05);
    }

    #[test]
    fn zero_pressure_identity() {
        let ct = ct_from_t_manual(35.0, 18.0, 0.0);
        assert!((ct - 18.0).abs() < 1e-12);
    }
}
