use seed_styles::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Breakpoint {
    Zero,
    Minimum,
    XXSmall,
    XSmall,
    Small,
    Medium,
    Normal,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}
impl BreakpointTheme for Breakpoint {} 

pub fn default_breakpoint_theme() -> Theme {
    use Breakpoint::*;
    Theme::new("default_breakpoint_theme")
        .set_breakpoint(Zero, (0, Some(640))) 
        .set_breakpoint(Minimum, (640, Some(800)))
        .set_breakpoint(XXSmall, (800, Some(1000)))
        .set_breakpoint(XSmall, (1000, Some(1300)))
        .set_breakpoint(Small, (1300, Some(1600)))
        .set_breakpoint(Medium, (1600, Some(1900)))
        .set_breakpoint(Normal, (1900, Some(2200)))
        .set_breakpoint(Large, (2200, Some(2500)))
        .set_breakpoint(XLarge, (2500, Some(2800)))
        .set_breakpoint(XXLarge, (2800,Some( 3800)))
        .set_breakpoint(XXXLarge, (3800, None))
        .breakpoint_scale([640, 800, 1000, 1300, 1600, 1900, 2200, 2500, 2800, 3800]) 
}

// @TODO: Remove unused
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    PrimaryLighter,
    PrimaryLighter80,
    PrimaryLighter60,
    PrimaryLighter40,
    PrimaryLighter20,
    PrimaryLight,
    PrimaryLight80,
    PrimaryLight60,
    PrimaryLight40,
    PrimaryLight20,
    Primary,
    Primary80,
    Primary60,
    Primary40,
    Primary20,
    PrimaryDark,
    PrimaryDark80,
    PrimaryDark60,
    PrimaryDark40,
    PrimaryDark20,
    PrimaryDarker,
    PrimaryDarker80,
    PrimaryDarker60,
    PrimaryDarker40,
    PrimaryDarker20,
    SecondaryLighter,
    SecondaryLighter80,
    SecondaryLighter60,
    SecondaryLighter40,
    SecondaryLighter20,
    SecondaryLight,
    SecondaryLight80,
    SecondaryLight60,
    SecondaryLight40,
    SecondaryLight20,
    Secondary,
    Secondary80,
    Secondary60,
    Secondary40,
    Secondary20,
    SecondaryDark,
    SecondaryDark80,
    SecondaryDark60,
    SecondaryDark40,
    SecondaryDark20,
    SecondaryDarker,
    SecondaryDarker80,
    SecondaryDarker60,
    SecondaryDarker40,
    SecondaryDarker20,
    BackgroundLighter,
    BackgroundLighter80,
    BackgroundLighter60,
    BackgroundLighter40,
    BackgroundLighter20,
    BackgroundLight,
    BackgroundLight80,
    BackgroundLight60,
    BackgroundLight40,
    BackgroundLight20,
    // Background,
    Background80,
    Background60,
    Background40,
    Background20,
    BackgroundDark,
    BackgroundDark80,
    BackgroundDark60,
    BackgroundDark40,
    BackgroundDark20,
    BackgroundDarker,
    BackgroundDarker80,
    BackgroundDarker60,
    BackgroundDarker40,
    BackgroundDarker20,
    SurfaceLighter,
    SurfaceLighter80,
    SurfaceLighter60,
    SurfaceLighter40,
    SurfaceLighter20,
    SurfaceLight,
    SurfaceLight80,
    SurfaceLight60,
    SurfaceLight40,
    SurfaceLight20,
    Surface,
    Surface80,
    Surface60,
    Surface40,
    Surface20,
    SurfaceDark,
    SurfaceDark80,
    SurfaceDark60,
    SurfaceDark40,
    SurfaceDark20,
    SurfaceDarker,
    SurfaceDarker80,
    SurfaceDarker60,
    SurfaceDarker40,
    SurfaceDarker20,
    Signal1,
    Signal180,
    Signal160,
    Signal140,
    Signal120,
    Signal2,
    Signal280,
    Signal260,
    Signal240,
    Signal220,
    Signal3,
    Signal380,
    Signal360,
    Signal340,
    Signal320,
    Signal4,
    Signal480,
    Signal460,
    Signal440,
    Signal420,
    Signal5,
    Signal580,
    Signal560,
    Signal540,
    Signal520,
    // new
    Accent3,
    Accent3_90,
    Accent3Light1,
    Accent3Light2,
    Accent4_90,
    Accent5_90,
    Accent4Light1_90,
    Background,
    BackgroundDark1,
    BackgroundDark2,
    BackgroundDark2_60,
    BackgroundDark2_70,
    BackgroundDark3,
    BackgroundLight1,
    BackgroundLight2,
    BackgroundLight3,
    SecondaryLight5_90,
    PrimaryVariant1,
    SecondaryVariant1_90,
    SecondaryVariant1Light1,
    SecondaryVariant1Light1_90,
    SecondaryVariant1Light2,
    SecondaryVariant1Light3,
    SecondaryVariant2Light1,
    SecondaryVariant2Light1_90,
    SecondaryVariant2Light2_90,
    SecondaryVariant1Dark1_60,
    SecondaryVariant1Dark4,
    SecondaryVariant1Dark5,
    Surface90,
    SurfaceLight1_90,
    SurfaceLight2,
    SurfaceLight2_90,
    SurfaceLight3_90,
    SurfaceLight5,
    SurfaceLight5_20,
    SurfaceLight5_30,
    SurfaceLight5_40,
    SurfaceLight5_60,
    SurfaceLight5_90,
    SurfaceDark2_90,
    SurfaceDark3_90,
    SurfaceDark4_90,
    SurfaceDark5,
    SurfaceDark5_10,
    SurfaceDark5_90,
}
impl ColorTheme for Color {} 

pub fn default_color_theme() -> Theme {
    use Color::*;
    Theme::new("default_color_theme")
        .set_color(PrimaryLighter, rgba(213,187,231,1))
        .set_color(PrimaryLighter80, rgba(213,187,231,0.8))
        .set_color(PrimaryLighter60, rgba(213,187,231,0.6))
        .set_color(PrimaryLighter40, rgba(213,187,231,0.4))
        .set_color(PrimaryLighter20, rgba(213,187,231,0.2))
        .set_color(PrimaryLight, rgba(176,135,203,1))
        .set_color(PrimaryLight80, rgba(176,135,203,0.8))
        .set_color(PrimaryLight60, rgba(176,135,203,0.6))
        .set_color(PrimaryLight40, rgba(176,135,203,0.4))
        .set_color(PrimaryLight20, rgba(176,135,203,0.2))
        .set_color(Primary, rgba(138,90,171,1))
        .set_color(Primary80, rgba(138,90,171,0.8))
        .set_color(Primary60, rgba(138,90,171,0.6))
        .set_color(Primary40, rgba(138,90,171,0.4))
        .set_color(Primary20, rgba(138,90,171,0.2))
        .set_color(PrimaryDark, rgba(112,59,148,1))
        .set_color(PrimaryDark80, rgba(112,59,148,0.8))
        .set_color(PrimaryDark60, rgba(112,59,148,0.6))
        .set_color(PrimaryDark40, rgba(112,59,148,0.4))
        .set_color(PrimaryDark20, rgba(112,59,148,0.2))
        .set_color(PrimaryDarker, rgba(89,34,127,1))
        .set_color(PrimaryDarker80, rgba(89,34,127,0.8))
        .set_color(PrimaryDarker60, rgba(89,34,127,0.6))
        .set_color(PrimaryDarker40, rgba(89,34,127,0.4))
        .set_color(PrimaryDarker20, rgba(89,34,127,0.2))
        .set_color(SecondaryLighter, rgba(164,176,214,1))
        .set_color(SecondaryLighter80, rgba(164,176,214,0.8))
        .set_color(SecondaryLighter60, rgba(164,176,214,0.6))
        .set_color(SecondaryLighter40, rgba(164,176,214,0.4))
        .set_color(SecondaryLighter20, rgba(164,176,214,0.2))
        .set_color(SecondaryLight, rgba(113,129,182,1))
        .set_color(SecondaryLight80, rgba(113,129,182,0.8))
        .set_color(SecondaryLight60, rgba(113,129,182,0.6))
        .set_color(SecondaryLight40, rgba(113,129,182,0.4))
        .set_color(SecondaryLight20, rgba(113,129,182,0.2))
        .set_color(Secondary, rgba(76,94,155,1))
        .set_color(Secondary80, rgba(76,94,155,0.8))
        .set_color(Secondary60, rgba(76,94,155,0.6))
        .set_color(Secondary40, rgba(76,94,155,0.4))
        .set_color(Secondary20, rgba(76,94,155,0.2))
        .set_color(SecondaryDark, rgba(51,70,133,1))
        .set_color(SecondaryDark80, rgba(51,70,133,0.8))
        .set_color(SecondaryDark60, rgba(51,70,133,0.6))
        .set_color(SecondaryDark40, rgba(51,70,133,0.4))
        .set_color(SecondaryDark20, rgba(51,70,133,0.2))
        .set_color(SecondaryDarker, rgba(30,48,109,1))
        .set_color(SecondaryDarker80, rgba(30,48,109,0.8))
        .set_color(SecondaryDarker60, rgba(30,48,109,0.6))
        .set_color(SecondaryDarker40, rgba(30,48,109,0.4))
        .set_color(SecondaryDarker20, rgba(30,48,109,0.2))
        .set_color(BackgroundLighter, rgba(41,43,68,1))
        .set_color(BackgroundLighter80, rgba(41,43,68,0.8))
        .set_color(BackgroundLighter60, rgba(41,43,68,0.6))
        .set_color(BackgroundLighter40, rgba(41,43,68,0.4))
        .set_color(BackgroundLighter20, rgba(41,43,68,0.2))
        .set_color(BackgroundLight, rgba(25,26,53,1))
        .set_color(BackgroundLight80, rgba(25,26,53,0.8))
        .set_color(BackgroundLight60, rgba(25,26,53,0.6))
        .set_color(BackgroundLight40, rgba(25,26,53,0.4))
        .set_color(BackgroundLight20, rgba(25,26,53,0.2))
        // .set_color(Background, rgba(13,14,37,1))
        .set_color(Background80, rgba(13,14,37,0.8))
        .set_color(Background60, rgba(13,14,37,0.6))
        .set_color(Background40, rgba(13,14,37,0.4))
        .set_color(Background20, rgba(13,14,37,0.2))
        .set_color(BackgroundDark, rgba(5,6,22,1))
        .set_color(BackgroundDark80, rgba(5,6,22,0.8))
        .set_color(BackgroundDark60, rgba(5,6,22,0.6))
        .set_color(BackgroundDark40, rgba(5,6,22,0.4))
        .set_color(BackgroundDark20, rgba(5,6,22,0.2))
        .set_color(BackgroundDarker, rgba(0,0,0,1))
        .set_color(BackgroundDarker80, rgba(0,0,0,0.8))
        .set_color(BackgroundDarker60, rgba(0,0,0,0.6))
        .set_color(BackgroundDarker40, rgba(0,0,0,0.4))
        .set_color(BackgroundDarker20, rgba(0,0,0,0.2))
        .set_color(SurfaceLighter, rgba(255,255,255,1))
        .set_color(SurfaceLighter80, rgba(255,255,255,0.8))
        .set_color(SurfaceLighter60, rgba(255,255,255,0.6))
        .set_color(SurfaceLighter40, rgba(255,255,255,0.4))
        .set_color(SurfaceLighter20, rgba(255,255,255,0.2))
        .set_color(SurfaceLight, rgba(225,225,225,1))
        .set_color(SurfaceLight80, rgba(225,225,225,0.8))
        .set_color(SurfaceLight60, rgba(225,225,225,0.6))
        .set_color(SurfaceLight40, rgba(225,225,225,0.4))
        .set_color(SurfaceLight20, rgba(225,225,225,0.2))
        .set_color(Surface, rgba(180,180,180,1))
        .set_color(Surface80, rgba(180,180,180,0.8))
        .set_color(Surface60, rgba(180,180,180,0.6))
        .set_color(Surface40, rgba(180,180,180,0.4))
        .set_color(Surface20, rgba(180,180,180,0.2))
        .set_color(SurfaceDark, rgba(137,137,137,1))
        .set_color(SurfaceDark80, rgba(137,137,137,0.8))
        .set_color(SurfaceDark60, rgba(137,137,137,0.6))
        .set_color(SurfaceDark40, rgba(137,137,137,0.4))
        .set_color(SurfaceDark20, rgba(137,137,137,0.2))
        .set_color(SurfaceDarker, rgba(88,88,88,1))
        .set_color(SurfaceDarker80, rgba(88,88,88,0.8))
        .set_color(SurfaceDarker60, rgba(88,88,88,0.6))
        .set_color(SurfaceDarker40, rgba(88,88,88,0.4))
        .set_color(SurfaceDarker20, rgba(88,88,88,0.2))
        .set_color(Signal1, rgba(251,185,25,1))
        .set_color(Signal180, rgba(251,185,25,0.8))
        .set_color(Signal160, rgba(251,185,25,0.6))
        .set_color(Signal140, rgba(251,185,25,0.4))
        .set_color(Signal120, rgba(251,185,25,0.2))
        .set_color(Signal2, rgba(251,94,25,1))
        .set_color(Signal280, rgba(251,94,25,0.8))
        .set_color(Signal260, rgba(251,94,25,0.6))
        .set_color(Signal240, rgba(251,94,25,0.4))
        .set_color(Signal220, rgba(251,94,25,0.2))
        .set_color(Signal3, rgba(199,150,44,1))
        .set_color(Signal380, rgba(199,150,44,0.8))
        .set_color(Signal360, rgba(199,150,44,0.6))
        .set_color(Signal340, rgba(199,150,44,0.4))
        .set_color(Signal320, rgba(199,150,44,0.2))
        .set_color(Signal4, rgba(25,251,184,1))
        .set_color(Signal480, rgba(25,251,184,0.8))
        .set_color(Signal460, rgba(25,251,184,0.6))
        .set_color(Signal440, rgba(25,251,184,0.4))
        .set_color(Signal420, rgba(25,251,184,0.2))
        .set_color(Signal5, rgba(34,180,103,1))
        .set_color(Signal580, rgba(34,180,103,0.8))
        .set_color(Signal560, rgba(34,180,103,0.6))
        .set_color(Signal540, rgba(34,180,103,0.4))
        .set_color(Signal520, rgba(34,180,103,0.2))
        // new
        .set_color(Accent3, hsl(147.7, 68, 41.7))
        .set_color(Accent3_90, hsla(147.7, 68, 41.7, 0.9))
        .set_color(Accent3Light1, hsl(147.7, 68, 46.7))
        .set_color(Accent3Light2, hsl(147.7, 68, 51.7))
        .set_color(Accent4_90, hsla(160, 81.5, 46.8, 0.9))
        .set_color(Accent4Light1_90, hsla(160, 81.5, 51.8, 0.9))
        .set_color(Accent5_90, hsla(42, 100, 54.9, 0.9))
        .set_color(Background, hsl(243, 24.4, 21))
        .set_color(BackgroundDark1, hsl(243, 24.4, 16))
        .set_color(BackgroundDark2, hsl(243, 24.4, 11))
        .set_color(BackgroundDark2_60, hsla(243, 24.4, 11, 0.6))
        .set_color(BackgroundDark2_70, hsla(243, 24.4, 11, 0.7))
        .set_color(BackgroundDark3, hsl(243, 24.4, 6))
        .set_color(BackgroundLight1, hsl(243, 24.4, 26))
        .set_color(BackgroundLight2, hsl(243, 24.4, 31))
        .set_color(BackgroundLight3, hsl(243, 24.4, 36))
        .set_color(SecondaryLight5_90, hsla(226.6, 37.2, 61.9, 0.9))
        .set_color(PrimaryVariant1, hsl(276.8, 48, 62))
        .set_color(SecondaryVariant1_90, hsla(224.3, 42.1, 66, 0.9))
        .set_color(SecondaryVariant1Light1, hsl(224.3, 42.1, 71))
        .set_color(SecondaryVariant1Light1_90, hsla(224.3, 42.1, 71, 0.9))
        .set_color(SecondaryVariant1Light2, hsl(224.3, 42.1, 76))
        .set_color(SecondaryVariant1Light3, hsl(224.3, 42.1, 81))
        .set_color(SecondaryVariant2Light1, hsl(222.8, 100, 78))
        .set_color(SecondaryVariant2Light1_90, hsla(222.8, 100, 78, 0.9))
        .set_color(SecondaryVariant2Light2_90, hsla(222.8, 100, 83, 0.9))
        .set_color(SecondaryVariant1Dark1_60, hsla(224.3, 42.1, 61, 0.6))
        .set_color(SecondaryVariant1Dark4, hsl(224.3, 42.1, 46))
        .set_color(SecondaryVariant1Dark5, hsl(224.3, 42.1, 41))
        .set_color(Surface90, hsla(0, 0, 75, 0.9))
        .set_color(SurfaceLight1_90, hsla(0, 0, 80, 0.9))
        .set_color(SurfaceLight2, hsl(0, 0, 85))
        .set_color(SurfaceLight2_90, hsla(0, 0, 85, 0.9))
        .set_color(SurfaceLight3_90, hsla(0, 0, 90, 0.9))
        .set_color(SurfaceLight5, hsl(0, 0, 100))
        .set_color(SurfaceLight5_20, hsla(0, 0, 100, 0.2))
        .set_color(SurfaceLight5_30, hsla(0, 0, 100, 0.3))
        .set_color(SurfaceLight5_40, hsla(0, 0, 100, 0.4))
        .set_color(SurfaceLight5_60, hsla(0, 0, 100, 0.6))
        .set_color(SurfaceLight5_90, hsla(0, 0, 100, 0.9))
        .set_color(SurfaceDark2_90, hsla(0, 0, 65, 0.9))
        .set_color(SurfaceDark3_90, hsla(0, 0, 60, 0.9))
        .set_color(SurfaceDark4_90, hsla(0, 0, 55, 0.9))
        .set_color(SurfaceDark5, hsl(0, 0, 50))
        .set_color(SurfaceDark5_10, hsla(0, 0, 50, 0.1))
        .set_color(SurfaceDark5_90, hsla(0, 0, 50, 0.9))
}

// @TODO refactor + hide in the library Styles
pub fn get_color_value(color: Color) -> String {
    app_themes().get_with(|themes| {
        themes
            .iter()
            .find(|theme| theme.name == "default_color_theme")
            .map(|theme| {
                let colors = theme
                    .anymap.get::<HashMap<Color, CssColor>>().unwrap();
                let css_color = colors.get(&color).unwrap();
                let string_color = css_color.to_string();
                string_color
                    .strip_prefix("color: ").unwrap()
                    .strip_suffix(";").unwrap()
                    .to_owned()
            })
    }).unwrap()
}

