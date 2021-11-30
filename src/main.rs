use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, Builder};

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("./layout.glade");
    let builder = Builder::from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.object("win1")
        .expect("Не получилось создать окно");
    let button: gtk::Button = builder.object("btn1")
        .expect("Не получилось создать кнопку");
    let txt_ipt: gtk::TextView = builder.object("txt_ipt")
        .expect("Не получилось создать поле ввода");
    let txt_opt1: gtk::TextView = builder.object("txt_otp1")
       .expect("Не получилось создать поле вывода1");
    let txt_opt2: gtk::TextView = builder.object("txt_otp1")
       .expect("Не получилось создать поле вывода2");

    window.set_application(Some(app));
    window.show_all();

    button.connect_clicked(move |_| {
        let read_buf = txt_ipt.buffer().unwrap();
        let start = read_buf.start_iter();
        let end = read_buf.end_iter();
        let read_string = read_buf.text(&start, &end, false)
            .expect("Считать строку не удалось");
        println!("{}", read_string);
    });
}

fn main() {
    let app = Application::new(Some("com.icupken.LZW"), gio::ApplicationFlags::FLAGS_NONE);
    app.connect_activate(build_ui);
    app.run();
}
