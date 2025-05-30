mod tui;
mod args;
mod views;
mod focus;
mod utils;
mod logger;
mod config;
mod state;

use crate::args::Args;
use crate::state::{AppState, ViewMode};
use clap::Parser;
use common::traits::savenload::SaveNLoad;
use cursive::align::{HAlign, VAlign};
use cursive::event::Key;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, Panel, ResizedView, SelectView, StackView, TextView};
use cursive::{theme, Cursive, CursiveExt, With};
use log::{debug, error, info, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, iter};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};


fn main() {
	let args = Args::parse();

	let config = config::prepare_config(&args.config_path).unwrap_or_else(|e| {
		println!("解析配置失败: {}", e);
		let c = config::Config::builder()
			.build();
		c.save(&args.config_path).expect("Failed to save config");
		c
	});
	logger::init_logger(
		config.log_level,
		&config.log_dir,
		config.log_to_console,
	);

	info!("数据目录：{}, 存储目录：{}", config.data_dir().to_string_lossy(), config.storage_dir().to_string_lossy());
	let mut siv = Cursive::default();
	siv.set_autorefresh(true);
	siv.set_fps(60);
	let classes = utils::load_saving(config.storage_dir()).expect("Failed to load classes");
	info!("{} classes loaded", classes.len());
	let state = AppState::builder()
		.config(config.clone())
		.classes(classes)
		.current_view_mode(
			ViewMode::ClassList
		)
		.build();
	siv.set_user_data(
		state
	);
	siv.set_global_callback(Key::Esc, |s| {
		s.pop_layer();
	});
	let view = siv.user_data::<AppState>().unwrap().build_view_mode();
	siv.add_layer(view);

	info!("{} classes loaded", siv.user_data::<AppState>().unwrap().classes.len());

	siv.run();
}
