//! Decodes codestreams written by OpenJPEG 2.5.4 and checks every sample
//! against the image that was encoded (see `tests/data/README.md`).
//!
//! Before these existed the decoder was only exercised against its own
//! synthetic inputs, and returned wrong samples for any standard codestream
//! (cool-japan/oxigeo#31).

#[cfg(test)]
mod tests {
    use super::super::types_2::Jpeg2000Reader;
    use std::io::Cursor;

    /// Decodes a single-tile, single-component codestream and returns its
    /// samples with the DC level shift undone (ISO/IEC 15444-1 Annex G.1.2).
    fn decode_unsigned(codestream: &[u8], precision: u8) -> Vec<i32> {
        let mut reader =
            Jpeg2000Reader::new(Cursor::new(codestream.to_vec())).expect("reader creation failed");
        reader.parse_headers().expect("parse_headers failed");
        let components = reader
            .decode_tile_to_components(0, 0)
            .expect("tile decode failed");
        let shift = 1i32 << (precision - 1);
        components[0].iter().map(|&x| x + shift).collect()
    }

    fn assert_matches(
        name: &str,
        decoded: &[i32],
        width: i32,
        height: i32,
        f: impl Fn(i32, i32) -> i32,
    ) {
        assert_eq!(
            decoded.len(),
            (width * height) as usize,
            "{name}: sample count"
        );
        for y in 0..height {
            for x in 0..width {
                let got = decoded[(y * width + x) as usize];
                let want = f(x, y);
                assert_eq!(got, want, "{name}: sample ({x}, {y})");
            }
        }
    }

    #[test]
    fn decodes_constant_8bit_without_wavelet() {
        let decoded = decode_unsigned(include_bytes!("../../tests/data/const_8x8_u8_l0.j2k"), 8);
        assert_matches("const_8x8_u8_l0", &decoded, 8, 8, |_, _| 200);
    }

    #[test]
    fn decodes_8bit_one_level() {
        let decoded = decode_unsigned(include_bytes!("../../tests/data/grad_33x17_u8_l1.j2k"), 8);
        assert_matches("grad_33x17_u8_l1", &decoded, 33, 17, |x, y| {
            (x * 11 + y * 29 + x * y) % 256
        });
    }

    #[test]
    fn decodes_11bit_two_levels_odd_size() {
        let decoded = decode_unsigned(include_bytes!("../../tests/data/grad_37x29_u11_l2.j2k"), 11);
        assert_matches("grad_37x29_u11_l2", &decoded, 37, 29, |x, y| {
            (x * 73 + y * 151 + x * y * 7) % 2048
        });
    }

    #[test]
    fn decodes_16bit_five_levels() {
        let decoded = decode_unsigned(include_bytes!("../../tests/data/grad_70x45_u16_l5.j2k"), 16);
        assert_matches("grad_70x45_u16_l5", &decoded, 70, 45, |x, y| {
            (x * 2917 + y * 7919 + x * y * 31) % 65536
        });
    }

    #[test]
    fn decodes_rgb_with_reversible_colour_transform() {
        let mut reader = Jpeg2000Reader::new(Cursor::new(
            include_bytes!("../../tests/data/rgb_11x7_u8_l1.j2k").to_vec(),
        ))
        .expect("reader creation failed");
        reader.parse_headers().expect("parse_headers failed");
        let rgb = reader.decode_rgb().expect("decode_rgb failed");
        assert_eq!(rgb.len(), 11 * 7 * 3, "sample count");
        for y in 0..7usize {
            for x in 0..11usize {
                let i = (y * 11 + x) * 3;
                let want = [
                    (x * 23 + y * 5) % 256,
                    (x * 7 + y * 31) % 256,
                    (x * x + y * 13) % 256,
                ];
                for (c, &w) in want.iter().enumerate() {
                    assert_eq!(usize::from(rgb[i + c]), w, "pixel ({x}, {y}) channel {c}");
                }
            }
        }
    }

    #[test]
    fn decode_rgb_applies_the_unsigned_dc_offset() {
        let mut reader = Jpeg2000Reader::new(Cursor::new(
            include_bytes!("../../tests/data/const_8x8_u8_l0.j2k").to_vec(),
        ))
        .expect("reader creation failed");
        reader.parse_headers().expect("parse_headers failed");
        let rgb = reader.decode_rgb().expect("decode_rgb failed");
        assert!(
            rgb.iter().all(|&v| v == 200),
            "every channel of every pixel is 200"
        );
    }
}
