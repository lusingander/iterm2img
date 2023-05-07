//! Build iTerm2 [Inline Images Protocol](https://iterm2.com/documentation-images.html) string
//!
//! # Examples
//!
//! ```
//! let bytes = "abcdefg".as_bytes().to_vec();
//! let result = iterm2img::from_bytes(bytes)
//!     .name("xyz".to_string())
//!     .width(100)
//!     .height(200)
//!     .preserve_aspect_ratio(false)
//!     .inline(true)
//!     .build();
//!
//! let expected =  "\x1b]1337;File=size=7;name=xyz;width=100;height=200;preserve_aspect_ratio=0;inline=1:YWJjZGVmZw==\u{0007}";
//! assert_eq!(result, expected);
//! ```

use base64::Engine;

/// builder
pub struct Builder {
    bytes: Vec<u8>,
    name: Option<String>,
    width: Option<LengthUnit>,
    height: Option<LengthUnit>,
    preserve_aspect_ratio: Option<bool>,
    inline: Option<bool>,
}

enum LengthUnit {
    Cell(u64),
    Pixel(u64),
    Percent(u64),
    Auto,
}

/// returns builder from bytes
pub fn from_bytes(bytes: Vec<u8>) -> Builder {
    Builder {
        bytes,
        name: None,
        width: None,
        height: None,
        preserve_aspect_ratio: None,
        inline: None,
    }
}

impl Builder {
    /// set filename
    pub fn name(mut self, v: String) -> Builder {
        self.name = Some(v);
        self
    }

    /// set width cells
    pub fn width(mut self, v: u64) -> Builder {
        self.width = Some(LengthUnit::Cell(v));
        self
    }

    /// set width pixels
    pub fn width_px(mut self, v: u64) -> Builder {
        self.width = Some(LengthUnit::Pixel(v));
        self
    }

    /// set width percent
    pub fn width_percent(mut self, v: u64) -> Builder {
        self.width = Some(LengthUnit::Percent(v));
        self
    }

    /// set width auto
    pub fn width_auto(mut self) -> Builder {
        self.width = Some(LengthUnit::Auto);
        self
    }

    /// set height cells
    pub fn height(mut self, v: u64) -> Builder {
        self.height = Some(LengthUnit::Cell(v));
        self
    }

    /// set height pixels
    pub fn height_px(mut self, v: u64) -> Builder {
        self.height = Some(LengthUnit::Pixel(v));
        self
    }

    /// set height percent
    pub fn height_percent(mut self, v: u64) -> Builder {
        self.height = Some(LengthUnit::Percent(v));
        self
    }

    /// set height auto
    pub fn height_auto(mut self) -> Builder {
        self.height = Some(LengthUnit::Auto);
        self
    }

    /// set preserve_aspect_ratio
    pub fn preserve_aspect_ratio(mut self, v: bool) -> Builder {
        self.preserve_aspect_ratio = Some(v);
        self
    }

    /// set inline
    pub fn inline(mut self, v: bool) -> Builder {
        self.inline = Some(v);
        self
    }

    /// build string
    pub fn build(self) -> String {
        let mut s = String::new();

        s.push_str("\x1b]1337;File=");
        s.push_str(format!("size={}", self.bytes.len()).as_str());

        if let Some(name) = self.name {
            s.push_str(format!(";name={}", name).as_str());
        }

        if let Some(width) = self.width {
            match width {
                LengthUnit::Cell(w) => s.push_str(format!(";width={}", w).as_str()),
                LengthUnit::Pixel(w) => s.push_str(format!(";width={}px", w).as_str()),
                LengthUnit::Percent(w) => s.push_str(format!(";width={}%", w).as_str()),
                LengthUnit::Auto => s.push_str(";width=auto"),
            }
        }

        if let Some(height) = self.height {
            match height {
                LengthUnit::Cell(h) => s.push_str(format!(";height={}", h).as_str()),
                LengthUnit::Pixel(h) => s.push_str(format!(";height={}px", h).as_str()),
                LengthUnit::Percent(h) => s.push_str(format!(";height={}%", h).as_str()),
                LengthUnit::Auto => s.push_str(";height=auto"),
            }
        }

        if let Some(preserve_aspect_ratio) = self.preserve_aspect_ratio {
            let b = i32::from(preserve_aspect_ratio);
            s.push_str(format!(";preserve_aspect_ratio={}", b).as_str());
        }

        if let Some(inline) = self.inline {
            let b = i32::from(inline);
            s.push_str(format!(";inline={}", b).as_str());
        }

        let encoded = to_base64_str(self.bytes);
        s.push_str(format!(":{}\u{0007}", encoded).as_str());

        s
    }
}

fn to_base64_str(bytes: Vec<u8>) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let result = from_bytes(Vec::new()).name("xyz".to_string()).build();
        assert_eq!(result, "\x1b]1337;File=size=0;name=xyz:\u{0007}");
    }

    #[test]
    fn width() {
        let result = from_bytes(Vec::new()).width(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;width=100:\u{0007}");
    }

    #[test]
    fn width_px() {
        let result = from_bytes(Vec::new()).width_px(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;width=100px:\u{0007}");
    }

    #[test]
    fn width_percent() {
        let result = from_bytes(Vec::new()).width_percent(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;width=100%:\u{0007}");
    }

    #[test]
    fn width_auto() {
        let result = from_bytes(Vec::new()).width_auto().build();
        assert_eq!(result, "\x1b]1337;File=size=0;width=auto:\u{0007}");
    }

    #[test]
    fn height() {
        let result = from_bytes(Vec::new()).height(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;height=100:\u{0007}");
    }

    #[test]
    fn height_px() {
        let result = from_bytes(Vec::new()).height_px(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;height=100px:\u{0007}");
    }

    #[test]
    fn height_percent() {
        let result = from_bytes(Vec::new()).height_percent(100).build();
        assert_eq!(result, "\x1b]1337;File=size=0;height=100%:\u{0007}");
    }

    #[test]
    fn height_auto() {
        let result = from_bytes(Vec::new()).height_auto().build();
        assert_eq!(result, "\x1b]1337;File=size=0;height=auto:\u{0007}");
    }

    #[test]
    fn preserve_aspect_ratio() {
        let result = from_bytes(Vec::new()).preserve_aspect_ratio(true).build();
        assert_eq!(
            result,
            "\x1b]1337;File=size=0;preserve_aspect_ratio=1:\u{0007}"
        );

        let result = from_bytes(Vec::new()).preserve_aspect_ratio(false).build();
        assert_eq!(
            result,
            "\x1b]1337;File=size=0;preserve_aspect_ratio=0:\u{0007}"
        );
    }

    #[test]
    fn inline() {
        let result = from_bytes(Vec::new()).inline(true).build();
        assert_eq!(result, "\x1b]1337;File=size=0;inline=1:\u{0007}");

        let result = from_bytes(Vec::new()).inline(false).build();
        assert_eq!(result, "\x1b]1337;File=size=0;inline=0:\u{0007}");
    }

    #[test]
    fn all_options() {
        let result = from_bytes(Vec::new())
            .name("xyz".to_string())
            .width(100)
            .height(200)
            .preserve_aspect_ratio(false)
            .inline(true)
            .build();
        assert_eq!(
            result,
            "\x1b]1337;File=size=0;name=xyz;width=100;height=200;preserve_aspect_ratio=0;inline=1:\u{0007}"
        );
    }

    #[test]
    fn content() {
        // $ echo -n abcdefg | base64
        // YWJjZGVmZw==
        let result = from_bytes("abcdefg".as_bytes().to_vec()).build();
        assert_eq!(result, "\x1b]1337;File=size=7:YWJjZGVmZw==\u{0007}")
    }
}
