#[cfg(feature = "approx_ct")]
use crate::adapters::manual_ct::ct_from_t_manual;
use gsw as gsw_teos10;

/// Absolute/Reference Salinity from Practical Salinity.
/// Note: This returns TEOS-10 Reference Salinity (SR) from SP and is used
/// as an approximation for Absolute Salinity (SA). For standard seawater
/// composition SR ≈ SA; use location-based SA conversions if available.
pub fn sa_from_sp(sp: f64) -> f64 {
    gsw_teos10::conversions::sr_from_sp(sp)
}

/// Conservative Temperature (CT) from in-situ temperature t.
pub fn ct_from_t(sa: f64, temp: f64, p_dbar: f64) -> f64 {
    #[cfg(feature = "approx_ct")]
    {
        ct_from_t_manual(sa, temp, p_dbar)
    }

    // Currently a placeholder: CT ≈ t. The difference CT−t is small near
    // surface pressures but non-zero in general.
    // Todo: Replace with a proper t→CT conversion when the gsw crate exposes it.
    #[cfg(not(feature = "approx_ct"))]
    {
        let _ = sa;
        let _ = p_dbar;
        temp
    }
}

/// In-situ density ρ frofm SA, CT and p (TEOS-10, 75-term polynomial).
/// Falls back to 1025.0 kg/m³ if the gsw library returns an error.
pub fn rho(sa: f64, ct: f64, p_dbar: f64) -> f64 {
    gsw_teos10::volume::rho(sa, ct, p_dbar).unwrap_or(1025.0)
}
