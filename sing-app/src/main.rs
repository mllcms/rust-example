use std::{thread, time::Duration};

use sing_app::SingApp;

fn main() {
    // let _app = SingApp::run();
    let _app = SingApp::run_current();
    thread::sleep(Duration::from_secs(30))
}
