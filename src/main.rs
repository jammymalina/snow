use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use gtk::prelude::*;

fn main() {
    gtk::init().unwrap();

    let m = gtk::Menu::new();
    let mi = gtk::MenuItem::with_label("Hello World");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.add(&mi);
    mi.show_all();

    let _indicator = Indicator::builder("Example")
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&m)
        .icon("weather-snow", "Snow icon to indicate a hidden meaning")
        .status(IndicatorStatus::Active)
        .build();

    gtk::main();
}
