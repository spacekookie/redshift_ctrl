extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate cairo;

mod rendering;

mod example {
    use gio;
    use gtk;

    use gio::prelude::*;
    use gtk::prelude::*;

    use gtk::{ApplicationWindow, Builder, Switch, Revealer, DrawingArea, Scale};
    use std::env::args;
    use std::cell::Cell;
    use std::sync::{Arc, Mutex};

    use std::f64;

    use rendering::*;

    static mut scale_val: Cell<f64> = Cell::new(0.0);

    // make moving clones into closures more convenient
    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }

    pub fn build_ui(application: &gtk::Application) {
        let ui_src = include_str!("settings.ui");
        let builder = Builder::new_from_string(ui_src);

        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get Window");
        let enable_simple_mode: Switch = builder.get_object("redctrl_simple_switch").unwrap();

        /* Have redshift enabled I guess */
        let global_state: Switch = builder.get_object("redctrl_enable_switch").unwrap();
        global_state.set_active(true);

        let simple_settings: Revealer = builder.get_object("redctrl_simple_setting").unwrap();
        let normal_settings: Revealer = builder.get_object("redctrl_normal_setting").unwrap();
        let draw_area: DrawingArea = builder.get_object("redctrl_curve_settings").unwrap();
        let color_scale: Scale = builder.get_object("redctl_change_value").unwrap();

        // let start_hour: SpinButton = builder.get_object("start_h").unwrap();
        // let start_minute: SpinButton = builder.get_object("start_m").unwrap();

        // let end_hour: SpinButton = builder.get_object("end_h").unwrap();
        // let end_minute: SpinButton = builder.get_object("end_m").unwrap();

        /* Handle drawing */
        draw_area.connect_draw(move |_self, ctx| {
            // let style_ctx = _self.get_style_context().unwrap();
            let width: f64 = _self.get_allocated_width() as f64;
            let height: f64 = _self.get_allocated_height() as f64;
            let area = Area { width, height };
            let offset = Point {
                x: 0.0,
                y: height / 2.0,
            };

            let other_offset = Point {
                x: -2.5,
                y: height / 2.0,
            };

            ctx.rectangle(0.0, 0.0, width, height);
            ctx.set_source_rgba(255.0, 196.0, 119.0, 0.85);
            ctx.fill();

            unsafe {
                let t = scale_val.get();
                set_time_displacement(t);
            }

            set_draw_color(255.0, 0.0, 0.0, 255.0);
            draw_wave(ctx, &area, &offset, 75.0, 75.0);

            set_draw_color(0.0, 255.0, 0.0, 255.0);
            draw_wave(ctx, &area, &other_offset, 25.0, 25.0);

            set_draw_color(0.0, 0.0, 255.0, 255.0);
            draw_wave(ctx, &area, &offset, 250.0, 50.0);

            return Inhibit(false);
        });

        color_scale.connect_value_changed(move |sc: &Scale| {
            unsafe {
                scale_val.set(sc.get_value());
            }

            println!("Scale value: {}", sc.get_value());
            draw_area.queue_draw();
        });


        enable_simple_mode.connect_changed_active(move |switch| {
            let state = switch.get_active();
            normal_settings.set_reveal_child(!state);
            simple_settings.set_reveal_child(state);
        });

        window.set_application(application);
        window.connect_delete_event(clone!(window => move |_, _| {
            window.destroy();
            Inhibit(false)
        }));

        window.show_all();
    }

    pub fn main() {
        let application =
            gtk::Application::new("de.spacekookie.redctrl", gio::ApplicationFlags::empty())
                .expect("Initialization failed...");

        application.connect_startup(move |app| { build_ui(app); });
        application.connect_activate(|_| {});

        /* Run our app */
        application.run(&args().collect::<Vec<_>>());
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example requires GTK 3.10 or later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}