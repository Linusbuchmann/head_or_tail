#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rand::prelude::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        min_window_size: None,
        max_window_size: Some(egui::vec2(320.0, 240.0)), 
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        ..Default::default()
    };
    
    // Our application state:
    let mut credits_var: i32 = 10;
    let mut heads_tails_var = rand::thread_rng().gen_range(1..3);
    let mut heads_tails_string = "heads_tails";
    let mut bet_value = 0;
    let mut checkbox_bool = false;
    let mut checkbox_string = "heads_tails";
    let mut win_or_lose: &str = "";
    let mut no_money: bool = false;

    eframe::run_simple_native("Head or tails", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(format!("your credits: {credits_var}"));
                ui.add_space(110.0);
                ui.heading("Head or tails");
            });

        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut bet_value, 0..=credits_var));
            ui.add(egui::Checkbox::new(&mut checkbox_bool, checkbox_string));
            if ui.add_enabled(no_money,egui::Button::new("Click")).clicked() {
                heads_tails_var = rand::thread_rng().gen_range(1..3);
                if checkbox_string == heads_tails_string {
                    credits_var += bet_value;
                    win_or_lose = "won";
                }
                else if checkbox_string != heads_tails_string {
                    credits_var -= bet_value;
                    win_or_lose = "lost";
                }
            }
        });

        ui.label(format!("it landed on: {heads_tails_string} and you: {win_or_lose}"));
        if credits_var <= 0 {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                if ui.button("MONEY").clicked() {
                    credits_var = 10;
                    bet_value = 10;   
                }
            });
        }
    });

// mhm....

    if credits_var >= 1 {
        no_money = true;
    }
    else if credits_var <= 1 {
        no_money = false;
    }

    if checkbox_bool == false {
        checkbox_string = "heads";
    }

    else if checkbox_bool == true {
        checkbox_string = "tails";
    }

    if heads_tails_var == 1 {
        heads_tails_string = "heads";
    }

    else if heads_tails_var == 2 {
        heads_tails_string = "tails";
    }
})
}