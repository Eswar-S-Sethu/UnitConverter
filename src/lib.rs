use wasm_bindgen::prelude::*;

// ==========================
//   MASS / WEIGHT CONVERSIONS
// ==========================

#[wasm_bindgen]
pub fn lbs_to_kg(lbs: f64) -> f64 {
    lbs / 2.20462
}

#[wasm_bindgen]
pub fn kg_to_lbs(kg: f64) -> f64 {
    kg * 2.20462
}

// ==========================
//   LENGTH / DISTANCE CONVERSIONS
// ==========================

#[wasm_bindgen]
pub fn cm_to_in(cm: f64) -> f64 {
    cm / 2.54
}

#[wasm_bindgen]
pub fn in_to_cm(inches: f64) -> f64 {
    inches * 2.54
}

#[wasm_bindgen]
pub fn m_to_km(m: f64) -> f64 {
    m / 1000.0
}

#[wasm_bindgen]
pub fn km_to_m(km: f64) -> f64 {
    km * 1000.0
}

// ==========================
//   TEMPERATURE CONVERSIONS
// ==========================

#[wasm_bindgen]
pub fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

#[wasm_bindgen]
pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

#[wasm_bindgen]
pub fn c_to_k(c: f64) -> f64 {
    c + 273.15
}

#[wasm_bindgen]
pub fn k_to_c(k: f64) -> f64 {
    k - 273.15
}

// ==========================
//   SPEED CONVERSIONS
// ==========================

#[wasm_bindgen]
pub fn kmh_to_mph(kmh: f64) -> f64 {
    kmh * 0.621371
}

#[wasm_bindgen]
pub fn mph_to_kmh(mph: f64) -> f64 {
    mph / 0.621371
}

// =============================
// AREA CONVERSIONS
// =============================

#[wasm_bindgen]
pub fn sqm_to_sqft(sqm: f64) -> f64 {
    sqm * 10.7639
}

#[wasm_bindgen]
pub fn sqft_to_sqm(sqft: f64) -> f64 {
    sqft / 10.7639
}

// =============================
// VOLUME CONVERSIONS
// =============================

#[wasm_bindgen]
pub fn l_to_gal(l: f64) -> f64 {
    l * 0.264172
}

#[wasm_bindgen]
pub fn gal_to_l(gal: f64) -> f64 {
    gal / 0.264172
}

// ============================
// TIME CONVERSIONS
// ============================

#[wasm_bindgen]
pub fn hrs_to_min(hrs: f64) -> f64 {
    hrs * 60.0
}

#[wasm_bindgen]
pub fn min_to_sec(min: f64) -> f64 {
    min * 60.0
}

// ============================
// PRESSURE CONVERSIONS
// ============================

#[wasm_bindgen]
pub fn pa_to_psi(pa: f64) -> f64 {
    pa * 0.000145038
}

#[wasm_bindgen]
pub fn psi_to_pa(psi: f64) -> f64 {
    psi / 0.000145038
}

// ============================
// ENERGY CONVERSIONS
// ============================

#[wasm_bindgen]
pub fn j_to_cal(j: f64) -> f64 {
    j / 4.184
}

#[wasm_bindgen]
pub fn cal_to_j(cal: f64) -> f64 {
    cal * 4.184
}

// ===========================
// POWER CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn w_to_kw(w: f64) -> f64 {
    w / 1000.0
}

#[wasm_bindgen]
pub fn kw_to_w(kw: f64) -> f64 {
    kw * 1000.0
}

// ===========================
// DIGITAL STORAGE CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn kb_to_mb(kb: f64) -> f64 {
    kb / 1024.0
}

#[wasm_bindgen]
pub fn mb_to_gb(mb: f64) -> f64 {
    mb / 1024.0
}

#[wasm_bindgen]
pub fn gb_to_tb(gb: f64) -> f64 {
    gb / 1024.0
}

// ===========================
// RING SIZE CONVERSIONS (US ↔ EU)
// ===========================

#[wasm_bindgen]
pub fn us_to_eu_ring(us_size: f64) -> f64 {
    (us_size * 2.5) + 36.5
}

#[wasm_bindgen]
pub fn eu_to_us_ring(eu_size: f64) -> f64 {
    (eu_size - 36.5) / 2.5
}

// ===========================
// Niche Conversions
// ===========================

// Shoe size US to EU (approx)
#[wasm_bindgen]
pub fn us_shoe_to_eu(us_size: f64) -> f64 {
    us_size + 33.0
}

#[wasm_bindgen]
pub fn eu_shoe_to_us(eu_size: f64) -> f64 {
    eu_size - 33.0
}

// ===========================
// CLOTHING SIZE CONVERSIONS (US ↔ EU)
// ===========================

#[wasm_bindgen]
pub fn us_to_eu_clothing(us_size: f64) -> f64 {
    us_size + 30.0
}

#[wasm_bindgen]
pub fn eu_to_us_clothing(eu_size: f64) -> f64 {
    eu_size - 30.0
}

// ===========================
// COOKING CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn tsp_to_tbsp(tsp: f64) -> f64 {
    tsp / 3.0
}

#[wasm_bindgen]
pub fn tbsp_to_tsp(tbsp: f64) -> f64 {
    tbsp * 3.0
}

#[wasm_bindgen]
pub fn cup_to_ml(cup: f64) -> f64 {
    cup * 236.588
}

#[wasm_bindgen]
pub fn ml_to_cup(ml: f64) -> f64 {
    ml / 236.588
}

#[wasm_bindgen]
pub fn oz_to_grams(oz: f64) -> f64 {
    oz * 28.3495
}

#[wasm_bindgen]
pub fn grams_to_oz(g: f64) -> f64 {
    g / 28.3495
}

// ===========================
// FITNESS UNIT CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn steps_to_km(steps: f64) -> f64 {
    steps * 0.000762
}

#[wasm_bindgen]
pub fn km_to_steps(km: f64) -> f64 {
    km / 0.000762
}

// #[wasm_bindgen]
// pub fn calories_burned_per_mets(mets: f64, weight_kg: f64, duration_min: f64) -> f64 {
//     mets * 3.5 * weight_kg / 200.0 * duration_min
// }

// ===========================
// FORCE CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn newtons_to_pounds_force(n: f64) -> f64 {
    n * 0.224809
}

#[wasm_bindgen]
pub fn pounds_force_to_newtons(lbf: f64) -> f64 {
    lbf / 0.224809
}

// ===========================
// TYPING SPEED CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn wpm_to_cpm(wpm: f64) -> f64 {
    wpm * 5.0
}

#[wasm_bindgen]
pub fn cpm_to_wpm(cpm: f64) -> f64 {
    cpm / 5.0
}

// ===========================
// UNIVERSITY GRADE CONVERSIONS
// ===========================

#[wasm_bindgen]
pub fn percent_to_gpa(percent: f64) -> f64 {
    match percent {
        p if p >= 85.0 => 4.0,
        p if p >= 75.0 => 3.7,
        p if p >= 65.0 => 3.3,
        p if p >= 55.0 => 2.7,
        p if p >= 50.0 => 2.0,
        _ => 0.0,
    }
}

#[wasm_bindgen]
pub fn gpa_to_percent(gpa: f64) -> f64 {
    match gpa {
        g if g >= 4.0 => 90.0,
        g if g >= 3.7 => 80.0,
        g if g >= 3.3 => 70.0,
        g if g >= 2.7 => 60.0,
        g if g >= 2.0 => 50.0,
        _ => 40.0,
    }
}

