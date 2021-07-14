use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
use enclose::enc;
use serde::Serialize;
use crate::{PageId, Context, Actions, Events};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::types::resource::{Stream, StreamSource};
use stremio_core::models::player::Selected as PlayerSelected;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg, Internal};
use super::Msg;

#[view]
pub fn screencast_button() -> Node<Msg> {
    div![
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(rem(4))
            .justify_content(CssJustifyContent::Center)
            .width(rem(4))
            .cursor(CssCursor::Pointer),
        C!["control-bar-button", "button-container"],
        attrs!{
            At::TabIndex => -1,
        },
        ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); }),
        icon(),
    ]
}

#[view]
pub fn icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, 100))
            .flex(CssFlex::None)
            .height(rem(2))
            .width(rem(3))
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 1248 1024",
            At::from("icon") => "ic_cast",
        },
        path![
            attrs!{
                At::D => "M1247.774 292.141v-163.539c0.014-0.705 0.022-1.536 0.022-2.369 0-25.286-7.419-48.839-20.2-68.602l0.3 0.496c-22.443-35.182-61.27-58.178-105.467-58.178-1.251 0-2.498 0.018-3.741 0.055l0.182-0.004h-995.991c-66.831 0.479-121.042 53.846-122.876 120.3l-0.004 0.17c0 37.346 0 75.294 0 111.736 0 15.661 0 30.118 0 46.984s6.927 25.6 26.202 25.901h82.522c1.616 0.279 3.476 0.439 5.374 0.439 7.919 0 15.189-2.778 20.889-7.414l-0.061 0.048c4.001-5.639 6.397-12.663 6.397-20.246 0-2.428-0.245-4.798-0.713-7.087l0.039 0.227c0-16.264 0-32.828 0-49.092v-34.635c0-13.854 0-27.708 0-43.369 0.072-0.722 0.112-1.561 0.112-2.409s-0.041-1.687-0.121-2.515l0.008 0.105h969.788v26.504c0 12.649 0 25.299 0 37.948s0 32.226 0 48.188v400.264c0 10.541 0 21.082 0 31.624 0 17.769 0 35.238 0 52.706v90.353c0 19.275 0 38.852 0 60.235h-364.725c-0.419-0.024-0.909-0.037-1.402-0.037-6.755 0-12.919 2.527-17.599 6.687l0.027-0.024c-3.974 4.647-6.392 10.728-6.392 17.373 0 0.669 0.025 1.333 0.073 1.99l-0.005-0.088c0 19.576 0 39.153 0 58.729v30.118c0 18.673 6.325 24.998 25.299 24.998h112.038q131.313 0 263.228 0v0c0 0 0.001 0 0.001 0 69.528 0 125.892-56.364 125.892-125.892 0-0.212-0.001-0.423-0.002-0.635l0 0.033c0-38.551 0-77.101 0-115.652z",
            }
        ],
        path![
            attrs!{
                At::D => "M570.127 726.136c-45.557-87.92-108.351-161.018-184.427-217.155l-1.701-1.198c-50.839-37.899-109.826-68.648-173.412-89.159l-4.282-1.194c-53.805-18.222-115.815-29.171-180.249-30.112l-0.457-0.005c-0.61-0.055-1.32-0.086-2.037-0.086-6.168 0-11.795 2.318-16.057 6.13l0.023-0.020c-3.774 4.152-6.085 9.694-6.085 15.775 0 0.596 0.022 1.186 0.066 1.771l-0.005-0.078q0 46.080 0 92.16c-0.071 0.651-0.112 1.406-0.112 2.17 0 11.643 9.439 21.082 21.082 21.082 0.569 0 1.134-0.023 1.692-0.067l-0.074 0.005c38.257 1.093 74.788 5.973 109.992 14.285l-3.677-0.732c34.559 8.541 64.648 19.581 93.113 33.439l-2.76-1.213c65.045 30.939 119.844 73.501 163.859 125.486l0.584 0.707c49.104 58.395 85.080 129.473 102.397 207.571l0.606 3.252c6.061 27.848 9.966 60.185 10.828 93.279l0.014 0.688c-0.045 0.508-0.070 1.099-0.070 1.696 0 11.311 9.169 20.48 20.48 20.48 0.981 0 1.945-0.069 2.889-0.202l-0.109 0.013h60.235c10.842 0 21.685 0 32.226 0h1.807c0.306 0.015 0.665 0.024 1.026 0.024 5.755 0 11.001-2.182 14.956-5.763l-0.019 0.017c4.123-4.402 6.655-10.338 6.655-16.865 0-0.424-0.011-0.846-0.032-1.264l0.002 0.059c-0.758-26.735-3.14-52.209-7.064-77.171l0.438 3.383c-9.612-74.923-31.586-142.854-63.924-204.494l1.581 3.308z",
            }
        ],
        path![
            attrs!{
                At::D => "M346.654 783.059c-30.246-43.515-67.507-79.817-110.518-108.088l-1.519-0.938c-58.496-38.891-129.786-63.029-206.538-65.638l-0.671-0.018c-0.987-0.115-2.13-0.18-3.288-0.18-6.563 0-12.635 2.099-17.583 5.662l0.089-0.061c-3.862 4.574-6.209 10.536-6.209 17.046 0 1.104 0.067 2.192 0.199 3.261l-0.013-0.128v88.847c0 18.974 7.228 26.805 26.202 26.805 49.069 1.007 94.378 16.277 132.181 41.814l-0.868-0.553c51.412 33.037 89.775 82.534 107.934 140.931l0.489 1.827c6.315 19.746 10.33 42.533 11.132 66.136l0.011 0.424c-0.016 0.325-0.025 0.706-0.025 1.090 0 6.372 2.537 12.152 6.656 16.384l-0.005-0.005c4.272 3.926 9.995 6.332 16.28 6.332 0.206 0 0.412-0.003 0.617-0.008l-0.030 0.001h90.353c1.034 0.153 2.227 0.241 3.441 0.241 6.323 0 12.092-2.377 16.461-6.285l-0.024 0.021c3.189-4.048 5.116-9.221 5.116-14.844 0-1.891-0.218-3.732-0.63-5.498l0.032 0.163c-0.799-18.573-2.523-35.909-5.156-52.945l0.337 2.649c-9.375-62.263-32.141-117.876-65.302-165.74l0.851 1.298z",
            }
        ],
        path![
            attrs!{
                At::D => "M27.407 830.042c-1.56-0.351-3.351-0.551-5.19-0.551-5.663 0-10.881 1.906-15.047 5.112l0.058-0.043c-4.014 4.335-6.477 10.157-6.477 16.555 0 0.959 0.055 1.904 0.163 2.834l-0.011-0.113v145.769c-0.040 0.518-0.062 1.122-0.062 1.731 0 6.313 2.428 12.060 6.402 16.356l-0.015-0.016c4.314 3.926 10.073 6.329 16.394 6.329 0.166 0 0.332-0.002 0.497-0.005l-0.025 0h148.179c0.467 0.033 1.011 0.051 1.56 0.051 6.27 0 11.974-2.425 16.224-6.389l-0.014 0.013c3.517-4.198 5.653-9.658 5.653-15.616 0-1.184-0.084-2.349-0.248-3.488l0.015 0.13c-1.725-23.969-7.987-46.1-17.929-66.077l0.461 1.023c-27.317-58.394-83.713-99.091-150.041-103.575l-0.547-0.030z",
            }
        ],
    ]
}
