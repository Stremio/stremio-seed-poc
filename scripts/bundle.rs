//# fs_extra = "*"
//# remove_dir_all = "*"

use std::{fs, path::Path};
use remove_dir_all::remove_dir_all;

const DIST_DIR: &str = "dist";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    prepare_dist_directory();
    bundle_assets();
    bundle_index_html();
    bundle_stremio_wasm();
}

fn prepare_dist_directory() {
    if Path::new(DIST_DIR).is_dir() {
        remove_dir_all(DIST_DIR).expect("remove dist directory");
    }
    fs::create_dir(DIST_DIR).expect("create dist directory");
}

fn bundle_assets() {
    fs_extra::dir::copy("fonts", DIST_DIR, &fs_extra::dir::CopyOptions::new()).expect("copy fonts");
}

fn bundle_index_html() {
    let mut index_html_content = fs::read_to_string("index.html").expect("read index.html");
    let text_min_js_content = fs::read_to_string("text.min.js").expect("read text.min.js");
    let styles_css_content = fs::read_to_string("styles.css").expect("read styles.css");
    let package_js_content = fs::read_to_string("pkg/package.js").expect("read pkg/package.js");

    index_html_content = index_html_content.replace(
        "<script src='text.min.js'></script>",
        &format!("<script>{}</script>", text_min_js_content)
    );

    index_html_content = index_html_content.replace(
        r#"<link rel="stylesheet" type="text/css" href="styles.css">"#,
        &format!("<style>{}</style>", styles_css_content)
    );

    index_html_content = index_html_content.replace(
        r#"<script type="module"> import init from '/pkg/package.js'; init('/pkg/package_bg.wasm'); </script>"#,
        &format!(r#"<script type="module">{} init('/stremio-{}.wasm'); </script>"#, package_js_content, VERSION)
    );

    fs::write(format!("{}/{}", DIST_DIR, "index.html"), index_html_content).expect("write index.html");
}

fn bundle_stremio_wasm() {
    fs::copy("pkg/package_bg.wasm", format!("{}/stremio-{}.wasm", DIST_DIR, VERSION)).expect("copy pkg/package_bg.wasm");
}
