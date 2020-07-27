use gio::prelude::*;
use gio::{ApplicationFlags};

use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    WindowPosition,
    Box as GtkBox,
    ButtonBuilder,
    Orientation,
    Button,
};

use glib::signal::Inhibit;

use super::minefield::board::Board;
use super::minefield::location::Location;

use std::rc::Rc;


pub fn run(width: i32, height: i32, count: i32) {

    let app =  Application::new(
        Some("org.gtkrsnotes.demo"),
        ApplicationFlags::FLAGS_NONE
    ).expect("Application::new failed");

    app.connect_activate(move |app| {
        let board = Rc::new(Board::new(width, height, count).unwrap());
        let win = ApplicationWindow::new(app);
        let rows = GtkBox::new(Orientation::Vertical, 2);

        for y in 0..board.height() {
            let row = GtkBox::new(Orientation::Horizontal, 2);

            for x in 0..board.width() {
                let item = create_grid_item(&board, x, y);
                row.add(&item);
            }
            rows.add(&row);
        }

        win.add(&rows);

        win.set_default_size(320, 200);
        win.set_title("Kaboom");
        win.set_position(WindowPosition::Center);
        win.set_border_width(10);
        win.show_all();
    });

    app.run(&[]);
}

fn create_grid_item(board: &Rc<Board>, x: i32, y: i32) -> Button {
    let board = Rc::clone(&board);

    let button = ButtonBuilder::new()
        .can_focus(false)
        .label("")
        .height_request(25)
        .width_request(25)
        .build();

    button.connect_event(|_,e|{
        println!("{:?}", e.get_button());
        Inhibit(false)
    });

    button.connect_clicked(move |b|{
        println!("{:?}", board.location_at(x, y));

        b.set_label(match board.location_at(x, y) {
            Location { has_bomb: true, .. } => "x",
            Location { surrounding_bomb_count: 0, .. } => " ",
            Location { surrounding_bomb_count: 1, .. } => "1",
            Location { surrounding_bomb_count: 2, .. } => "2",
            Location { surrounding_bomb_count: 3, .. } => "3",
            Location { surrounding_bomb_count: 4, .. } => "4",
            Location { surrounding_bomb_count: 5, .. } => "5",
            Location { surrounding_bomb_count: 6, .. } => "6",
            Location { surrounding_bomb_count: 7, .. } => "7",
            Location { surrounding_bomb_count: 8, .. } => "8",
            _ => "?"
        });
        b.set_sensitive(false);
    });

    button
}
