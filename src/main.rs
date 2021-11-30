use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, Builder};
use std::collections::HashMap;

fn compress(data: &[u8]) -> Vec<u32> {
    // Build initial dictionary.
    let mut dictionary: HashMap<Vec<u8>, u32> = (0u32..=255)
        .map(|i| (vec![i as u8], i))
        .collect();

    let mut w = Vec::new();
    let mut compressed = Vec::new();

    for &b in data {
        let mut wc = w.clone();
        wc.push(b);

        if dictionary.contains_key(&wc) {
            w = wc;
        } else {
            // Write w to output.
            compressed.push(dictionary[&w]);

            // wc is a new sequence; add it to the dictionary.
            dictionary.insert(wc, dictionary.len() as u32);
            w.clear();
            w.push(b);
        }
    }
    // Write remaining output if necessary.
    if !w.is_empty() {
        compressed.push(dictionary[&w]);
    }
    compressed
}

fn decompress(mut data: &[u32]) -> Vec<u8> {
    // Build the dictionary.
    let mut dictionary: HashMap::<u32, Vec<u8>> = (0u32..=255)
        .map(|i| (i, vec![i as u8]))
        .collect();

    let mut w = dictionary[&data[0]].clone();
    data = &data[1..];
    let mut decompressed = w.clone();

    for &k in data {
        let entry = if dictionary.contains_key(&k) {
            dictionary[&k].clone()
        } else if k == dictionary.len() as u32 {
            let mut entry = w.clone();
            entry.push(w[0]);
            entry
        } else {
            panic!("Invalid dictionary!");
        };

        decompressed.extend_from_slice(&entry);

        // New sequence; add it to the dictionary.
        w.push(entry[0]);
        dictionary.insert(dictionary.len() as u32, w);

        w = entry;
    }
    decompressed
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("./layout.glade");
    let builder = Builder::from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.object("win1")
        .expect("Не получилось создать окно");
    let button: gtk::Button = builder.object("btn_1")
        .expect("Не получилось создать кнопку");
    let txt_ipt: gtk::TextView = builder.object("txt_ipt")
        .expect("Не получилось создать поле ввода");
    let txt_opt1: gtk::TextView = builder.object("txt_otp1")
        .expect("Не получилось создать поле вывода1");
    let txt_opt2: gtk::TextView = builder.object("txt_otp2")
        .expect("Не получилось создать поле вывода2");

    window.set_application(Some(app));
    window.show_all();

    button.connect_clicked(move |_| {
        let read_buf = txt_ipt.buffer().unwrap();
        let output_buf1 = txt_opt1.buffer().unwrap();
        let output_buf2 = txt_opt2.buffer().unwrap();

        let start = read_buf.start_iter();
        let end = read_buf.end_iter();

        let read_string = read_buf.text(&start, &end, false)
            .expect("Считать строку не удалось");

        output_buf2.set_text(""); // очистка буфера
        output_buf1.set_text(""); // очистка буфера
        let inp_string_as_bytes = format!("{:?}", read_string.as_bytes());
        output_buf1.set_text(&inp_string_as_bytes);
        let compress_byte = compress(read_string.as_bytes());
        let comp_str = format!("{:?}", compress_byte);
        output_buf2.set_text(&comp_str);
    });
}

fn main() {
    let app = Application::new(Some("com.icupken.LZW"), gio::ApplicationFlags::FLAGS_NONE);
    app.connect_activate(build_ui);
    app.run();
}
