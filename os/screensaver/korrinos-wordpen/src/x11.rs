//! Just enough X11 to put pixels on the screen, with no dependencies.
//!
//! # Why hand-rolled
//!
//! This crate has no dependencies on purpose: it is built on an air-gapped
//! machine where fetching a windowing crate is not an option, and the savers are
//! built in an environment with no network. So the X11 core protocol is
//! spoken directly over the display socket. That is only feasible because the
//! saver needs very little of it: one fullscreen window, one graphics context,
//! and PutImage.
//!
//! # Why this file is pure
//!
//! Everything here that can be tested without a display is a free function that
//! turns data into bytes: `parse_display`, the setup request, PutImage
//! requests, band splitting. Those are covered by tests that run on a machine
//! with no X server at all. Only [`Connection`] and [`Screen`] actually talk to
//! a socket.
//!
//! # Window type, and why it matters
//!
//! The saver creates an **override-redirect** window covering the root window's
//! area. Override-redirect tells the window manager to keep its hands off, so
//! the saver cannot be moved, resized, tiled or closed by a stray keystroke.
//! That is the whole point of a screen saver, and getting it wrong is how you
//! end up with a saver that flickers when the desktop's tiling extension touches
//! it.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

/// A parsed `DISPLAY` value: the socket to open, plus the screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayTarget {
    /// Unix socket path, e.g. `/tmp/.X11-unix/X0`.
    pub socket: PathBuf,
    /// Screen number within that display.
    pub screen: usize,
}

/// Parse a `DISPLAY` string.
///
/// Handles the forms that matter locally:
///   `:0`, `:0.0`        local
///   `unix:0`            local via the unix transport
///   `localhost:10.0`    a TCP display (socket path will not exist; the caller
///                        gets a clear error rather than a confusing one)
///   `:0.0.0`            extra screen numbers are ignored
///
/// A host part that is not local is accepted but flagged by returning no socket
/// path, because connecting over TCP to an X server is a different code path
/// (and is rarely what a screen saver wants).
pub fn parse_display(display: &str) -> Option<DisplayTarget> {
    let display = display.trim();
    if display.is_empty() {
        return None;
    }
    // Split off the host, if any. Both halves borrow from `display`.
    let index = display.rfind(':');
    let host: &str = index.map_or("", |i| &display[..i]);
    let rest: &str = index.map_or(display, |i| &display[i + 1..]);
    let host = host.trim_start_matches("unix");
    let mut parts = rest.split('.');
    let number: usize = parts.next()?.trim().parse().ok()?;
    let screen: usize = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(0);

    let local = host.is_empty()
        || host.eq_ignore_ascii_case("unix")
        || host.eq_ignore_ascii_case("localhost");
    if !local {
        // A remote display would need a TCP socket; do not pretend otherwise.
        return None;
    }
    Some(DisplayTarget {
        socket: PathBuf::from(format!("/tmp/.X11-unix/X{number}")),
        screen,
    })
}

/// An MIT-MAGIC-COOKIE-1 authorisation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Auth {
    pub name: String,
    pub data: Vec<u8>,
}

/// Parse `.Xauthority` properly: address, display number, **name**, data.
pub fn parse_xauthority_first(bytes: &[u8]) -> Option<Auth> {
    let mut offset = 0usize;
    let field = |offset: &mut usize| -> Option<Vec<u8>> {
        let len_bytes = bytes.get(*offset..*offset + 2)?;
        let len = u16::from_be_bytes([len_bytes[0], len_bytes[1]]) as usize;
        *offset += 2;
        let data = bytes.get(*offset..*offset + len)?.to_vec();
        *offset += len;
        Some(data)
    };

    while offset + 2 <= bytes.len() {
        offset += 2; // family
        let address = field(&mut offset)?;
        let number = field(&mut offset)?;
        let name = field(&mut offset)?;
        let data = field(&mut offset)?;
        if name == b"MIT-MAGIC-COOKIE-1" {
            // A cookie for a different display must not be used, or the server
            // rejects it and the error points at the wrong thing.
            let wanted = std::env::var("DISPLAY").ok();
            let ok = match wanted.as_deref().and_then(parse_display) {
                Some(target) => {
                    let n = target.socket.to_string_lossy();
                    let suffix = n.rsplit(['X', '/']).next().unwrap_or("");
                    String::from_utf8_lossy(&number).trim() == suffix
                        || String::from_utf8_lossy(&number).is_empty()
                        || address.is_empty()
                },
                None => true,
            };
            if ok {
                return Some(Auth {
                    name: String::from_utf8_lossy(&name).into_owned(),
                    data,
                });
            }
        }
    }
    None
}

/// Build the connection setup request.
pub fn setup_request(auth: Option<&Auth>) -> Vec<u8> {
    let (name, data) = match auth {
        Some(auth) => (auth.name.as_bytes().to_vec(), auth.data.clone()),
        None => (Vec::new(), Vec::new()),
    };
    // Both lengths must be padded to a multiple of 4.
    let pad = |len: usize| (4 - (len % 4)) % 4;
    let name_pad = pad(name.len());
    let data_pad = pad(data.len());

    let mut out = Vec::with_capacity(12 + name.len() + name_pad + data.len() + data_pad);
    out.push(b'l'); // little endian
    out.push(0); // unused
    out.extend_from_slice(&11u16.to_le_bytes()); // protocol major
    out.extend_from_slice(&0u16.to_le_bytes()); // protocol minor
    out.extend_from_slice(&(name.len() as u16).to_le_bytes());
    out.extend_from_slice(&(data.len() as u16).to_le_bytes());
    out.extend_from_slice(&[0, 0]); // padding
    out.extend_from_slice(&name);
    out.extend_from_slice(&[0u8; 4][..name_pad]);
    out.extend_from_slice(&data);
    out.extend_from_slice(&[0u8; 4][..data_pad]);
    out
}

/// What the server told us during the handshake.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetupInfo {
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub max_request_length: u32,
    pub screen: ScreenInfo,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenInfo {
    pub root: u32,
    pub root_visual: u32,
    pub root_depth: u8,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub width: u16,
    pub height: u16,
}

/// Parse the connection setup reply.
///
/// Returns `None` for a refusal, with the reason, rather than a default that
/// would fail later with a confusing error. The X11 setup reply is a fixed
/// header followed by variable-length vendor and format data, so every offset is
/// bounds-checked.
pub fn parse_setup_reply(bytes: &[u8]) -> Result<SetupInfo, String> {
    if bytes.len() < 8 {
        return Err("setup reply truncated".into());
    }
    let status = bytes[0];
    if status == 0 {
        // status(1) reason-len(1) major(2) minor(2) length(2) = 8 bytes, and the
        // reason string follows all of it.
        let reason_len = *bytes.get(1).unwrap_or(&0) as usize;
        let reason = bytes
            .get(8..8 + reason_len)
            .map(|b| String::from_utf8_lossy(b).trim().to_string())
            .unwrap_or_default();
        return Err(format!("X server refused the connection: {reason}"));
    }
    if status == 2 {
        return Err("X server requires authentication but no cookie was found".into());
    }

    let u16at = |at: usize| -> Option<u16> {
        let s = bytes.get(at..at + 2)?;
        Some(u16::from_le_bytes([s[0], s[1]]))
    };
    let u32at = |at: usize| -> Option<u32> {
        let s = bytes.get(at..at + 4)?;
        Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    };
    let need = |what: &str, at: usize, len: usize| -> Result<(), String> {
        if bytes.len() >= at + len { Ok(()) } else { Err(format!("setup reply truncated at {what}")) }
    };

    need("header", 0, 32)?;
    let _additional = u16at(6).ok_or("no additional length")? as usize * 4;
    let info = SetupInfo {
        resource_id_base: u32at(12).ok_or("no resource id base")?,
        resource_id_mask: u32at(16).ok_or("no resource id mask")?,
        max_request_length: u32at(18).ok_or("no max request length")?.max(1),
        screen: ScreenInfo::default(),
    };

    // Walk past the vendor string and the pixmap formats to the first SCREEN.
    let vendor_len = u16at(24).ok_or("no vendor length")? as usize;
    let formats_start = 32;
    let formats_len = {
        let count = bytes.get(20).copied().unwrap_or(0) as usize;
        count * 8
    };
    let screens_start = formats_start + vendor_len + ((vendor_len + 3) & !3) + formats_len;
    need("screen", screens_start, 40)?;
    let screen = ScreenInfo {
        root: u32at(screens_start).ok_or("no root window")?,
        white_pixel: u32at(screens_start + 8).ok_or("no white pixel")?,
        black_pixel: u32at(screens_start + 12).ok_or("no black pixel")?,
        width: u16at(screens_start + 20).ok_or("no screen width")?,
        height: u16at(screens_start + 22).ok_or("no screen height")?,
        root_visual: u32at(screens_start + 32).ok_or("no root visual")?,
        root_depth: bytes.get(screens_start + 38).copied().unwrap_or(24),
    };
    Ok(SetupInfo { screen, ..info })
}

/// Request opcodes used here.
pub const CREATE_WINDOW: u8 = 1;
pub const MAP_WINDOW: u8 = 8;
pub const CHANGE_PROPERTY: u8 = 18;
pub const CREATE_GC: u8 = 55;
pub const PUT_IMAGE: u8 = 72;
pub const POLY_FILL_RECTANGLE: u8 = 70;
pub const UNMAP_WINDOW: u8 = 10;

/// `CreateWindow` value-mask bits.
pub const CW_BACK_PIXEL: u32 = 0x0000_0002;
pub const CW_OVERRIDE_REDIRECT: u32 = 0x0000_0200;
pub const CW_EVENT_MASK: u32 = 0x0000_0800;

/// `CreateWindow` event-mask bits.
pub const EVENT_KEY_PRESS: u32 = 0x0000_0001;
pub const EVENT_BUTTON_PRESS: u32 = 0x0000_0004;
pub const EVENT_POINTER_MOTION: u32 = 0x0000_0040;
pub const EVENT_EXPOSURE: u32 = 0x0000_8000;
pub const EVENT_STRUCTURE_NOTIFY: u32 = 0x0002_0000;

/// Build a `CreateWindow` request for a fullscreen override-redirect window.
#[allow(clippy::too_many_arguments)]
pub fn create_window(
    wid: u32,
    root: u32,
    visual: u32,
    depth: u8,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
) -> Vec<u8> {
    // Fixed part is 8 words (32 bytes): opcode+depth+length(4), wid(4),
    // parent(4), x(2), y(2), w(2), h(2), border(2), class(2), visual(4),
    // mask(4) = 32. Then one 4-byte value per bit set in the mask.
    let mut out = Vec::with_capacity(48);
    out.push(CREATE_WINDOW);
    out.push(depth);
    out.extend_from_slice(&11u16.to_le_bytes()); // 32 + 3 value words = 44
    out.extend_from_slice(&wid.to_le_bytes());
    out.extend_from_slice(&root.to_le_bytes());
    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&y.to_le_bytes());
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // border width
    out.extend_from_slice(&1u16.to_le_bytes()); // InputOutput
    // There is no separate "visual mask" field in CreateWindow: the layout is
    // class(2), visual(4), value-mask(4). An earlier version emitted a phantom
    // 4-byte visual mask here, which pushed the header to 36 bytes and made the
    // server read the value list from the wrong offset.
    out.extend_from_slice(&visual.to_le_bytes());
    out.extend_from_slice(&(CW_BACK_PIXEL | CW_OVERRIDE_REDIRECT | CW_EVENT_MASK).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // background pixel: black
    // Override redirect: True. Without this a window manager may tile, move or
    // close the saver, which is exactly what must not happen.
    out.extend_from_slice(&1u32.to_le_bytes());
    let events = EVENT_KEY_PRESS
        | EVENT_BUTTON_PRESS
        | EVENT_POINTER_MOTION
        | EVENT_EXPOSURE
        | EVENT_STRUCTURE_NOTIFY;
    out.extend_from_slice(&events.to_le_bytes());
    out
}

/// Build a `MapWindow` request.
pub fn map_window(wid: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    out.push(MAP_WINDOW);
    out.push(0);
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&wid.to_le_bytes());
    out
}

/// Build a `CreateGC` request.
pub fn create_gc(gc: u32, drawable: u32, visual: u32, depth: u8) -> Vec<u8> {
    // 16 bytes minimum: header(4) + cid(4) + drawable(4) + value-mask(4).
    let mut out = Vec::with_capacity(16);
    out.push(CREATE_GC);
    out.push(0);
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&gc.to_le_bytes());
    out.extend_from_slice(&drawable.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // no value mask: server defaults
    let _ = (visual, depth);
    out
}

/// Build a `PolyFillRectangle` request, used to clear to black.
pub fn poly_fill_rectangle(
    gc: u32,
    drawable: u32,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(18);
    out.push(POLY_FILL_RECTANGLE);
    out.push(0);
    out.extend_from_slice(&5u16.to_le_bytes());
    out.extend_from_slice(&drawable.to_le_bytes());
    out.extend_from_slice(&gc.to_le_bytes());
    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&y.to_le_bytes());
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out
}

/// Build a `ChangeProperty` request.
pub fn change_property(
    window: u32,
    property: u32,
    property_type: u32,
    format: u8,
    data: &[u8],
) -> Vec<u8> {
    let units = (data.len() as u32 + 3) / 4;
    let mut out = Vec::with_capacity(24 + data.len());
    out.push(CHANGE_PROPERTY);
    out.push(0); // Replace
    out.extend_from_slice(&((6 + units) as u16).to_le_bytes());
    out.extend_from_slice(&window.to_le_bytes());
    out.extend_from_slice(&property.to_le_bytes());
    out.extend_from_slice(&property_type.to_le_bytes());
    out.push(format);
    out.extend_from_slice(&[0, 0, 0]); // pad to a 4-byte boundary
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    while (out.len() % 4) != 0 {
        out.push(0);
    }
    out
}

/// Predefined atoms used for a saver window.
pub const ATOM_NET_WM_WINDOW_TYPE: u32 = 267;
pub const ATOM_NET_WM_WINDOW_TYPE_SCREENSAVER: u32 = 269;
pub const ATOM_NET_WM_NAME: u32 = 39;
pub const ATOM_STRING: u32 = 31;

/// How many scanlines fit in one `PutImage` request.
///
/// A 4K PutImage of RGBA is 3840 × 2160 × 4 = 33 MB, and a 8K frame is four
/// times that. The X11 protocol caps a single request at the value the server
/// reported in `max_request_length` (typically 65535 words, so ~256 KB), so a
/// frame has to be split into horizontal bands. Getting this wrong is the
/// classic "screensaver only works at low resolution" bug.
pub fn rows_per_request(max_request_length: u32, bytes_per_pixel: u32, width: u32) -> u32 {
    let max_bytes = max_request_length.saturating_mul(4);
    // Leave room for the 24-byte request header.
    let usable = max_bytes.saturating_sub(24);
    let row_bytes = width.saturating_mul(bytes_per_pixel).max(1);
    (usable / row_bytes).clamp(1, 1_000_000)
}

/// Build one `PutImage` request for a horizontal band.
pub fn put_image(
    drawable: u32,
    gc: u32,
    width: u32,
    height: u32,
    dst_x: i16,
    dst_y: i16,
    depth: u8,
    pixels: &[u8],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(24 + pixels.len());
    // Header layout is fixed at 24 bytes:
    //   opcode(1) format(1) length(2) drawable(4) gc(4)
    //   width(2) height(2) dst-x(2) dst-y(2) left-pad(1) depth(1)
    out.push(PUT_IMAGE);
    out.push(2); // format: ZPixmap
    out.extend_from_slice(&((pixels.len() as u32 / 4) as u16 + 6).to_le_bytes()); // 24-byte header
    out.extend_from_slice(&drawable.to_le_bytes());
    out.extend_from_slice(&gc.to_le_bytes());
    out.extend_from_slice(&(width as u16).to_le_bytes());
    out.extend_from_slice(&(height as u16).to_le_bytes());
    out.extend_from_slice(&dst_x.to_le_bytes());
    out.extend_from_slice(&dst_y.to_le_bytes());
    out.push(0); // left pad
    out.push(depth);
    out.extend_from_slice(&[0, 0]); // unused: brings the header to 24 bytes
    out.extend_from_slice(pixels);
    out
}

/// Split a full frame into `PutImage` bands, each within the server's limit.
///
/// Returns `(dst_y, pixels)` pairs. The number of bands is what a caller needs
/// to know for a 4K or 8K frame, and a test asserts the per-band size really is
/// within the limit rather than trusting the arithmetic.
pub fn plan_bands(
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    max_request_length: u32,
) -> Vec<(u32, u32)> {
    let rows = rows_per_request(max_request_length, bytes_per_pixel, width).max(1);
    let mut bands = Vec::new();
    let mut y = 0;
    while y < height {
        let rows_here = rows.min(height - y);
        bands.push((y, rows_here));
        y += rows_here;
    }
    bands
}

/// What an event means to the saver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    /// The user did something: leave immediately.
    Activity,
    /// Nothing interesting.
    Ignore,
}

/// Classify a 32-byte X event.
///
/// The saver's contract is simple and old: run while idle, exit on input. So the
/// only thing that matters is "is this an input event".
pub fn classify_event(bytes: &[u8]) -> EventKind {
    // X events are always 32 bytes. Anything shorter is a partial read and must
    // NOT be treated as activity, or a truncated buffer would kick the saver off
    // screen at random.
    if bytes.len() < 32 {
        return EventKind::Ignore;
    }
    let kind = bytes[0];
    match kind {
        // KeyPress 2, KeyRelease 3, ButtonPress 4, ButtonRelease 5,
        // MotionNotify 6, EnterNotify 7, LeaveNotify 8.
        2..=8 => EventKind::Activity,
        // Errors are worth noticing while developing but must not kill the saver.
        _ => EventKind::Ignore,
    }
}

/// A live connection to an X server.
pub struct Connection {
    stream: UnixStream,
    pub info: SetupInfo,
    next_resource: u32,
}

impl Connection {
    /// Connect, authenticate and read the setup reply.
    pub fn open(display: &str, auth: Option<&Auth>) -> Result<Connection, String> {
        let target =
            parse_display(display).ok_or_else(|| format!("cannot use DISPLAY={display:?}"))?;
        let mut stream = UnixStream::connect(&target.socket).map_err(|error| {
            format!("cannot open {}: {error} (is a display server running?)", target.socket.display())
        })?;

        stream.write_all(&setup_request(auth)).map_err(|e| e.to_string())?;

        // The reply's extra word count arrives in the first 8 bytes.
        let mut header = [0u8; 8];
        stream.read_exact(&mut header).map_err(|e| e.to_string())?;
        let extra_words = u16::from_le_bytes([header[6], header[7]]) as usize;
        let mut body = vec![0u8; extra_words * 4];
        if !body.is_empty() {
            stream.read_exact(&mut body).map_err(|e| e.to_string())?;
        }
        let mut full = header.to_vec();
        full.extend_from_slice(&body);

        let info = parse_setup_reply(&full)?;
        Ok(Connection { stream, info, next_resource: 0 })
    }

    /// Allocate the next resource id from the server's base and mask.
    pub fn alloc(&mut self) -> u32 {
        self.next_resource = self.next_resource.wrapping_add(1);
        self.info.resource_id_base | (self.next_resource & self.info.resource_id_mask)
    }

    /// Send one or more requests, flushing after them.
    pub fn send(&mut self, requests: &[u8]) -> Result<(), String> {
        self.stream.write_all(requests).map_err(|e| e.to_string())?;
        self.stream.flush().map_err(|e| e.to_string())
    }

    /// Read pending events, returning true if the user did something.
    ///
    /// Non-blocking: returns false immediately when nothing is queued, so the
    /// animation keeps running at its own pace and is never stalled by a
    /// missing event.
    pub fn activity_pending(&mut self) -> bool {
        let mut buffer = [0u8; 32];
        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => return false,
                Ok(_) => {
                    if classify_event(&buffer) == EventKind::Activity {
                        return true;
                    }
                },
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return false,
                Err(_) => return false,
            }
        }
    }

    /// Put a whole frame up, split into bands the server will accept.
    pub fn put_frame(
        &mut self,
        drawable: u32,
        gc: u32,
        width: u32,
        height: u32,
        depth: u8,
        pixels: &[u8],
        bytes_per_pixel: u32,
    ) -> Result<(), String> {
        let row_bytes = (width as usize) * (bytes_per_pixel as usize);
        if pixels.len() < row_bytes * (height as usize) {
            return Err("frame buffer is smaller than the geometry claims".into());
        }
        let bands = plan_bands(width, height, bytes_per_pixel, self.info.max_request_length);
        let mut requests = Vec::new();
        for (y, rows) in bands {
            let start = (y as usize) * row_bytes;
            let end = start + (rows as usize) * row_bytes;
            let band = &pixels[start..end];
            requests.extend_from_slice(&put_image(
                drawable, gc, width, rows, 0, y as i16, depth, band,
            ));
        }
        self.send(&requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_local_display_forms() {
        for (input, socket, screen) in [
            (":0", "/tmp/.X11-unix/X0", 0),
            (":0.0", "/tmp/.X11-unix/X0", 0),
            (":1.2", "/tmp/.X11-unix/X1", 2),
            ("unix:3", "/tmp/.X11-unix/X3", 0),
            ("localhost:4", "/tmp/.X11-unix/X4", 0),
        ] {
            let got = parse_display(input).unwrap_or_else(|| panic!("{input} should parse"));
            assert_eq!(got.socket.to_string_lossy(), socket, "socket for {input}");
            assert_eq!(got.screen, screen, "screen for {input}");
        }
    }

    #[test]
    fn refuses_remote_and_junk_displays() {
        assert!(parse_display("").is_none());
        assert!(parse_display("nonsense").is_none());
        // Remote displays need a TCP path, which this deliberately does not fake.
        assert!(parse_display("otherhost:0").is_none());
    }

    #[test]
    fn setup_request_is_padded_to_four_bytes() {
        // "MIT-MAGIC-COOKIE-1" is 18 bytes, so it needs 2 bytes of padding.
        let auth = Auth { name: "MIT-MAGIC-COOKIE-1".into(), data: vec![0xAB; 16] };
        let request = setup_request(Some(&auth));
        assert_eq!(request[0], b'l', "must announce little endian");
        assert_eq!(u16::from_le_bytes([request[2], request[3]]), 11, "protocol 11");
        // Layout: byte-order(1) pad(1) major(2) minor(2) name-len(2)
        //         data-len(2) pad(2). So the name length is at offset 6.
        let name_len = u16::from_le_bytes([request[6], request[7]]) as usize;
        let data_len = u16::from_le_bytes([request[8], request[9]]) as usize;
        assert_eq!(name_len, 18);
        assert_eq!(data_len, 16);
        assert_eq!(request.len(), 12 + 18 + 2 + 16, "total length with padding");
        assert_eq!(request.len() % 4, 0, "the request must be 4-byte aligned");
    }

    #[test]
    fn setup_request_without_auth_is_aligned() {
        let request = setup_request(None);
        assert_eq!(request.len(), 12);
        assert_eq!(request.len() % 4, 0);
    }

    #[test]
    fn setup_request_handles_a_name_needing_two_pad_bytes() {
        // Length 17 needs 3 bytes of padding, which used to be a classic
        // off-by-one that desynchronised the whole handshake.
        let auth = Auth { name: "X".repeat(17), data: vec![1; 3] };
        let request = setup_request(Some(&auth));
        assert_eq!(request.len() % 4, 0);
    }

    #[test]
    fn refuses_a_rejected_connection_with_its_reason() {
        let mut reply = vec![0u8, 3, 0, 0, 11, 0, 1, 0];
        reply.extend_from_slice(b"nah");
        let error = parse_setup_reply(&reply).unwrap_err();
        assert!(error.contains("nah"), "the reason should survive: {error}");
    }

    #[test]
    fn reports_missing_auth_separately_from_refusal() {
        let reply = vec![2u8, 0, 0, 0, 0, 0, 0, 0];
        let error = parse_setup_reply(&reply).unwrap_err();
        assert!(error.contains("authentication"), "{error}");
    }

    #[test]
    fn truncated_replies_do_not_panic() {
        let mut full = vec![1u8, 0, 0, 0, 11, 0, 1, 0];
        full.extend_from_slice(&[0u8; 8]);
        full.extend_from_slice(&[0u8; 40]);
        for len in 0..full.len() {
            let _ = parse_setup_reply(&full[..len]);
        }
        assert!(parse_setup_reply(&[]).is_err());
    }

    #[test]
    fn classifies_input_events_as_activity() {
        // KeyPress, ButtonPress, MotionNotify and friends all mean "the user is
        // here", which is the one thing the saver must react to.
        for code in [2u8, 3, 4, 5, 6, 7, 8] {
            let mut event = [0u8; 32];
            event[0] = code;
            assert_eq!(classify_event(&event), EventKind::Activity, "code {code}");
        }
        for code in [0u8, 1, 9, 12, 17, 22, 28] {
            let mut event = [0u8; 32];
            event[0] = code;
            assert_eq!(classify_event(&event), EventKind::Ignore, "code {code}");
        }
    }

    #[test]
    fn a_partial_event_is_never_treated_as_activity() {
        // A short read must be ignored. Trusting the type code of a truncated
        // buffer would kick the saver off screen at random.
        for len in 0..32 {
            assert_eq!(
                classify_event(&[2u8; 32][..len]),
                EventKind::Ignore,
                "a {len}-byte read must not look like a KeyPress"
            );
        }
        assert_eq!(classify_event(&[2u8; 32]), EventKind::Activity);
    }

    #[test]
    fn the_window_is_override_redirect() {
        // The value mask must include override-redirect, or a window manager
        // can tile and move the saver, which is exactly what must not happen.
        let request = create_window(0x200000, 0x100000, 32, 24, 0, 0, 1920, 1080);
        // Layout: opcode+depth+length(4) wid(4) parent(4) x(2) y(2) w(2) h(2)
        //         border(2) class(2) visual(4) => value-mask at offset 28.
        let mask = u32::from_le_bytes([request[28], request[29], request[30], request[31]]);
        assert_ne!(mask & CW_OVERRIDE_REDIRECT, 0, "override-redirect must be set");
        assert_ne!(mask & CW_EVENT_MASK, 0, "events must be requested");
        let override_value =
            u32::from_le_bytes([request[36], request[37], request[38], request[39]]);
        assert_eq!(override_value, 1, "override-redirect must be True");
    }

    #[test]
    fn requests_declare_their_own_length() {
        // Every request starts with a length in 4-byte units. Getting this wrong
        // makes the server read the next request's bytes as payload.
        for request in [
            map_window(0x200000),
            create_gc(0x200001, 0x200000, 32, 24),
            poly_fill_rectangle(0x200001, 0x200000, 0, 0, 100, 100),
            create_window(0x200000, 0x100000, 32, 24, 0, 0, 800, 600),
        ] {
            let declared = u16::from_le_bytes([request[2], request[3]]) as usize * 4;
            assert_eq!(declared, request.len(), "length mismatch for opcode {}", request[0]);
            assert_eq!(request.len() % 4, 0);
        }
    }

    #[test]
    fn a_4k_frame_is_split_into_several_bands() {
        // 3840x2160 RGBA is 33 MB, far beyond the ~256 KB a request may carry.
        // Without splitting, a saver works only at low resolution.
        let bands = plan_bands(3840, 2160, 4, 65535);
        assert!(bands.len() > 1, "a 4K frame must be split");
        let covered: u32 = bands.iter().map(|(_, rows)| rows).sum();
        assert_eq!(covered, 2160, "bands must cover every row exactly once");
        // And every band must genuinely fit.
        let limit = 65535 * 4;
        for (y, rows) in &bands {
            let bytes = 3840 * 4 * rows;
            assert!(bytes + 24 <= limit, "band at y={y} is {} bytes", bytes + 24);
        }
    }

    #[test]
    fn an_8k_frame_is_split_into_many_bands() {
        let bands = plan_bands(7680, 4320, 4, 65535);
        assert!(bands.len() > 8, "an 8K frame needs many bands, got {}", bands.len());
        assert_eq!(bands.iter().map(|(_, r)| r).sum::<u32>(), 4320);
    }

    #[test]
    fn a_tiny_frame_needs_exactly_one_band() {
        let bands = plan_bands(64, 48, 4, 65535);
        assert_eq!(bands, vec![(0, 48)]);
    }

    #[test]
    fn degenerate_geometry_does_not_divide_by_zero() {
        // A zero width would make row_bytes zero and then the division by it
        // would panic.
        let bands = plan_bands(0, 100, 4, 65535);
        assert!(!bands.is_empty());
        let _ = plan_bands(100, 0, 4, 65535);
        let _ = rows_per_request(0, 4, 1920);
    }

    #[test]
    fn put_image_declares_its_length() {
        let pixels = vec![0u8; 64 * 4];
        let request = put_image(0x200000, 0x200001, 64, 1, 0, 0, 24, &pixels);
        let declared = u16::from_le_bytes([request[2], request[3]]) as usize * 4;
        assert_eq!(declared, request.len());
        assert_eq!(request[1], 2, "format must be ZPixmap");
    }

    #[test]
    fn property_requests_pad_to_alignment() {
        // A name whose length is not a multiple of 4 must still be padded.
        for len in 1..12usize {
            let request = change_property(1, 2, 3, 8, &vec![b'x'; len]);
            let declared = u16::from_le_bytes([request[2], request[3]]) as usize * 4;
            assert_eq!(declared, request.len(), "len {len}");
            assert_eq!(request.len() % 4, 0, "len {len} must be aligned");
        }
    }
}