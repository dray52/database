/*
By: <Draydon Levesque>
Date: 2026-03-26
Program Details: Database of movies
*/

mod modules;

use std::process::exit;

use crate::modules::database::{DatabaseTable, create_database_client, create_table_from_struct};
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use crate::modules::listview::ListView;
use macroquad::prelude::*;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "movie_database".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let client = create_database_client();

          let mut records: Vec<DatabaseTable> = Vec::new();
    if let Ok(result) = client.fetch_table("movies").await {
        records = result;
        // To update a ListView with these records:
        // update_listview(&mut list_view, &records);
    } else {
        // Handle error
    }


  let items = vec!["Item 1".to_string(), "Item 2".to_string()];
    let mut list_view = ListView::new(&items, 700.0, 100.0, 30);
    

    let btn_add = TextButton::new(50.0, 600.0, 150.0, 60.0, "ADD", BLUE, GREEN, 30);
    let btn_remove = TextButton::new(250.0, 600.0, 150.0, 60.0, "REMOVE", BLUE, GREEN, 30);
    let btn_exit = TextButton::new(450.0, 600.0, 150.0, 60.0, "EXIT", BLUE, GREEN, 30);
    let btn_search = TextButton::new(650.0, 600.0, 150.0, 60.0, "SEARCH", BLUE, GREEN, 30);
    let btn_update = TextButton::new(850.0, 600.0, 150.0, 60.0, "UPDATE", BLUE, GREEN, 30);

    let mut txt_name = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);
    let mut txt_id = TextInput::new(100.0, 200.0, 370.0, 40.0, 25.0);
    let mut txt_summary = TextInput::new(100.0, 300.0, 300.0, 40.0, 25.0);
    let mut txt_length = TextInput::new(100.0, 400.0, 300.0, 40.0, 25.0);

    txt_id.set_prompt("ID (ONLY FOR SEARCHING/REMOVING)");
    txt_name.set_prompt("Name");
    txt_summary.set_prompt("Summary");
    txt_length.set_prompt("Length (in hours) EX: 2.5");

    let lbl_title = Label::new("MOVIES", 400.0, 50.0, 50);
    let lbl_name = Label::new("NAME", 500.0, 80.0, 25);
    let lbl_summary = Label::new("SUMMARY", 500.0, 280.0, 25);
    let lbl_length = Label::new("LENGTH", 500.0, 500.0, 25);
    loop {
        clear_background(RED);
        draw_grid(50.0, BLACK);

        if btn_search.click() {
            println!("Search button clicked!");
        }

        if btn_exit.click() {
            exit(0)
        }
        if btn_add.click() {
            println!("Add button clicked!");
        }
        if btn_remove.click() {
            println!("Remove button clicked!");
        }
        if btn_update.click() {
            println!("Update button clicked!");
        }
        lbl_title.draw();
        lbl_name.draw();
        lbl_summary.draw();
        lbl_length.draw();
        txt_name.draw();
        txt_id.draw();
        txt_summary.draw();
        txt_length.draw();
        list_view.draw();
        next_frame().await;
    }
}
