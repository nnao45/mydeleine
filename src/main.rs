extern crate gtk;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let ui = include_str!("../ui/dialog_window_01.ui");
    let builder = gtk::Builder::new_from_string(ui);
    
    let window1 : gtk::Window = builder.get_object("dialog_window_01").unwrap();
    window1.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window1.show_all();
    
    gtk::main();
}