use std::borrow::Cow;

use dryad::client::{MetadataApp, sdl::SdlClient, traits::AppBuilder};

fn main() {
    let client = SdlClient::init(
        MetadataApp::default()
            .with_title(Cow::from("Dryad"))
            .with_width(800)
            .with_height(600),
    );

    dryad::start(client);
}
