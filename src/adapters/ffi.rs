#[cfg(not(feature = "std"))]
compile_error!("ffi feature requires std; enable with `--features ffi`.");

use crate::{Assumptions, Inputs, compute_summary};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = RefCell::new(None);
}

fn set_last_error(msg: impl Into<String>) {
    let s = msg.into();
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(CString::new(s).unwrap_or_else(|_| CString::new("error").unwrap()));
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn salinity_last_error() -> *const c_char {
    LAST_ERROR.with(|e| match &*e.borrow() {
        Some(s) => s.as_ptr(),
        None => std::ptr::null(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn salinity_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

static VERSION_CSTR: &[u8] =
    concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), "\0").as_bytes();

#[unsafe(no_mangle)]
pub extern "C" fn salinity_version() -> *const c_char {
    VERSION_CSTR.as_ptr() as *const c_char
}

/// Compute a calculation summary from two JSON strings.
///
/// Parameters:
/// - `inputs_json`: JSON matching the `Inputs` struct.
/// - `assumptions_json`: JSON matching the `Assumptions` struct. If null, defaults are used.
///
/// Returns: newly-allocated C string containing JSON with the summary on success,
///          or null on error. Retrieve the error via `salinity_last_error()`.
#[unsafe(no_mangle)]
pub extern "C" fn salinity_compute_summary_json(
    inputs_json: *const c_char,
    assumptions_json: *const c_char,
) -> *mut c_char {
    let inp_str = unsafe {
        if inputs_json.is_null() {
            set_last_error("inputs_json is null");
            return std::ptr::null_mut();
        }
        match CStr::from_ptr(inputs_json).to_str() {
            Ok(s) => s,
            Err(_) => {
                set_last_error("inputs_json not valid UTF-8");
                return std::ptr::null_mut();
            }
        }
    };

    let assumptions: Assumptions = unsafe {
        if assumptions_json.is_null() {
            Assumptions::default()
        } else {
            let s = match CStr::from_ptr(assumptions_json).to_str() {
                Ok(s) => s,
                Err(_) => {
                    set_last_error("assumptions_json not valid UTF-8");
                    return std::ptr::null_mut();
                }
            };
            match serde_json::from_str::<Assumptions>(s) {
                Ok(v) => v,
                Err(e) => {
                    set_last_error(format!("failed to parse Assumptions: {}", e));
                    return std::ptr::null_mut();
                }
            }
        }
    };

    let inputs: Inputs = match serde_json::from_str(inp_str) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(format!("failed to parse Inputs: {}", e));
            return std::ptr::null_mut();
        }
    };

    let summary = compute_summary(&inputs, &assumptions);
    match serde_json::to_string(&summary) {
        Ok(js) => CString::new(js).unwrap().into_raw(),
        Err(e) => {
            set_last_error(format!("failed to serialize output: {}", e));
            std::ptr::null_mut()
        }
    }
}

#[repr(C)]
pub struct InputsC {
    pub na: f64,
    pub ca: f64,
    pub mg: f64,
    pub k: f64,
    pub sr: f64,
    pub br: f64,
    pub s: f64,
    pub b: f64,
    // Optional fields (use *_present as u8 0/1)
    pub cl: f64,
    pub cl_present: u8,
    pub f: f64,
    pub f_present: u8,
    pub alk_dkh: f64,
    pub alk_dkh_present: u8,
}

/// C-compatible representation of `Assumptions` with presence flags for optionals.
#[repr(C)]
pub struct AssumptionsC {
    pub temp: f64,
    pub pressure_dbar: f64,
    // optional alkalinity (used if inputs.alk_dkh missing)
    pub alkalinity: f64,
    pub alkalinity_present: u8,
    pub assume_borate: u8,
    pub default_f_mg_l: f64,
    pub ref_alk_dkh: f64,
    pub ref_alk_dkh_present: u8,
    pub salinity_norm: f64,
    pub return_components: u8,
    pub borate_fraction: f64,
    pub borate_fraction_present: u8,
    pub alk_mg_per_meq: f64,
    pub alk_mg_per_meq_present: u8,
    pub rn_compat: u8,
}

/// C-compatible output structure for `CalculationSummary`.
#[repr(C)]
pub struct CalculationSummaryC {
    pub sp: f64,
    pub sa: f64,
    pub density_kg_per_m3: f64,
    pub sg_20_20: f64,
    pub sg_25_25: f64,
}

fn to_inputs(c: &InputsC) -> Inputs {
    Inputs {
        na: c.na,
        ca: c.ca,
        mg: c.mg,
        k: c.k,
        sr: c.sr,
        br: c.br,
        s: c.s,
        b: c.b,
        cl: if c.cl_present != 0 { Some(c.cl) } else { None },
        f: if c.f_present != 0 { Some(c.f) } else { None },
        alk_dkh: if c.alk_dkh_present != 0 {
            Some(c.alk_dkh)
        } else {
            None
        },
    }
}

fn to_assumptions(c_opt: Option<&AssumptionsC>) -> Assumptions {
    match c_opt {
        None => Assumptions::default(),
        Some(c) => Assumptions {
            temp: c.temp,
            pressure_dbar: c.pressure_dbar,
            alkalinity: if c.alkalinity_present != 0 {
                Some(c.alkalinity)
            } else {
                None
            },
            assume_borate: c.assume_borate != 0,
            default_f_mg_l: c.default_f_mg_l,
            ref_alk_dkh: if c.ref_alk_dkh_present != 0 {
                Some(c.ref_alk_dkh)
            } else {
                None
            },
            salinity_norm: c.salinity_norm,
            return_components: c.return_components != 0,
            borate_fraction: if c.borate_fraction_present != 0 {
                Some(c.borate_fraction)
            } else {
                None
            },
            alk_mg_per_meq: if c.alk_mg_per_meq_present != 0 {
                Some(c.alk_mg_per_meq)
            } else {
                None
            },
            rn_compat: c.rn_compat != 0,
        },
    }
}

/// Compute a calculation summary using C-compatible structs. Returns 0 on success, non-zero on error.
#[unsafe(no_mangle)]
pub extern "C" fn salinity_compute_summary_struct(
    inputs_ptr: *const InputsC,
    assumptions_ptr: *const AssumptionsC, // nullable -> defaults
    out_summary: *mut CalculationSummaryC,
) -> i32 {
    if inputs_ptr.is_null() || out_summary.is_null() {
        set_last_error("inputs/out_summary pointer is null");
        return -1;
    }
    let inputs_r = unsafe { &*inputs_ptr };
    let assumptions_r = if assumptions_ptr.is_null() {
        None
    } else {
        Some(unsafe { &*assumptions_ptr })
    };

    let inputs = to_inputs(inputs_r);
    let assumptions = to_assumptions(assumptions_r);

    let summary = compute_summary(&inputs, &assumptions);
    let out = unsafe { &mut *out_summary };
    out.sp = summary.sp;
    out.sa = summary.sa;
    out.density_kg_per_m3 = summary.density_kg_per_m3;
    out.sg_20_20 = summary.sg_20_20;
    out.sg_25_25 = summary.sg_25_25;
    0
}
