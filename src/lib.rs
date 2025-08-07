use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use std::io::Cursor;
use sha2::{Digest, Sha256, Sha512};
use sha1::Sha1;
use md5::{Md5};
use regex::RegexBuilder;
use serde::{Serialize, Deserialize};

// --------------- BULK CONVERSIONS ---------------------
// ------------------------------------------------------
#[wasm_bindgen]
pub fn convert_columns(data_json: &str, from_unit: &str, to_unit: &str, whole_num: &JsValue, roundoff: &JsValue) -> String {
    let parsed: Vec<Vec<String>> = serde_json::from_str(data_json).unwrap_or_default();
    
    let whole=whole_num.as_bool().unwrap_or_default();
    let rounded=roundoff.as_bool().unwrap_or_default();

    let converted: Vec<Vec<String>> = parsed
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|val| {
                    match convert_value(&val, from_unit, to_unit, &whole, &rounded) {
                        Ok(v) => v.to_string(),
                        Err(_) => val,
                    }
                })
                .collect()
        })
        .collect();

    serde_json::to_string(&converted).unwrap()
}

fn convert_value(
    value: &str,
    from: &str,
    to: &str,
    whole_number: &bool,
    round_off: &bool,
) -> Result<f64, ()> {
    let parsed = value.parse::<f64>().map_err(|_| ())?;

    let result = match (from, to) {
        // Mass
        ("mg", "g") => parsed / 1000.0,
        ("g", "mg") => parsed * 1000.0,
        ("g", "kg") => parsed / 1000.0,
        ("kg", "g") => parsed * 1000.0,
        ("kg", "tonne") => parsed / 1000.0,
        ("tonne", "kg") => parsed * 1000.0,
        ("oz", "g") => parsed * 28.3495,
        ("g", "oz") => parsed / 28.3495,
        ("lb", "kg") => parsed * 0.453592,
        ("kg", "lb") => parsed / 0.453592,

        // Length / Distance
        ("mm", "cm") => parsed / 10.0,
        ("cm", "mm") => parsed * 10.0,
        ("cm", "m") => parsed / 100.0,
        ("m", "cm") => parsed * 100.0,
        ("m", "km") => parsed / 1000.0,
        ("km", "m") => parsed * 1000.0,
        ("inch", "cm") => parsed * 2.54,
        ("cm", "inch") => parsed / 2.54,
        ("ft", "m") => parsed * 0.3048,
        ("m", "ft") => parsed / 0.3048,
        ("yard", "m") => parsed * 0.9144,
        ("m", "yard") => parsed / 0.9144,
        ("mile", "km") => parsed * 1.60934,
        ("km", "mile") => parsed / 1.60934,

        // Area
        ("sqm", "sqft") => parsed * 10.7639,
        ("sqft", "sqm") => parsed / 10.7639,

        // Volume
        ("ml", "l") => parsed / 1000.0,
        ("l", "ml") => parsed * 1000.0,
        ("l", "gal") => parsed / 3.78541,
        ("gal", "l") => parsed * 3.78541,

        // Temperature
        ("celsius", "fahrenheit") => (parsed * 9.0 / 5.0) + 32.0,
        ("fahrenheit", "celsius") => (parsed - 32.0) * 5.0 / 9.0,
        ("celsius", "kelvin") => parsed + 273.15,
        ("kelvin", "celsius") => parsed - 273.15,
        ("fahrenheit", "kelvin") => (parsed + 459.67) / 1.8,
        ("kelvin", "fahrenheit") => (parsed * 1.8) - 459.67,

        // Speed
        ("kmh", "mph") => parsed / 1.60934,
        ("mph", "kmh") => parsed * 1.60934,
        ("ms", "kmh") => parsed * 3.6,
        ("kmh", "ms") => parsed / 3.6,

        // Power
        ("w", "kw") => parsed / 1000.0,
        ("kw", "w") => parsed * 1000.0,

        // Pressure
        ("pa", "kpa") => parsed / 1000.0,
        ("kpa", "pa") => parsed * 1000.0,
        ("bar", "pa") => parsed * 100000.0,
        ("pa", "bar") => parsed / 100000.0,
        ("psi", "pa") => parsed * 6894.76,
        ("pa", "psi") => parsed / 6894.76,

        // Energy
        ("j", "kj") => parsed / 1000.0,
        ("kj", "j") => parsed * 1000.0,
        ("j", "cal") => parsed / 4.184,
        ("cal", "j") => parsed * 4.184,

        // Digital Storage
        ("bit", "byte") => parsed / 8.0,
        ("byte", "bit") => parsed * 8.0,
        ("kb", "mb") => parsed / 1024.0,
        ("mb", "gb") => parsed / 1024.0,
        ("gb", "tb") => parsed / 1024.0,
        ("tb", "gb") => parsed * 1024.0,
        ("gb", "mb") => parsed * 1024.0,
        ("mb", "kb") => parsed * 1024.0,

        // Time
        ("sec", "min") => parsed / 60.0,
        ("min", "sec") => parsed * 60.0,
        ("min", "hr") => parsed / 60.0,
        ("hr", "min") => parsed * 60.0,
        ("hr", "day") => parsed / 24.0,
        ("day", "hr") => parsed * 24.0,

        // Sound
        ("dbm", "watt") => 10f64.powf(parsed / 10.0) / 1000.0,
        ("watt", "dbm") => 10.0 * (parsed * 1000.0).log10(),

        // Radiation
        ("gy", "rad") => parsed * 100.0,       // Gray to Rad
        ("rad", "gy") => parsed / 100.0,
        ("sv", "rem") => parsed * 100.0,       // Sievert to Rem
        ("rem", "sv") => parsed / 100.0,

        // Electricity
        ("amp", "milliamp") => parsed * 1000.0,
        ("milliamp", "amp") => parsed / 1000.0,
        ("volt", "millivolt") => parsed * 1000.0,
        ("millivolt", "volt") => parsed / 1000.0,
        ("ohm", "kiloohm") => parsed / 1000.0,
        ("kiloohm", "ohm") => parsed * 1000.0,
        ("farad", "microfarad") => parsed * 1_000_000.0,
        ("microfarad", "farad") => parsed / 1_000_000.0,
        ("coulomb", "millicoulomb") => parsed * 1000.0,
        ("millicoulomb", "coulomb") => parsed / 1000.0,
        ("watt_hour", "joule") => parsed * 3600.0,
        ("joule", "watt_hour") => parsed / 3600.0,

        // Illuminance
        ("lux", "footcandle") => parsed / 10.764,
        ("footcandle", "lux") => parsed * 10.764,

        // Angle
        ("deg", "rad") => parsed * std::f64::consts::PI / 180.0,
        ("rad", "deg") => parsed * 180.0 / std::f64::consts::PI,
        ("deg", "grad") => parsed * (200.0 / 180.0),
        ("grad", "deg") => parsed * (180.0 / 200.0),

        // Nautical
        ("nautical_mile", "km") => parsed * 1.852,
        ("km", "nautical_mile") => parsed / 1.852,
        ("knot", "kmh") => parsed * 1.852,
        ("kmh", "knot") => parsed / 1.852,

        // Frequency
        ("hz", "khz") => parsed / 1000.0,
        ("khz", "hz") => parsed * 1000.0,
        ("khz", "mhz") => parsed / 1000.0,
        ("mhz", "khz") => parsed * 1000.0,
        ("mhz", "ghz") => parsed / 1000.0,
        ("ghz", "mhz") => parsed * 1000.0,

        // Torque
        ("nm", "lbft") => parsed * 0.737562,
        ("lbft", "nm") => parsed / 0.737562,

        // Fuel Economy
        ("km_per_l", "l_per_100km") => 100.0 / parsed,
        ("l_per_100km", "km_per_l") => 100.0 / parsed,
        ("mpg_us", "km_per_l") => parsed * 0.425144,
        ("km_per_l", "mpg_us") => parsed / 0.425144,
        ("mpg_uk", "km_per_l") => parsed * 0.354006,
        ("km_per_l", "mpg_uk") => parsed / 0.354006,

        // Magnetism
        ("tesla", "gauss") => parsed * 10000.0,
        ("gauss", "tesla") => parsed / 10000.0,
        ("wb", "mwb") => parsed * 1_000_000.0,  // Weber to microweber
        ("mwb", "wb") => parsed / 1_000_000.0,

        // Building and Construction units
        ("sq_m", "sq_foot") => parsed * 10.7639,
        ("sq_foot", "sq_m") => parsed / 10.7639,
        ("cu_m", "cu_foot") => parsed * 35.3147,
        ("cu_foot", "cu_m") => parsed / 35.3147,



        _ => return Err(()),
    };



    // Apply formatting rules
    let final_result = if *whole_number {
        result.trunc()
    } else if *round_off {
        result.round()
    } else {
        result
    };

    Ok(final_result)
}

// Image Compressor -----------------
#[wasm_bindgen]
pub struct CompressedResult {
    bytes: Vec<u8>,
    format: String,
    size_kb: f64,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CompressedResult {
    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn size_kb(&self) -> f64 {
        self.size_kb
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }
}
#[wasm_bindgen]
pub fn compress_image(base64_input: &str, quality: u8, format: &str) -> Result<CompressedResult, JsValue> {
    // Strip data URL prefix
    let base64_data = base64_input.split(',').nth(1).ok_or("Invalid base64 input")?;
    let image_data = base64::decode(base64_data).map_err(|_| "Failed to decode base64")?;

    let img = image::load_from_memory(&image_data).map_err(|_| "Failed to load image")?;
    let (width, height) = img.dimensions();
    let mut buffer = Cursor::new(Vec::new());

    // Determine format
    let (target_format, fmt_str) = match format {
        "jpg" => (ImageOutputFormat::Jpeg(quality), "JPG"),
        "png" => (ImageOutputFormat::Png, "PNG"),
        "original" => {
            // Try to guess from data
            match image::guess_format(&image_data) {
                Ok(image::ImageFormat::Png) => (ImageOutputFormat::Png, "PNG"),
                _ => (ImageOutputFormat::Jpeg(quality), "JPG"), // fallback
            }
        }
        _ => return Err("Unknown format".into()),
    };

    img.write_to(&mut buffer, target_format)
        .map_err(|_| "Failed to encode image")?;

    let compressed_bytes = buffer.into_inner();
    let size_kb = compressed_bytes.len() as f64 / 1024.0;

    Ok(CompressedResult {
        bytes: compressed_bytes,
        format: fmt_str.to_string(),
        size_kb,
        width,
        height,
    })
}
#[wasm_bindgen]
pub fn hash_file_bytes(data: &[u8], algo: &str) -> String {
    match algo {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        _ => "Unknown algorithm".to_string(),
    }
}

// JSON Validator

#[derive(Serialize, Deserialize)]
pub struct MatchResult {
    start: usize,
    end: usize,
    matched: String,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RegexResponse {
    ok: bool,
    matches: Vec<MatchResult>,
    error: Option<String>,
}

#[wasm_bindgen]
pub fn run_regex(pattern: &str, text: &str, flags: &str) -> String {
    let mut builder = RegexBuilder::new(pattern);
    builder.case_insensitive(flags.contains('i'));
    builder.multi_line(flags.contains('m'));
    builder.dot_matches_new_line(flags.contains('s')); // optional
    builder.ignore_whitespace(false);

    let regex = match builder.build() {
        Ok(r) => r,
        Err(e) => {
            let error = RegexResponse {
                ok: false,
                matches: vec![],
                error: Some(e.to_string()),
            };
            return serde_json::to_string(&error).unwrap();
        }
    };

    let mut results = Vec::new();

    for cap in regex.captures_iter(text) {
        let mat = cap.get(0).unwrap();
        let groups: Vec<String> = cap.iter()
            .skip(1)
            .filter_map(|g| g.map(|m| m.as_str().to_string()))
            .collect();

        results.push(MatchResult {
            start: mat.start(),
            end: mat.end(),
            matched: mat.as_str().to_string(),
            groups,
        });

        if !flags.contains('g') {
            break;
        }
    }

    let response = RegexResponse {
        ok: true,
        matches: results,
        error: None,
    };

    serde_json::to_string(&response).unwrap()
}