use seed_style::{px, em, pc, rem, Style};
use seed_style::*;

pub trait GlobalStyleFontFaces {
    fn add_font_faces(self) -> Self;
}

impl GlobalStyleFontFaces for GlobalStyle {
    fn add_font_faces(self) -> Self where Self: Sized {
        self
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("300")
                    .raw("src: url('/fonts/Roboto-LightItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("400")
                    .raw("src: url('/fonts/Roboto-RegularItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("500")
                    .raw("src: url('/fonts/Roboto-MediumItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("700")
                    .raw("src: url('/fonts/Roboto-BoldItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("300")
                    .raw("src: url('/fonts/Roboto-Light.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("400")
                    .raw("src: url('/fonts/Roboto-Regular.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("500")
                    .raw("src: url('/fonts/Roboto-Medium.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("700")
                    .raw("src: url('/fonts/Roboto-Bold.ttf') format('truetype');") 
            )
    }
}
