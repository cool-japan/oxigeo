//! GRIB2 DRT 5.40 decoding of codestreams written by OpenJPEG 2.5.4 (see
//! `tests/data/README.md`), end to end through
//! `decode_jpeg2000_values`, including the DC level shift (cool-japan/oxigeo#31).

use oxigeo_grib::grib2::jpeg2000::decode_jpeg2000_values;

/// Decodes with R = 0, E = 0, D = 0, so the values are the integer samples X.
fn decode(codestream: &[u8], n: usize) -> Result<Vec<f32>, oxigeo_grib::GribError> {
    decode_jpeg2000_values(codestream, 0.0, 0, 0, n)
}

fn assert_matches(
    name: &str,
    values: &[f32],
    width: i64,
    height: i64,
    f: impl Fn(i64, i64) -> i64,
) {
    assert_eq!(
        values.len(),
        (width * height) as usize,
        "{name}: sample count"
    );
    for y in 0..height {
        for x in 0..width {
            let got = values[(y * width + x) as usize] as i64;
            assert_eq!(got, f(x, y), "{name}: sample ({x}, {y})");
        }
    }
}

#[test]
fn decodes_openjpeg_codestreams() {
    let values =
        decode(include_bytes!("data/const_8x8_u8_l0.j2k"), 8 * 8).expect("decode const_8x8_u8_l0");
    assert_matches("const_8x8_u8_l0", &values, 8, 8, |_, _| 200);
    let values = decode(include_bytes!("data/grad_33x17_u8_l1.j2k"), 33 * 17)
        .expect("decode grad_33x17_u8_l1");
    assert_matches("grad_33x17_u8_l1", &values, 33, 17, |x, y| {
        (x * 11 + y * 29 + x * y) % 256
    });
    let values = decode(include_bytes!("data/grad_37x29_u11_l2.j2k"), 37 * 29)
        .expect("decode grad_37x29_u11_l2");
    assert_matches("grad_37x29_u11_l2", &values, 37, 29, |x, y| {
        (x * 73 + y * 151 + x * y * 7) % 2048
    });
    let values = decode(include_bytes!("data/grad_70x45_u16_l5.j2k"), 70 * 45)
        .expect("decode grad_70x45_u16_l5");
    assert_matches("grad_70x45_u16_l5", &values, 70, 45, |x, y| {
        (x * 2917 + y * 7919 + x * y * 31) % 65536
    });
}

#[test]
fn applies_grib2_scaling_after_the_level_shift() {
    // 200 at every point, scaled by (R + X * 2^E) / 10^D with R = 1000, E = 1, D = 1.
    let values =
        decode_jpeg2000_values(include_bytes!("data/const_8x8_u8_l0.j2k"), 1000.0, 1, 1, 64)
            .expect("decode failed");
    assert!(
        values.iter().all(|&v| (v - 140.0).abs() < 1e-4),
        "(1000 + 200 * 2) / 10 = 140"
    );
}
