pub enum Color {
    RGB { red: u8, green: u8, blue: u8 },
    HSV { hue: u16, saturation: u8, value: u8 },
}

impl Color {
    /// Constructs a new value from variant `RGB` with the given red, green, and blue values.
    ///
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::RGB { red, green, blue }
    }

    /// Constructs a new value from variant `HSV` with the given values.
    ///
    /// In case hue is above 360 ​​or saturation or value is above 100, we expect you to `panic!` with
    /// whatever message you choose.
    ///
    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        if hue > 360 {
            panic!()
        } else if saturation > 100 {
            panic!()
        } else if value > 100 {
            panic!()
        }

        Color::HSV {
            hue,
            saturation,
            value,
        }
    }

    /// If `self` is `RGB`, then you return its `red`, `green`, `blue` components in that order.
    /// Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_rgb(&self) -> (u8, u8, u8) {
        match self {
            &Color::RGB { red, blue, green } => (red, blue, green),
            _ => panic!(),
        }
    }

    /// If `self` is `HSV` then you return its `hue`, `saturation`, `value` components in this
    /// row. Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_hsv(&self) -> (u16, u8, u8) {
        match self {
            &Color::HSV {
                hue,
                saturation,
                value,
            } => (hue, saturation, value),
            _ => panic!(),
        }
    }

    /// In case the variant of `self` is `RGB`, we expect a string with the content `#rrggbb`, where
    /// the red, green, and blue components are formatted in hexadecimal, and each is
    /// exactly two lowercase (zero-padded) characters.
    ///
    /// If the variant is `HSV`, we expect a string `hsv(h,s%,v%)` where the numbers are typed in
    /// the decimal system, no leading zeros, no spaces after the commas, the second two ending in
    /// `%`.
    ///
    pub fn to_string(&self) -> String {
        match self {
            &Color::RGB { red, green, blue } => format!("#{:02x?}{:02x?}{:02x?}", red, green, blue),
            &Color::HSV {
                hue,
                saturation,
                value,
            } => format!("hsv({},{}%,{}%)", hue, saturation, value),
        }
    }

    /// Inverts color componentwise -- for each of the values, the difference with the maximum is taken.
    ///
    pub fn invert(&self) -> Self {
        match self {
            &Color::RGB { red, green, blue } => Color::RGB {
                red: u8::MAX - red,
                green: u8::MAX - green,
                blue: u8::MAX - blue,
            },
            &Color::HSV {
                hue,
                saturation,
                value,
            } => Color::HSV {
                hue: 360 - hue,
                saturation: 100 - saturation,
                value: 100 - value,
            },
        }
    }
}
