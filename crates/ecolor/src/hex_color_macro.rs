/// Construct a [`crate::Color32`] from a hex RGB or RGBA string.
///
/// Requires the "color-hex" feature.
///
/// See also [`crate::Color32::from_hex`] and [`crate::Color32::to_hex`].
///
/// ```
/// # use ecolor::{hex_color, Color32};
/// assert_eq!(hex_color!("#202122"), Color32::from_hex("#202122").unwrap());
/// assert_eq!(hex_color!("#202122"), Color32::from_rgb(0x20, 0x21, 0x22));
/// assert_eq!(hex_color!("#abcdef12"), Color32::from_rgba_unmultiplied(0xab, 0xcd, 0xef, 0x12));
/// ```
#[macro_export]
macro_rules! hex_color {
    ($s:literal) => {{
        match $crate::color_hex::color_from_hex!($s).as_slice() {
            [r, g, b] => $crate::Color32::from_rgb(*r, *g, *b),
            [r, g, b, a] => $crate::Color32::from_rgba_unmultiplied(*r, *g, *b, *a),
            _ => unreachable!(),
        }
    }};
}

#[test]
fn test_from_rgb_hex() {
    assert_eq!(
        crate::Color32::from_rgb(0x20, 0x21, 0x22),
        hex_color!("#202122")
    );
    assert_eq!(
        crate::Color32::from_rgb_additive(0x20, 0x21, 0x22),
        hex_color!("#202122").additive()
    );
}

#[test]
fn test_from_rgba_hex() {
    assert_eq!(
        crate::Color32::from_rgba_unmultiplied(0x20, 0x21, 0x22, 0x50),
        hex_color!("20212250")
    );
}
