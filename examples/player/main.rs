// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(not(target_os = "android"))]
extern crate clap;
extern crate euclid;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[cfg(not(target_os = "android"))]
extern crate glutin;
extern crate ipc_channel;
extern crate servo_media;
extern crate webrender;
extern crate webrender_api;
#[cfg(not(target_os = "android"))]
extern crate winit;

use servo_media::ServoMedia;
use std::path::Path;

#[cfg(not(target_os = "android"))]
#[path = "app.rs"]
mod app;
use app::*;


#[cfg(not(target_os = "android"))]
#[path = "mse_app.rs"]
mod mse_app;
use mse_app::*;

#[cfg(target_os = "android")]
fn main() {
    panic!("Unsupported");
}

#[cfg(not(target_os = "android"))]
fn main() {
    ServoMedia::init::<servo_media_auto::Backend>();

    let clap_matches = clap::App::new("Servo-media player example")
        .setting(clap::AppSettings::DisableVersion)
        .author("Servo developers")
        .about("Servo/MediaPlayer example using WebRender")
        .usage("player [[--gl, --wr-stats]|--no-video] <FILE>")
        .arg(
            clap::Arg::with_name("gl")
                .long("gl")
                .display_order(1)
                .help("Tries to render frames as GL textures")
                .conflicts_with("no-video"),
        )
        .arg(
            clap::Arg::with_name("no-video")
                .long("no-video")
                .display_order(2)
                .help("Don't render video, only audio"),
        )
        .arg(
            clap::Arg::with_name("wr-stats")
                .long("wr-stats")
                .display_order(3)
                .help("Show WebRender profiler on screen")
                .conflicts_with("no-video"),
        )
        .arg(
            clap::Arg::with_name("mse")
                .long("mse")
                .display_order(4)
                .help("Render video using the mse extension")
                .conflicts_with("no-video"),
        )
        .arg(
            clap::Arg::with_name("file")
                .required(true)
                .value_name("FILE"),
        )
        .get_matches();

    let opts = Options {
        no_video: clap_matches.is_present("no-video"),
        use_gl: clap_matches.is_present("gl"),
        wr_stats: clap_matches.is_present("wr-stats"),
    };


    if clap_matches.is_present("mse") {
    let path = clap_matches.value_of("file").map(|s| Path::new(s)).unwrap();

    match MseApp::new(path).and_then(mse_main_loop).and_then(mse_cleanup) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
    } else {
    let path = clap_matches.value_of("file").map(|s| Path::new(s)).unwrap();

    match App::new(opts, path).and_then(main_loop).and_then(cleanup) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
    }
}
