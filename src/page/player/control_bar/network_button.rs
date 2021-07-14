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
pub fn network_button() -> Node<Msg> {
    let disabled = true;
    div![
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(rem(4))
            .justify_content(CssJustifyContent::Center)
            .width(rem(4))
            .cursor(CssCursor::Pointer),
        IF!(disabled => s().pointer_events("none")),
        C!["control-bar-button", "button-container", IF!(disabled => "disabled")],
        attrs!{
            At::TabIndex => -1,
        },
        icon(disabled),
    ]
}

#[view]
pub fn icon(disabled: bool) -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, if disabled { 75 } else { 100 }))
            .flex(CssFlex::None)
            .height(rem(2))
            .width(rem(3))
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 1024 1024",
            At::from("icon") => "ic_network",
        },
        path![
            attrs!{
                At::D => "M512 0c-282.77 0-512 229.23-512 512s229.23 512 512 512c282.77 0 512-229.23 512-512v0c0-282.77-229.23-512-512-512v0zM512 77.101h12.649c-77.345 56.905-140.107 129.003-184.771 212.261l-1.657 3.382c-5.762-1.488-12.377-2.342-19.192-2.342-44.079 0-79.812 35.733-79.812 79.812 0 5.842 0.628 11.538 1.82 17.024l-0.096-0.527c-62.24 30.758-115.88 66.688-164.173 108.616l0.935-0.795c9.792-232.397 200.422-417.131 434.267-417.431l0.030-0zM659.275 315.934c-23.69 3.618-44.377 14.442-60.246 30.128l0.010-0.010c-8.965 8.837-16.272 19.327-21.431 30.981l-0.254 0.642c-48.249-9.688-103.715-15.233-160.477-15.233-6.054 0-12.093 0.063-18.116 0.189l0.899-0.015c-1.756-19.196-10.172-36.151-22.885-48.787l-0.004-0.004-2.711-2.409c50.882-94.221 122.83-171.184 209.87-226.529l2.459-1.461h11.144c30.321 67.798 52.112 146.432 61.406 228.851l0.335 3.657zM646.325 98.485c102.992 34.487 186.786 103.149 239.833 192.248l1.109 2.011c-35.501 27.252-77.527 48.044-123.202 59.718l-2.389 0.518c-2.078-2.918-4.154-5.471-6.376-7.887l0.052 0.057c-14.754-14.871-33.742-25.521-55.005-29.992l-0.712-0.125s0-3.012 0-4.216c-9.737-78.922-28.462-150.65-55.38-218.226l2.072 5.896zM602.353 506.278c-59.324 72.3-133.142 130.407-217.259 170.346l-3.804 1.625c-1.807-2.108-3.313-4.518-5.421-6.626-14.479-14.387-34.432-23.279-56.463-23.279-1.327 0-2.646 0.032-3.957 0.096l0.185-0.007c-8.020-29.597-12.628-63.578-12.628-98.635s4.608-69.039 13.25-101.372l-0.622 2.736c0.428 0.008 0.933 0.013 1.439 0.013 22.143 0 42.187-8.983 56.688-23.504l0.001-0.001c6.681-6.773 12.156-14.757 16.064-23.588l0.2-0.506c6.793-0.203 14.784-0.318 22.803-0.318 53.604 0 106.004 5.158 156.73 15.005l-5.151-0.833c-0.195 2.453-0.307 5.311-0.307 8.195 0 30.184 12.198 57.519 31.934 77.343l-0.004-0.004zM259.012 421.647l4.216 4.819c4.689 4.775 10.039 8.878 15.908 12.165l0.356 0.183c-9.152 32.974-14.412 70.837-14.412 109.929s5.26 76.956 15.11 112.922l-0.699-2.992c-5.789 3.582-10.814 7.459-15.407 11.789l0.047-0.044c-13.346 13.61-21.966 31.897-23.18 52.179l-0.011 0.225c-42.108-0.883-82.289-7.757-120.166-19.8l3.008 0.826c-23.48-46.088-39.196-99.914-43.869-156.868l-0.102-1.55c51.649-48.156 110.63-89.437 174.871-121.806l4.329-1.977zM248.169 764.386v0c3.784 8.027 8.63 14.896 14.463 20.787l-0.006-0.006c14.453 14.239 34.306 23.032 56.213 23.032 7.54 0 14.837-1.042 21.754-2.989l-0.564 0.136c33.319 54.071 74.076 99.91 121.514 137.722l1.065 0.82c-130.722-15.402-242.154-86.903-311.034-189.39l-0.985-1.556c29.211 6.983 62.832 11.137 97.372 11.443l0.21 0.001zM375.567 785.769v0c14.495-14.452 23.463-34.44 23.463-56.522 0-4.824-0.428-9.547-1.248-14.135l0.072 0.483c95.317-44.691 175.116-108.096 237.525-186.031l1.007-1.301c6.431 2.497 14.025 4.59 21.878 5.924l0.711 0.1c-18.048 150.881-63.115 287.869-130.361 411.008l2.963-5.926c-69.271-30.419-126.494-107.52-156.009-153.6zM575.849 941.779c63.072-118.673 107.119-257.162 123.888-403.936l0.498-5.362c21.599-4.891 40.226-15.487 54.812-30.115l0.003-0.003c2.711-3.012 5.12-6.325 7.529-9.336 48.489 14.456 134.024 50.899 169.261 130.409-45.658 166.472-183.294 291.49-353.538 318.029l-2.453 0.315zM783.059 457.487c3.295-10.036 5.195-21.587 5.195-33.581s-1.9-23.545-5.416-34.367l0.221 0.786c47.577-13.559 89.1-34.183 125.799-61.011l-1.112 0.775c24.728 52.867 39.157 114.791 39.157 180.086 0 0.642-0.001 1.283-0.004 1.924l0-0.099c-0.146 18.836-1.455 37.217-3.859 55.251l0.245-2.244c-40.552-50.443-95.202-88.040-157.955-106.933l-2.271-0.587z",
            }
        ]
    ]
}
