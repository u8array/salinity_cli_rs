use gsw as gsw_teos10;

pub fn sa_from_sp(sp: f64) -> f64 {
    gsw_teos10::conversions::sr_from_sp(sp)
}

pub fn ct_from_t(_sa: f64, temp: f64, _p_dbar: f64) -> f64 {
    temp
}

pub fn rho(sa: f64, ct: f64, p_dbar: f64) -> f64 {
    gsw_teos10::volume::rho(sa, ct, p_dbar).unwrap_or(1025.0)
}
