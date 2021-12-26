// #![allow(dead_code)]
// #![allow(unused_variables)]

use gtk::prelude::*;
use gtk::*;
use lzw;
use std::fs::*;
use std::io::*;
use std::path::PathBuf;

// TODO: убрать все unwrap
fn read(path: PathBuf) -> Vec<u32> {
    let file = File::open(path).unwrap(); // open file by given path
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line.unwrap();
        for i in line.chars() {
            v.push(i as u32);
        }
    }
    v
}

fn _write(path: &str, data: Vec<f64>) -> Result<()> {
    let mut output = File::create(path)?;
    for i in 0..data.len() {
        if data[i] > 0.0 {
            write!(output, "{}\n", data[i])?;
        }
    }
    Ok(())
}


fn process(ui: &Ui) {
    let start = ui.read_buf.start_iter();
    let end = ui.read_buf.end_iter();

    let read_string = &ui.read_buf
        .text(&start, &end, false)
        .expect("Can't read buffer!");

    // перегон считанной строки в u32 для дальнейшего сжатия и вывода
    let mut input_str_byte: Vec<u32> = Vec::new();
    for i in read_string.chars() {
        input_str_byte.push(i as u32);
    };

    // вывод в буффер не сжатого массива
    Ui::buffer_out(&ui.output_buf1, format!("{:?}", &input_str_byte));

    // сжатие
    let compress_byte = lzw::compress(&input_str_byte);

    // вычисление "успешности" сжатия
    let profit = compress_byte.len() as f64 / input_str_byte.len() as f64;
    let profit_str = format!(
        "Compression ratio is {:.3} ({}%). Input symbols - {} and Output symbols - {}",
        profit,
        (profit * 100.0).round(),
        input_str_byte.len(),
        compress_byte.len(),
    );
    // вывод статистки в буффер
    Ui::buffer_out(&ui.stats_buf, profit_str);

    // вывод в буффер сжатого массива
    Ui::buffer_out(&ui.output_buf2, format!("{:?}", compress_byte));
}

fn menu_open(ui: &Ui) {
    let dialog = gtk::FileChooserDialog::builder().build();
    dialog.set_action(gtk::FileChooserAction::Open);
    dialog.set_title("Open File");
    dialog.add_button("Open", gtk::ResponseType::Accept);
    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    let res = dialog.run();
    if res == ResponseType::Accept {
        let filename = dialog.filename().expect("Filename to open not received!");
        let file_read_buf = read(filename);
        Ui::buffer_out(&ui.read_buf, format!("{:?}",  file_read_buf));
        dialog.close();
    } else if res == ResponseType::Cancel || res == gtk::ResponseType::DeleteEvent {
        dialog.close();
    }
}

fn menu_save(ui: &Ui) {
    let dialog = gtk::FileChooserDialog::builder().build();
    dialog.set_action(gtk::FileChooserAction::Save);
    dialog.set_title("Save File");
    dialog.add_button("Save", gtk::ResponseType::Accept);
    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    dialog.set_do_overwrite_confirmation(true);
    let res = dialog.run();
    if res == ResponseType::Accept {
        // TODO: сохранение в файл
        dialog.close();
    } else if res == ResponseType::Cancel || res == gtk::ResponseType::DeleteEvent {
        dialog.close();
    }
}


struct App {
    window: ApplicationWindow,
    builder: Builder,
}

struct Ui {
    open: MenuItem,
    save: MenuItem,
    output_buf1: TextBuffer,
    output_buf2: TextBuffer,
    stats_buf: TextBuffer,
    read_buf: TextBuffer,
    button: Button,
}

impl App {
    fn new() -> App {
        // Создадим новое окно и билдер
        let glade_src = include_str!("./layout.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window: gtk::ApplicationWindow = builder.object("win1")
            .expect("Can't create window-main!");

        // Закрыть окно по нажатию кнопки
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            builder,
        }
    }
}

impl Ui {
    fn new(app: &App) -> Ui {
        let menubar: gtk::MenuBar = app.builder.object("menubar")
            .expect("Can't create menubar!");
        let menu_file = gtk::Menu::new();
        let menu_help = gtk::Menu::new();

        let file = gtk::MenuItem::with_label("File");
        let open = gtk::MenuItem::with_label("Open");
        let save = gtk::MenuItem::with_label("Save");
        let quit = gtk::MenuItem::with_label("Quit");

        let help = gtk::MenuItem::with_label("Help");
        let about = gtk::MenuItem::with_label("About");

        menu_file.append(&open);
        menu_file.append(&save);
        menu_file.append(&quit);

        menu_help.append(&about);

        file.set_submenu(Some(&menu_file));
        help.set_submenu(Some(&menu_help));

        menubar.append(&file);
        menubar.append(&help);

        let button: gtk::Button = app.builder.object("btn_1")
            .expect("Can't create button-encode!");
        let txt_ipt: gtk::TextView = app.builder.object("txt_ipt")
            .expect("Can't create input field!");
        let txt_opt1: gtk::TextView = app.builder.object("txt_otp1")
            .expect("Can't create output field1!");
        let txt_opt2: gtk::TextView = app.builder.object("txt_otp2")
            .expect("Can't create output field2!");
        let stat_buff: gtk::TextView = app.builder.object("stats")
            .expect("Can't create statistic field");
        let read_buf = txt_ipt.buffer().unwrap();
        let output_buf1 = txt_opt1.buffer().unwrap();
        let output_buf2 = txt_opt2.buffer().unwrap();
        let stats_buf = stat_buff.buffer().unwrap();

        // FIXME:
        // let start = read_buf.start_iter();
        // let end = read_buf.end_iter();

        let about_win: gtk::Window = app.builder.object("win2")
            .expect("Can't create window-about!");

        about.connect_activate(move |_| {
            about_win.show();
        });

        quit.connect_activate(move |_| {
            main_quit();
        });

        Ui {
            open,
            save,
            read_buf,
            output_buf1,
            output_buf2,
            stats_buf,
            button,
        }
    }

    fn buffer_out(buffer_name: &gtk::TextBuffer, out_str: String) {
        buffer_name.set_text("");
        buffer_name.set_text(&out_str);
    }
}

fn main() {
    // Инициализируем GTK перед продолжением.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        std::process::exit(1);
    }
    let app = App::new();
    let ui = Ui::new(&app);
    app.window.show_all();

    ui.button.clone().connect_clicked(move |_| {
        process(&ui);
    });

    // ui.open.connect_activate(move |_| {
    //     menu_open(&ui);
    // });

    // ui.save.connect_activate(move |_| {
    //     menu_save(&ui);
    // });

    gtk::main();
}