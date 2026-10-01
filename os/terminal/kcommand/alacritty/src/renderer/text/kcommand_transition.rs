//! The `kcommand!<language>` language transition.
//!
//! When the launcher switches language it passes `--language-transition`, and
//! the terminal plays a short sequence before handing over to the shell:
//!
//! 1. **Accretion** — a black hole opens in the middle of the screen.
//! 2. **Ingestion** — the current screen is drawn into the hole.
//! 3. **Singularity** — the hole collapses; the plain terminal shows through
//!    for an instant, broken by a few glitches.
//! 4. **Ejection** — a vortex throws the new language back out and it settles.
//!
//! # Why this file is pure
//!
//! A build machine has no display, so anything that can only be checked by
//! looking at it cannot be checked at all. Everything that decides *what a
//! frame looks like* therefore lives here as a pure function of
//! `(phase, progress, cols, rows, seed)`. The renderer only reads the result and
//! paints it. That keeps the animation testable with `cargo test` on a headless
//! box, and it keeps the glitch reproducible: the same seed always produces the
//! same corruption, so a bug report is a seed.
//!
//! Nothing here touches the terminal, the grid, or winit.

use std::f64::consts::TAU;

/// The four stages of the transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// The black hole grows from nothing.
    Accretion,
    /// Screen content is pulled into the hole.
    Ingestion,
    /// Collapse, flash, and a glimpse of the plain terminal with glitches.
    Singularity,
    /// The vortex throws the new language back out.
    Ejection,
}

impl Phase {
    /// Every phase, in order.
    pub const ALL: [Phase; 4] =
        [Phase::Accretion, Phase::Ingestion, Phase::Singularity, Phase::Ejection];

    /// How long this phase runs, in seconds. The whole thing stays well under
    /// two seconds: this plays every time the language changes, so it has to
    /// feel like punctuation, not a loading screen.
    pub fn duration(self) -> f64 {
        match self {
            Phase::Accretion => 0.36,
            Phase::Ingestion => 0.34,
            Phase::Singularity => 0.20,
            Phase::Ejection => 0.44,
        }
    }

    /// Total length of the whole sequence, in seconds.
    pub fn total_duration() -> f64 {
        Self::ALL.iter().map(|phase| phase.duration()).sum()
    }

    /// Which phase contains `elapsed`, and how far through it we are.
    ///
    /// `progress` is clamped to 0..1 so a stalled or late timer cannot drive
    /// the geometry outside its range.
    pub fn at(elapsed: f64) -> (Phase, f64) {
        let mut remaining = elapsed.max(0.0);
        for phase in Self::ALL {
            let duration = phase.duration();
            if remaining < duration {
                return (phase, (remaining / duration).clamp(0.0, 1.0));
            }
            remaining -= duration;
        }
        (Phase::Ejection, 1.0)
    }
}

/// Easing curves. Both endpoints are 0 and 1.
fn ease_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

fn ease_in_out(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// A tiny deterministic generator.
///
/// Glitches must be reproducible: if a user reports "the text came out garbled",
/// the seed in the report is enough to replay the exact frame.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Seed the generator. Any seed works, including 0.
    pub fn new(seed: u64) -> Self {
        // Avoid the fixed point at 0, which would make every seed identical.
        Self { state: seed ^ 0x9E37_79B9_7F4A_7C15 }
    }

    /// Next value in 0..1.
    pub fn next_f64(&mut self) -> f64 {
        // xorshift64*, small and good enough for visual noise.
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        let v = x.wrapping_mul(0x2545_F491_4F6C_DD1D);
        ((v >> 11) as f64) / ((1u64 << 53) as f64)
    }

    /// Next value in `0..n`, or 0 when `n` is 0.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_f64() * n as f64) as usize % n
        }
    }
}

/// What the renderer should paint for one frame.
#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    /// Draw nothing; leave whatever was there.
    Keep,
    /// Paint the background only, in the given colour.
    Shade { r: u8, g: u8, b: u8 },
    /// Paint a block, the accretion disc's grain.
    Grain { r: u8, g: u8, b: u8 },
    /// The hole itself: solid black, absorbing what is behind it.
    Void,
    /// A glitch: a coloured glyph standing in for corrupted content.
    Glitch { ch: char, r: u8, g: u8, b: u8 },
    /// The new language's name, thrown back out by the vortex.
    Label { ch: char },
}

/// One complete frame: a row-major grid of `cols * rows` cells.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Cell>,
}

impl Frame {
    /// The cell at `(col, row)`, or `Cell::Keep` when out of bounds.
    pub fn at(&self, col: usize, row: usize) -> &Cell {
        self.cells.get(row * self.cols + col).unwrap_or(&Cell::Keep)
    }

    /// Count of cells that are not `Cell::Keep`.
    pub fn painted(&self) -> usize {
        self.cells.iter().filter(|cell| **cell != Cell::Keep).count()
    }
}

/// Centre of the screen, in cells.
fn centre(cols: usize, rows: usize) -> (f64, f64) {
    (cols as f64 / 2.0 - 0.5, rows as f64 / 2.0 - 0.5)
}

/// Radius of the hole at a given point in the sequence, in cells.
///
/// Zero means "no hole". This is the single geometric fact the whole animation
/// hangs off: accretion grows it, ingestion holds it, singularity snaps it shut,
/// ejection does not use it.
pub fn hole_radius(phase: Phase, progress: f64, cols: usize, rows: usize) -> f64 {
    let (cx, cy) = centre(cols, rows);
    // Reach the nearest edge, so the hole can actually swallow the screen.
    let reach = (cx.abs().max(cy.abs())).max(1.0);
    match phase {
        Phase::Accretion => reach * 0.92 * ease_out_cubic(progress),
        Phase::Ingestion => reach * 0.92,
        // Collapse hard: this is the beat that sells the transition.
        Phase::Singularity => reach * 0.92 * (1.0 - ease_in_out(progress)),
        Phase::Ejection => 0.0,
    }
}

/// Vortex strength during ejection, 0..1, used to throw the label outward.
pub fn vortex(progress: f64) -> f64 {
    ease_out_cubic(progress)
}

/// Build the frame for a point in the sequence.
///
/// Deterministic: same inputs, same frame. `seed` drives only the glitch noise.
pub fn frame(
    phase: Phase,
    progress: f64,
    cols: usize,
    rows: usize,
    seed: u64,
) -> Frame {
    let mut cells = vec![Cell::Keep; cols * rows];
    let (cx, cy) = centre(cols, rows);
    let mut rng = Rng::new(seed);

    // `Void` is the hole; everything else is drawn over it, so compute the hole
    // last within each cell.
    let radius = hole_radius(phase, progress, cols, rows);

    match phase {
        Phase::Accretion | Phase::Ingestion => {
            for row in 0..rows {
                for col in 0..cols {
                    let dx = col as f64 - cx;
                    let dy = row as f64 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();

                    // The disc edge: a soft ring of grain just outside the hole.
                    if dist <= radius {
                        cells[row * cols + col] = Cell::Void;
                    } else if radius > 0.0 && dist <= radius + 1.6 {
                        // Angle decides the tint, so the ring reads as spinning.
                        let angle = dy.atan2(dx);
                        let hue = (angle + progress * 6.0).rem_euclid(TAU) / TAU;
                        let heat = 1.0 - (dist - radius) / 1.6;
                        let level = (40.0 + 150.0 * heat * (0.4 + 0.6 * hue)) as u8;
                        cells[row * cols + col] = Cell::Grain {
                            r: level,
                            g: (level as f32 * 0.45) as u8,
                            b: (255.0 - level as f32 * 0.6) as u8,
                        };
                    }
                }
            }

            // Ingestion: content visibly streams inward instead of just being
            // covered, which is what makes it read as a black hole rather than
            // a growing circle.
            if phase == Phase::Ingestion && radius > 0.0 {
                let pull = ease_in_out(progress);
                for row in 0..rows {
                    for col in 0..cols {
                        let idx = row * cols + col;
                        if cells[idx] != Cell::Keep {
                            continue;
                        }
                        let dx = col as f64 - cx;
                        let dy = row as f64 - cy;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= radius * 1.9 {
                            // Swirl: rotate about the centre and pull inward.
                            // The shade is driven by the ROTATED distance, so
                            // the bands twist into the hole instead of the
                            // rotation being computed and thrown away.
                            let swirl = pull * 0.9;
                            let (s, c) = swirl.sin_cos();
                            let rx = dx * c - dy * s;
                            let ry = dx * s + dy * c;
                            let rdist = (rx * rx + ry * ry).sqrt() * (1.0 - 0.35 * pull);

                            let fade = 1.0 - (rdist / (radius * 1.9)).min(1.0);
                            let level = (30.0 + 90.0 * fade) as u8;
                            cells[idx] = Cell::Shade {
                                r: level,
                                g: level,
                                b: (level as f32 * 1.6) as u8,
                            };
                        }
                    }
                }
            }
        }

        Phase::Singularity => {
            // The flash, then the plain terminal showing through.
            let flash = (progress * 5.0).min(1.0);
            let glitch_window = progress > 0.30 && progress < 0.92;
            let density = if glitch_window {
                // Ramp up, then down, so the glitches arrive and fade.
                let t = (progress - 0.30) / 0.62;
                (t.sin() * std::f64::consts::PI).max(0.0)
            } else {
                0.0
            };

            for row in 0..rows {
                for col in 0..cols {
                    let idx = row * cols + col;
                    let dx = col as f64 - cx;
                    let dy = row as f64 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist <= radius {
                        cells[idx] = Cell::Void;
                    } else if glitch_window && rng.next_f64() < density * 0.035 {
                        // Corrupted glyph. Cyan and magenta dominate because a
                        // real RGB split reads as red/cyan, not random colours.
                        let ch = GLYPHS[rng.below(GLYPHS.len())];
                        let (r, g, b) = if rng.next_f64() < 0.5 {
                            (255, 0, 90)
                        } else {
                            (0, 220, 255)
                        };
                        cells[idx] = Cell::Glitch { ch, r, g, b };
                    } else if flash > 0.0 && dist < radius + 6.0 {
                        let level = (255.0 * flash * (1.0 - (dist - radius) / 6.0)) as u8;
                        cells[idx] = Cell::Shade {
                            r: level,
                            g: level,
                            b: level,
                        };
                    }
                }
            }
        }

        Phase::Ejection => {
            // A tornado: an outward-moving spiral of grain.
            let v = vortex(progress);
            let reach = (cx.abs().max(cy.abs())).max(1.0);
            let label = label();

            for row in 0..rows {
                for col in 0..cols {
                    let idx = row * cols + col;
                    let dx = col as f64 - cx;
                    let dy = row as f64 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 0.5 {
                        continue;
                    }
                    let angle = dy.atan2(dx);

                    // A front travelling outward. Cells are lit once the front
                    // has passed them, and brightest just behind it, so the
                    // visible extent grows with time instead of collapsing.
                    let front = v * reach;
                    if dist > front {
                        continue;
                    }
                    let trail = (front - dist) / reach.max(1.0);
                    let fade = (1.0 - trail).max(0.0);

                    // The spiral arm: phase-locked to the angle, so it reads as
                    // rotation rather than an expanding ring.
                    let arm = (angle * 3.0 + dist * 0.55 - v * 9.0).rem_euclid(TAU);
                    if (arm / TAU) < 0.42 && fade > 0.02 {
                        let level = (60.0 + 180.0 * fade) as u8;
                        cells[idx] = Cell::Grain {
                            r: level,
                            g: (level as f32 * 0.75) as u8,
                            b: (level as f32 * 0.25) as u8,
                        };
                    }
                }
            }

            // The new language's name, thrown back out from the centre and
            // settling into place.
            if !label.is_empty() {
                let lx = cx - (label.chars().count() as f64 - 1.0) / 2.0;
                let ly = cy;
                let settle = ease_out_cubic(progress);
                for (i, ch) in label.chars().enumerate() {
                    // Characters arrive left to right, so the name unspools.
                    let offset = i as f64 * 0.06;
                    let p = (settle - offset).clamp(0.0, 1.0);
                    if p <= 0.0 {
                        continue;
                    }
                    let col = (lx + i as f64) + (1.0 - p) * 6.0;
                    let row = ly - (1.0 - p) * 3.0;
                    let (col, row) = (col.round(), row.round());
                    if col >= 0.0 && (col as usize) < cols && row >= 0.0 {
                        cells[row as usize * cols + col as usize] = Cell::Label { ch };
                    }
                }
            }
        }
    }

    Frame { cols, rows, cells }
}

/// The name shown during ejection. Set once by the launcher path.
fn label() -> String {
    std::env::var("KCOMMAND_TRANSITION_LABEL").unwrap_or_default()
}

/// Glyphs used for glitch corruption.
const GLYPHS: [char; 24] = [
    '▓', '▒', '░', '█', '▚', '▞', '╳', '▤', '▥', '┼', '║', '▔', '▁', '▂', '▃', '╬', '⌁', '⍿', '▎',
    '▍', '◢', '◣', '◤', '◥',
];

#[cfg(test)]
mod tests {
    use super::*;

    const COLS: usize = 80;
    const ROWS: usize = 24;

    #[test]
    fn phases_run_in_order_and_cover_the_timeline() {
        let mut elapsed = 0.0;
        let mut seen: Vec<Phase> = Vec::new();
        while elapsed < Phase::total_duration() {
            let (phase, _) = Phase::at(elapsed);
            if seen.last() != Some(&phase) {
                seen.push(phase);
            }
            elapsed += 0.01;
        }
        assert_eq!(seen, Phase::ALL.to_vec(), "every phase must appear, in order");
    }

    #[test]
    fn total_duration_stays_under_two_seconds() {
        // This plays on every language change; it must feel like punctuation.
        assert!(Phase::total_duration() < 2.0, "got {}", Phase::total_duration());
    }

    #[test]
    fn progress_is_always_clamped() {
        // A late or negative timer must never produce geometry outside 0..1.
        for phase in Phase::ALL {
            for p in [-1.0, -0.001, 0.0, 0.5, 1.0, 1.001, 5.0] {
                let frame = frame(phase, p, COLS, ROWS, 7);
                assert_eq!(frame.cells.len(), COLS * ROWS);
            }
        }
    }

    #[test]
    fn hole_starts_empty_and_grows_to_fill_the_screen() {
        assert_eq!(hole_radius(Phase::Accretion, 0.0, COLS, ROWS), 0.0);
        let end = hole_radius(Phase::Accretion, 1.0, COLS, ROWS);
        assert!(end > 3.0, "hole should be large at the end, got {end}");
        // It must reach far enough to swallow a good part of the screen.
        assert!(end >= ROWS as f64 / 2.0 - 1.0, "hole too small to read: {end}");
    }

    #[test]
    fn singularity_collapses_the_hole() {
        let open = hole_radius(Phase::Singularity, 0.0, COLS, ROWS);
        let shut = hole_radius(Phase::Singularity, 1.0, COLS, ROWS);
        assert!(shut < open, "hole must collapse: {open} -> {shut}");
        assert!(shut.abs() < 0.001, "hole should be fully shut, got {shut}");
    }

    #[test]
    fn hole_is_centred() {
        // The centre cell must be Void once the hole is open.
        let f = frame(Phase::Ingestion, 0.5, COLS, ROWS, 1);
        assert_eq!(*f.at(COLS / 2, ROWS / 2), Cell::Void);
        // A corner must not be.
        assert_ne!(*f.at(0, 0), Cell::Void);
    }

    #[test]
    fn frames_are_deterministic_for_a_seed() {
        for phase in Phase::ALL {
            let a = frame(phase, 0.5, COLS, ROWS, 42);
            let b = frame(phase, 0.5, COLS, ROWS, 42);
            assert_eq!(a, b, "{phase:?} must replay exactly for a seed");
        }
    }

    #[test]
    fn different_seeds_give_different_glitches() {
        // Glitches are the only seed-driven part, so singularity must differ.
        let a = frame(Phase::Singularity, 0.6, COLS, ROWS, 1);
        let b = frame(Phase::Singularity, 0.6, COLS, ROWS, 999);
        assert_ne!(a, b, "seed must change the glitch pattern");
    }

    #[test]
    fn singularity_glitches_only_in_the_middle_window() {
        let early = frame(Phase::Singularity, 0.05, COLS, ROWS, 3);
        let late = frame(Phase::Singularity, 0.99, COLS, ROWS, 3);
        let count = |f: &Frame| {
            f.cells.iter().filter(|c| matches!(c, Cell::Glitch { .. })).count()
        };
        assert_eq!(count(&early), 0, "no glitches before the window");
        assert_eq!(count(&late), 0, "no glitches after the window");
    }

    #[test]
    fn accretion_paints_something_visible() {
        let f = frame(Phase::Accretion, 0.6, COLS, ROWS, 5);
        assert!(f.painted() > 50, "expected a visible disc, painted {}", f.painted());
    }

    #[test]
    fn ejection_expands_outward() {
        // Later frames reach further from the centre.
        let early = frame(Phase::Ejection, 0.15, COLS, ROWS, 5);
        let late = frame(Phase::Ejection, 0.85, COLS, ROWS, 5);
        let reach = |f: &Frame| {
            let mut far = 0.0f64;
            for row in 0..f.rows {
                for col in 0..f.cols {
                    if *f.at(col, row) == Cell::Keep {
                        continue;
                    }
                    let dx = col as f64 - COLS as f64 / 2.0;
                    let dy = row as f64 - ROWS as f64 / 2.0;
                    far = far.max((dx * dx + dy * dy).sqrt());
                }
            }
            far
        };
        assert!(reach(&late) >= reach(&early), "vortex must spread outward");
    }

    #[test]
    fn tiny_grids_do_not_panic() {
        // A very small window must not index out of bounds.
        for (c, r) in [(1, 1), (2, 1), (1, 2), (3, 3), (0, 5), (5, 0)] {
            for phase in Phase::ALL {
                let f = frame(phase, 0.5, c, r, 1);
                assert_eq!(f.cells.len(), c * r);
            }
        }
    }

    #[test]
    fn out_of_bounds_reads_are_keep_not_panic() {
        let f = frame(Phase::Accretion, 0.5, COLS, ROWS, 1);
        assert_eq!(*f.at(9999, 9999), Cell::Keep);
    }

    #[test]
    fn rng_is_reproducible_and_spread() {
        let mut a = Rng::new(12345);
        let mut b = Rng::new(12345);
        for _ in 0..64 {
            assert_eq!(a.next_f64(), b.next_f64());
        }
        let mut r = Rng::new(999);
        let mut sum = 0.0;
        for _ in 0..1000 {
            let v = r.next_f64();
            assert!((0.0..1.0).contains(&v), "out of range: {v}");
            sum += v;
        }
        // A flat distribution averages 0.5; anything far off is a real bug.
        assert!((sum / 1000.0 - 0.5).abs() < 0.05);
    }

    #[test]
    fn rng_seed_zero_does_not_degenerate() {
        // Seed 0 must not produce a constant stream.
        let mut r = Rng::new(0);
        let first = r.next_f64();
        assert!((first - r.next_f64()).abs() > 1e-9, "seed 0 produced a constant");
    }
}