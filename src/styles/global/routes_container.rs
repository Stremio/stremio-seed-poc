use seed_style::{px, em, pc, rem, Style};
use seed_style::*;

pub trait GlobalStyleRoutesContainer {
    fn add_routes_container(self) -> Self;
}

impl GlobalStyleRoutesContainer for GlobalStyle {
    fn add_routes_container(self) -> Self where Self: Sized {
        self
            .style(
                ".routes-container",
                s()
                    .position(CssPosition::Relative)
                    .z_index("0")
            )
            .style(
                ".routes-container .route-container",
                s()
                    .position(CssPosition::Absolute)
                    .top("0")
                    .right("0")
                    .bottom("0")
                    .left("0")
                    .z_index("0")
            )
            .style(
                ".routes-container .route-container:not(:last-child)",
                s()
                    .display(CssDisplay::None)
            )
            .style(
                ".routes-container .route-container .route-content",
                s()
                    .position(CssPosition::Absolute)
                    .top("0")
                    .right("0")
                    .bottom("0")
                    .left("0")
                    .z_index("0")
                    .overflow(CssOverflow::Hidden)
            )
            .style(
                ".routes-container .route-container .modals-container",
                s()
                    .width("0")
                    .height("0")
            )
            .style(
                ".routes-container .route-container .modals-container .modal-container",
                s()
                    .position(CssPosition::Absolute)
                    .top("0")
                    .right("0")
                    .bottom("0")
                    .left("0")
                    .z_index("1")
                    .overflow(CssOverflow::Hidden)
            )
    }
}
