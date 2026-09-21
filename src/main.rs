use gtk::cairo;
use gtk::gio::ApplicationFlags;
use gtk::{Application, ApplicationWindow};
use gtk::{CssProvider, prelude::*};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use std::fs;
use std::time::Duration;

mod audio;
mod voskr;

const APP_ID: &str = "com.brayan.ttsoverlay";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_command_line(|app, command_line| {
        let args = command_line.arguments();
        let modelo;
        let argumentos = args.iter().skip(1).cloned().collect::<Vec<_>>();
        if argumentos.len() > 0 {
            modelo = argumentos[0].to_string_lossy().to_string();
        } else {
            let exe = std::env::current_exe().unwrap();
            let exe_dir = exe.parent().unwrap();
            let archivo = exe_dir.join("model.txt");
            if archivo.exists() {
                modelo = fs::read_to_string(archivo).unwrap();
            } else {
                println!("tts-overlay [modelo]");
                println!(
                    "[TTS-OVERLAY] Puedes crear un archivo model.txt con la ubicacion del modelo."
                );
                println!("[ERROR] No se ha proporcionado un modelo.");
                return 0;
            }
        }
        build_ui(app, modelo);
        0
    });

    app.run();
}

fn build_ui(app: &Application, ruta_modelo: String) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TTS Overlay")
        .decorated(false)
        .build();

    let css = CssProvider::new();

    css.load_from_data(include_str!("style.css").as_bytes())
        .expect("No se pudo cargar el CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Posición

    let label = gtk::Label::new(Some("-"));

    label.set_line_wrap(true);
    label.set_width_request(800);
    label.set_valign(gtk::Align::Start);
    label.set_xalign(1.0);

    // Espacio entre el texto y el borde inferior
    label.set_margin_bottom(20);

    window.add(&label);

    let model = if let Ok(variable) = std::env::var("vosk_model") {
        variable
    } else {
        ruta_modelo
    };

    let reciber = audio::start_audio_thread(&model);

    let label_clone = label.clone();

    gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
        if let Ok(value) = reciber.try_recv() {
            let texto = format!("- {} -", value);
            label_clone.set_text(&texto);
        }

        gtk::glib::ControlFlow::Continue
    });
    window.set_keyboard_interactivity(false);
    window.init_layer_shell();
    window.set_anchor(Edge::Bottom, true);
    window.set_layer(Layer::Overlay);
    label.set_margin(100);
    // window.set_keyboard_interactivity(false);
    window.show_all();
    let region = cairo::Region::create();
    window.input_shape_combine_region(Some(&region));
}
