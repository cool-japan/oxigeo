# OpenJPEG reference codestreams

Raw JPEG2000 codestreams written by OpenJPEG 2.5.4 (`opj_compress`), used by
`src/reader/openjpeg_tests.rs` to check decoding against an independent encoder.
`oxigeo-grib` and `oxigeo-drivers-advanced` keep copies of the ones they use in
their own `tests/data`. Each is lossless (reversible 5/3, one quality layer,
64x64 code blocks, one tile, unsigned samples), and every pixel is a formula of
its coordinates, so the tests recompute the expected image instead of storing it.

| File | Size | Bits | Wavelet levels | Pixel `(x, y)` |
|---|---|---|---|---|
| `const_8x8_u8_l0.j2k` | 8x8 | 8 | 0 | `200` |
| `grad_33x17_u8_l1.j2k` | 33x17 | 8 | 1 | `(x*11 + y*29 + x*y) % 256` |
| `grad_37x29_u11_l2.j2k` | 37x29 | 11 | 2 | `(x*73 + y*151 + x*y*7) % 2048` |
| `grad_70x45_u16_l5.j2k` | 70x45 | 16 | 5 | `(x*2917 + y*7919 + x*y*31) % 65536` |
| `rgb_11x7_u8_l1.j2k` | 11x7, RGB | 8 | 1 | `((x*23 + y*5) % 256, (x*7 + y*31) % 256, (x*x + y*13) % 256)`, with the reversible colour transform (RCT) |

To regenerate one, write the image as a binary PGM (maxval `2^bits - 1`,
big-endian 16-bit samples above 8 bits), or a PPM for the RGB one, and run
`opj_compress -i image.pgm -o NAME.j2k -n LEVELS+1`.
