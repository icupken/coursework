#![allow(dead_code)]
#![allow(unused_variables)]

use gtk::prelude::*;
use gtk::{gio};
use gtk::{Application, Builder, Menu, MenuItem};
use lzw;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("./layout.glade");
    let builder = Builder::from_string(glade_src);
    // todo: make something pls.... it's awful
    let window: gtk::ApplicationWindow = builder.object("win1")
        .expect("Can't create window-main!");
    let about_win: gtk::Window = builder.object("win2")
        .expect("Can't create window-about!");
    let button: gtk::Button = builder.object("btn_1")
        .expect("Can't create button-encode!");
    let txt_ipt: gtk::TextView = builder.object("txt_ipt")
        .expect("Can't create input field!");
    let txt_opt1: gtk::TextView = builder.object("txt_otp1")
        .expect("Can't create output field1!");
    let txt_opt2: gtk::TextView = builder.object("txt_otp2")
        .expect("Can't create output field2!");
    let menubar: gtk::MenuBar = builder.object("menubar")
        .expect("Can't create menubar!");

    window.set_application(Some(app));
    window.show_all();

    // build menubar
    let menu_file = Menu::new();
    let menu_help = Menu::new();

    let file = MenuItem::with_label("File");
    let open = MenuItem::with_label("Open");
    let save = MenuItem::with_label("Save");
    let quit = MenuItem::with_label("Quit");

    let help = MenuItem::with_label("Help");
    let about = MenuItem::with_label("About");

    menu_file.append(&open);
    menu_file.append(&save);
    menu_file.append(&quit);

    menu_help.append(&about);

    file.set_submenu(Some(&menu_file));
    help.set_submenu(Some(&menu_help));

    menubar.append(&file);
    menubar.append(&help);

    quit.connect_activate(move |_| {
        window.close();
    });
    about.connect_activate(move |_| {
        about_win.show();
    });

    button.connect_clicked(move |_| {
        let read_buf = txt_ipt.buffer().unwrap();
        let output_buf1 = txt_opt1.buffer().unwrap();
        let output_buf2 = txt_opt2.buffer().unwrap();

        let start = read_buf.start_iter();
        let end = read_buf.end_iter();

        let read_string = read_buf
            .text(&start, &end, false)
            .expect("Can't read buffer!");

        output_buf2.set_text(""); // clear buff
        output_buf1.set_text(""); // clear buff
        let inp_string_as_bytes = format!("{:?}", read_string.as_bytes());
        output_buf1.set_text(&inp_string_as_bytes);
        let compress_byte = lzw::compress(read_string.as_bytes());
        let comp_str = format!("{:?}", compress_byte);
        output_buf2.set_text(&comp_str);
    });
}

fn main() {
    let app = Application::new(Some("com.icupken.LZW"),
                               gio::ApplicationFlags::FLAGS_NONE);
    app.connect_activate(build_ui);
    app.run();
}
