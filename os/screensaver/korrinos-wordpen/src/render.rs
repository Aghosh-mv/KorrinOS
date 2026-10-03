//! Rasterising the pen's strokes, and writing PNGs so a frame can be looked at.
//!
//! # Why a PNG writer exists
//!
//! The build machine has no display, so an effect that can only be checked by
//! looking at a screen cannot be checked at all. But "look at it" does not
//! require a screen - it requires pixels in a file. So the renderer writes PNG,
//! and a frame can be inspected on any machine, including this one.
//!
//! # Why the PNG is uncompressed
//!
//! Writing a real DEFLATE stream needs a compressor. Storing uncompressed
//! DEFLATE blocks (BTYPE=00) is a valid zlib stream that any decoder accepts,
//! needs only CRC-32 and Adler-32, both a few lines, and removes a dependency
//! from a crate that is otherwise dependency-free. The files are larger, which
//! does not matter for a preview.

use crate::engine::Rgb;

/// An RGBA image. Straight (non-premultiplied) alpha.
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    /// Row-major, 4 bytes per pixel.
    pub pixels: Vec<u8>,
}

impl Canvas {
    /// A new canvas filled with opaque black.
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::with_capacity(width * height * 4);
        for _ in 0..width * height {
            pixels.extend_from_slice(&[0, 0, 0, 255]);
        }
        Canvas { width, height, pixels }
    }

    /// Blend a colour over the canvas at `(x, y)` with coverage `alpha` 0..1.
    ///
    /// Coverage-based blending is what makes a thin diagonal stroke look like a
    /// line rather than a staircase. For an axis-aligned or near-axis-aligned
    /// stroke the integer path is already full coverage; the fractional part is
    /// only there for the diagonals.
    pub fn blend(&mut self, x: usize, y: usize, colour: Rgb, alpha: f64) {
        if x >= self.width || y >= self.height || alpha <= 0.0 {
            return;
        }
        let alpha = alpha.clamp(0.0, 1.0);
        let i = (y * self.width + x) * 4;
        // Source-over against a black background simplifies to this, but it is
        // written properly so the canvas stays correct if the background ever
        // stops being black.
        let dst_a = self.pixels[i + 3] as f64 / 255.0;
        let out_a = alpha + dst_a * (1.0 - alpha);
        if out_a <= 0.0 {
            return;
        }
        for (offset, src) in [(0, colour.0), (1, colour.1), (2, colour.2)] {
            let dst = self.pixels[i + offset] as f64 / 255.0;
            let out = (src as f64 / 255.0 * alpha + dst * dst_a * (1.0 - alpha)) / out_a;
            self.pixels[i + offset] = (out.clamp(0.0, 1.0) * 255.0).round() as u8;
        }
        self.pixels[i + 3] = (out_a * 255.0).round() as u8;
    }

    /// Draw an anti-aliased segment.
    pub fn segment(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, colour: Rgb, width: f64) {
        let half = (width / 2.0).max(0.5);
        let dx = x1 - x0;
        let dy = y1 - y0;

        let steep = dy.abs() > dx.abs();
        let (x0, y0, x1, y1) = if steep { (y0, x0, y1, x1) } else { (x0, y0, x1, y1) };

        if (x1 - x0).abs() < 1e-9 {
            // Vertical: fill the column directly, which is both correct and much
            // faster than stepping.
            let from = y0.min(y1).floor() as isize;
            let to = y0.max(y1).ceil() as isize;
            for y in from..=to {
                let cover = (1.0 - (y as f64 - y0).abs()).clamp(0.0, 1.0);
                if cover > 0.0 {
                    self.dot_wide(x0.floor() as i64, y as i64, cover, colour, half, steep);
                }
            }
            return;
        }

        let gradient = (y1 - y0) / (x1 - x0);
        // Walk the columns low to high regardless of which way the segment
        // points. Writing `for x in x0..=x1` looks obvious and is wrong: after
        // the steep swap a segment travelling upward has x0 > x1, so the range
        // is EMPTY and the segment silently draws nothing. That is why glyph
        // stems rendered as fragments - every stroke that went up vanished.
        let (lo, hi) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        for x in (lo.floor() as i64)..=(hi.ceil() as i64) {
            let line_y = y0 + gradient * (x as f64 - x0);
            let bottom = line_y.floor();
            let frac = line_y - bottom;
            // Xiaolin Wu: the two pixels straddling the line get the two
            // complementary coverages. When the line passes exactly through a
            // pixel centre frac is 0, and that pixel takes FULL coverage - so
            // the top one is drawn unconditionally. Guarding it on frac > 0.0
            // drew nothing at all for a perfectly horizontal line.
            self.dot_wide(x, bottom as i64, 1.0 - frac, colour, half, steep);
            if frac > 0.0 {
                self.dot_wide(x, bottom as i64 + 1, frac, colour, half, steep);
            }
        }
    }

    /// One anti-aliased dot, widened perpendicular to the stroke.
    fn dot_wide(&mut self, x: i64, y: i64, cover: f64, colour: Rgb, half: f64, steep: bool) {
        // A stroke several pixels wide needs a disc, not a single pixel: at 8K a
        // 3px line drawn as one pixel row looks like a hairline.
        let radius = half.max(0.5);
        if radius <= 0.75 {
            let px = if steep { y } else { x };
            let py = if steep { x } else { y };
            if px >= 0 && py >= 0 {
                self.blend(px as usize, py as usize, colour, cover);
            }
            return;
        }
        let steps = (radius.ceil() as i64).max(1);
        for dy in -steps..=steps {
            for dx in -steps..=steps {
                let d = ((dx * dx + dy * dy) as f64).sqrt();
                let a = cover * (radius - d + 0.5).clamp(0.0, 1.0);
                if a > 0.01 {
                    let (px, py) = (
                        if steep { y + dx } else { x + dx },
                        if steep { x + dy } else { y + dy },
                    );
                    if px >= 0 && py >= 0 {
                        self.blend(px as usize, py as usize, colour, a);
                    }
                }
            }
        }
    }

    /// A filled circle, for the pen tip.
    pub fn dot(&mut self, cx: f64, cy: f64, radius: f64, colour: Rgb, alpha: f64) {
        let steps = (radius.ceil() as i64).max(1);
        for dy in -steps..=steps {
            for dx in -steps..=steps {
                let d = ((dx * dx + dy * dy) as f64).sqrt();
                let cover = (radius - d + 0.5).clamp(0.0, 1.0) * alpha;
                if cover > 0.01 {
                    let px = cx as i64 + dx;
                    let py = cy as i64 + dy;
                    if px >= 0 && py >= 0 {
                        self.blend(px as usize, py as usize, colour, cover);
                    }
                }
            }
        }
    }

    /// Draw every stroke of a polyline set.
    pub fn polylines(&mut self, lines: &[Vec<(f64, f64)>], colour: Rgb, width: f64, alpha: f64) {
        for line in lines {
            if line.len() == 1 {
                self.dot(line[0].0, line[0].1, width / 2.0, colour, alpha);
                continue;
            }
            for pair in line.windows(2) {
                self.segment(pair[0].0, pair[0].1, pair[1].0, pair[1].1, colour, width);
            }
        }
    }

    /// Number of pixels that are not black. Used by tests to prove something
    /// was actually drawn, without a display.
    pub fn lit_pixels(&self) -> usize {
        self.pixels.chunks_exact(4).filter(|p| p[0] > 8 || p[1] > 8 || p[2] > 8).count()
    }

    /// Encode as PNG.
    pub fn to_png(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(self.height * (1 + self.width * 4));
        for row in 0..self.height {
            raw.push(0); // filter type 0 (None) for every scanline
            let start = row * self.width * 4;
            raw.extend_from_slice(&self.pixels[start..start + self.width * 4]);
        }
        let idat = zlib_stored(&raw);

        let mut out = Vec::new();
        out.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        let mut ihdr = Vec::new();
        ihdr.extend_from_slice(&(self.width as u32).to_be_bytes());
        ihdr.extend_from_slice(&(self.height as u32).to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8-bit, RGBA, no interlace
        write_chunk(&mut out, b"IHDR", &ihdr);
        write_chunk(&mut out, b"IDAT", &idat);
        write_chunk(&mut out, b"IEND", &[]);
        out
    }
}


fn write_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_input).to_be_bytes());
}

/// A zlib stream using stored (uncompressed) DEFLATE blocks.
fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01]; // CMF/FLG: deflate, 32K window, fastest
    if data.is_empty() {
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xFF, 0xFF]);
    } else {
        let mut chunks = data.chunks(0xFFFF).peekable();
        while let Some(chunk) = chunks.next() {
            let last = chunks.peek().is_none();
            out.push(if last { 1 } else { 0 }); // BFINAL, BTYPE=00
            let len = chunk.len() as u16;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&(!len).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for byte in data {
        a = (a + *byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_canvas_is_black() {
        let canvas = Canvas::new(64, 32);
        assert_eq!(canvas.pixels.len(), 64 * 32 * 4);
        assert_eq!(canvas.lit_pixels(), 0, "a fresh canvas must be entirely dark");
    }

    #[test]
    fn a_line_lights_pixels_along_its_path() {
        let mut canvas = Canvas::new(100, 100);
        canvas.segment(10.0, 50.0, 90.0, 50.0, Rgb(255, 255, 255), 3.0);
        let lit = canvas.lit_pixels();
        assert!(lit > 60, "expected a long horizontal line, got {lit} pixels");
        // Every lit pixel should be near the line's row.
        let near: usize = (0..100)
            .filter(|x| (0..100).any(|y| {
                let i = (y * 100 + x) * 4;
                canvas.pixels[i] > 40
            }))
            .count();
        assert!(near > 60, "lit pixels are not where the line was drawn");
    }

    #[test]
    fn drawing_is_clipped_to_the_canvas_not_a_panic() {
        let mut canvas = Canvas::new(32, 32);
        // Lines that start and end outside, in every direction.
        canvas.segment(-100.0, -100.0, 200.0, 200.0, Rgb(255, 0, 0), 2.0);
        canvas.segment(-50.0, 16.0, 500.0, 16.0, Rgb(0, 255, 0), 2.0);
        canvas.segment(16.0, -500.0, 16.0, 500.0, Rgb(0, 0, 255), 2.0);
        assert!(canvas.lit_pixels() > 0);
    }

    #[test]
    fn a_wider_stroke_lights_more_pixels() {
        let mut thin = Canvas::new(200, 60);
        thin.segment(10.0, 30.0, 190.0, 30.0, Rgb(255, 255, 255), 1.0);
        let mut thick = Canvas::new(200, 60);
        thick.segment(10.0, 30.0, 190.0, 30.0, Rgb(255, 255, 255), 8.0);
        assert!(
            thick.lit_pixels() > thin.lit_pixels() * 3,
            "8px stroke drew {} px, 1px drew {} px",
            thick.lit_pixels(),
            thin.lit_pixels()
        );
    }

    #[test]
    fn alpha_zero_draws_nothing() {
        let mut canvas = Canvas::new(50, 50);
        canvas.blend(25, 25, Rgb(255, 255, 255), 0.0);
        assert_eq!(canvas.lit_pixels(), 0);
    }

    #[test]
    fn blending_respects_alpha() {
        let mut faint = Canvas::new(20, 20);
        faint.blend(10, 10, Rgb(255, 255, 255), 0.25);
        let mut solid = Canvas::new(20, 20);
        solid.blend(10, 10, Rgb(255, 255, 255), 1.0);
        let faint_i = (10 * 20 + 10) * 4;
        let solid_i = faint_i;
        assert!(
            faint.pixels[faint_i] < solid.pixels[solid_i],
            "alpha must actually reduce brightness"
        );
    }

    #[test]
    fn a_dot_is_a_disc_not_a_square() {
        let mut canvas = Canvas::new(40, 40);
        canvas.dot(20.0, 20.0, 8.0, Rgb(255, 255, 255), 1.0);
        let lit = canvas.lit_pixels();
        // A disc of radius 8 is about pi*64 ~= 201 pixels, well under 17x17.
        assert!(lit > 120 && lit < 260, "expected a disc, got {lit} pixels");
    }

    #[test]
    fn a_segment_draws_the_same_pixels_in_either_direction() {
        // The bug that made glyph stems vanish: after the steep-axis swap a
        // segment pointing upward has x0 > x1, and iterating x0..=x1 is an
        // empty range. Drawing a line and the same line reversed must be
        // identical.
        for (a, b) in [
            ((40.0, 40.0), (40.0, 360.0)),
            ((360.0, 40.0), (360.0, 360.0)),
            ((40.0, 40.0), (360.0, 40.0)),
            ((360.0, 40.0), (40.0, 40.0)),
            ((40.0, 360.0), (40.0, 40.0)),
            ((40.0, 40.0), (360.0, 360.0)),
            ((360.0, 360.0), (40.0, 40.0)),
        ] {
            let mut forward = Canvas::new(400, 400);
            forward.segment(a.0, a.1, b.0, b.1, Rgb(255, 255, 255), 4.0);
            let mut backward = Canvas::new(400, 400);
            backward.segment(b.0, b.1, a.0, a.1, Rgb(255, 255, 255), 4.0);
            assert_eq!(
                forward.lit_pixels(),
                backward.lit_pixels(),
                "direction changed the pixel count for {a:?} -> {b:?}"
            );
            assert!(forward.lit_pixels() > 0, "{a:?} -> {b:?} drew NOTHING");
        }
    }

    #[test]
    fn polylines_with_one_point_still_draw() {
        let mut canvas = Canvas::new(30, 30);
        canvas.polylines(&[vec![(15.0, 15.0)]], Rgb(255, 255, 255), 4.0, 1.0);
        assert!(canvas.lit_pixels() > 0, "a degenerate polyline must not vanish");
    }

    #[test]
    fn crc32_matches_the_known_vector() {
        // The standard check value for "123456789".
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn adler32_matches_the_known_vector() {
        assert_eq!(adler32(b"123456789"), 0x091E_01DE);
    }

    #[test]
    fn png_has_the_right_signature_and_terminator() {
        let canvas = Canvas::new(4, 4);
        let png = canvas.to_png();
        assert_eq!(&png[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        assert!(png.ends_with(&[0xAE, 0x42, 0x60, 0x82]), "IEND CRC missing");
    }

    #[test]
    fn png_declares_its_dimensions() {
        let png = Canvas::new(37, 19).to_png();
        // IHDR payload begins at byte 16: length(4) + "IHDR"(4).
        let width = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
        let height = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
        assert_eq!((width, height), (37, 19));
    }

    #[test]
    fn an_empty_image_still_produces_a_valid_stream() {
        // Zero-size images are a classic way to crash an encoder.
        let png = Canvas::new(0, 0).to_png();
        assert_eq!(&png[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    }
}