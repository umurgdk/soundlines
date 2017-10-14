#![feature(cargo_resource_root)]

#[macro_use] extern crate soundlines_core;
#[macro_use] extern crate log;

extern crate env_logger;
extern crate ggez;
extern crate spade;

use soundlines_core::*;

use ggez::conf;
use ggez::event;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Point2;
use std::time::Duration;
use std::path;
use std::env;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use spade::rtree::RTree;
use spade::SpatialObject;

const WINDOW_WIDTH: f32 = 1033.0;
const WINDOW_HEIGHT: f32 = 800.0;
const VIEWPORT_WIDTH: f32 = WINDOW_WIDTH;
const VIEWPORT_HEIGHT: f32 = WINDOW_HEIGHT;

const MAP_GEO_LEFT: f64 = 126.989223;
const MAP_GEO_TOP: f64 = 37.579291;
const MAP_GEO_RIGHT: f64 = 127.015067;
const MAP_GEO_BOTTOM: f64 = 37.563660;

#[derive(Copy, Clone, Eq, PartialEq)]
enum HighlightTarget {
	Seeds,
	Entities
}

struct MainState {
	pos_x: f32,
	world_map: graphics::Image,
	font: graphics::Font,
	message_sender: Sender<Message>,
	callback_receiver: Receiver<Message>,
	cells: Vec<models::Cell>,
	mouse_pos: Point2,
	cells_shade: graphics::Mesh,
	cells_line: graphics::Mesh,
	seeds_mesh: graphics::Mesh,
	entities_mesh: graphics::Mesh,
	seeds_tree: RTree<models::Seed>,
	entities_tree: RTree<models::Entity>,
	highlight_target: HighlightTarget
}

impl MainState {
	fn new(ctx: &mut Context, message_sender: Sender<Message>, callback_receiver: Receiver<Message>) -> GameResult<MainState> {
		let world_map = graphics::Image::new(ctx, "/worldmap.png")?;
		let font = graphics::Font::new(ctx, "/andale_mono.ttf", 12)?;

		let state = MainState {
			pos_x: 0.0,
			cells: Vec::new(),
			mouse_pos: Point2::new(0.0, 0.0),
			cells_shade: graphics::Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 1.0, 10.0)?,
			cells_line: graphics::Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 1.0, 10.0)?,
			seeds_mesh: graphics::Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 1.0, 10.0)?,
			entities_mesh: graphics::Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 1.0, 10.0)?,
			seeds_tree: RTree::new(),
			entities_tree: RTree::new(),
			highlight_target: HighlightTarget::Seeds,
			world_map,
			font,
			message_sender,
			callback_receiver
		};

		Ok(state)
	}

	fn handle_callbacks(&mut self, ctx: &mut Context) -> GameResult<()> {
		for msg in self.callback_receiver.try_iter() {
			match msg {
				Message::ReceiveCells(cells) => {
					self.cells = cells;

					let mut cell_shade = graphics::MeshBuilder::new();
					let mut cell_line = graphics::MeshBuilder::new();
					for cell in &self.cells {
						let polygon = geo_to_polygon(&cell.geom);
						cell_shade.polygon(DrawMode::Fill, &polygon);
						cell_line.polygon(DrawMode::Line(2.0), &polygon);
					}

					self.cells_shade = cell_shade.build(ctx)?;
					self.cells_line = cell_line.build(ctx)?;
				},

				Message::ReceiveSeeds(seeds) => {
					self.seeds_tree = RTree::new();
					let mut builder = graphics::MeshBuilder::new();

					for (_, mut seed) in seeds.into_iter() {
						builder.circle(DrawMode::Fill, geo_to_point(&seed.point), 2.0, 0.9);
						let screen_pos = geo_to_point(&seed.point);
						seed.point = [screen_pos[0] as f64, screen_pos[1] as f64];
						self.seeds_tree.insert(seed);
					}

					self.seeds_mesh = builder.build(ctx)?;
				},

				Message::ReceiveEntities(entities) => {
					self.entities_tree = RTree::new();
					let mut builder = graphics::MeshBuilder::new();

					for (_, mut entity) in entities.into_iter() {
						builder.circle(DrawMode::Fill, geo_to_point(&entity.point), 2.0, 0.9);
						let screen_pos = geo_to_point(&entity.point);
						entity.point = [screen_pos[0] as f64, screen_pos[1] as f64];
						self.entities_tree.insert(entity);
					}

					self.entities_mesh = builder.build(ctx)?;
				}

				_ => ()
			}
		}

		Ok(())
	}

	pub fn highlight_seeds(&self, ctx: &mut Context) -> GameResult<()> {
		let seeds_around_mouse = self.seeds_tree.lookup_in_circle(&[self.mouse_pos[0] as f64, self.mouse_pos[1] as f64], &1000.0);
		graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
		for (i, seed) in seeds_around_mouse.iter().enumerate() {
			graphics::circle(ctx, DrawMode::Line(1.0), Point2::new(seed.point[0] as f32, seed.point[1] as f32), 10.0, 1.0);
			let txt = graphics::Text::new(ctx, &format!("ID: {}, AGE: {}, SHOULD BLOOM: {:?}", seed.id, seed.age, seed.should_bloom()), &self.font)?;
			graphics::draw(ctx, &txt, Point2::new(WINDOW_WIDTH - 450.0 + txt.width() as f32 / 2.0, i as f32 * 30.0 + 10.0), 0.0)?;
		}

		Ok(())
	}

	pub fn highlight_entities(&self, ctx: &mut Context) -> GameResult<()> {
		let entities_around_mouse = self.entities_tree.lookup_in_circle(&[self.mouse_pos[0] as f64, self.mouse_pos[1] as f64], &1000.0);
		graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
		for (i, entitiy) in entities_around_mouse.iter().enumerate() {
			graphics::circle(ctx, DrawMode::Line(1.0), Point2::new(entitiy.point[0] as f32, entitiy.point[1] as f32), 10.0, 1.0);
			let txt = graphics::Text::new(ctx, &format!("ID: {}, AGE: {}, MATING: {:?}", entitiy.id, entitiy.age, entitiy.is_mating()), &self.font)?;
			graphics::draw(ctx, &txt, Point2::new(WINDOW_WIDTH - 450.0 + txt.width() as f32 / 2.0, i as f32 * 30.0 + 10.0), 0.0)?;
		}

		Ok(())
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.handle_callbacks(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let fps = ggez::timer::get_fps(ctx);

		graphics::clear(ctx);
		graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0));
		graphics::draw(ctx, &self.world_map, Point2::new(1033.0 / 2.0, 400.0), 0.0);

		let fps_text = graphics::Text::new(ctx, &format!("FPS: {:.2}", fps), &self.font)?;
		let n_seeds_text = graphics::Text::new(ctx, &format!("Seeds: {}", self.seeds_tree.size()), &self.font)?;
		let n_entities_text = graphics::Text::new(ctx, &format!("Entities: {}", self.entities_tree.size()), &self.font)?;

		graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0))?;
		graphics::draw(ctx, &fps_text, Point2::new(150.0, 20.0), 0.0)?;
		graphics::draw(ctx, &n_seeds_text, Point2::new(150.0, 40.0), 0.0)?;
		graphics::draw(ctx, &n_entities_text, Point2::new(150.0, 60.0), 0.0)?;

		// Draw cell meshes
		graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 0.5))?;
		graphics::draw(ctx, &self.cells_shade, Point2::new(0.0, 0.0), 0.0)?;
		graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0))?;
		graphics::draw(ctx, &self.cells_line, Point2::new(0.0, 0.0), 0.0)?;

		// Draw mouse area
		graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0));
		graphics::circle(ctx, DrawMode::Line(2.0), self.mouse_pos.clone(), 50.0, 0.9)?;

		match self.highlight_target {
			HighlightTarget::Seeds => self.highlight_seeds(ctx),
			HighlightTarget::Entities => self.highlight_entities(ctx)
		}?;

		// Draw seeds
		graphics::set_color(ctx, Color::new(0.8, 0.7, 0.2, 1.0))?;
		graphics::draw(ctx, &self.seeds_mesh, Point2::new(0.0, 0.0), 0.0)?;

		// Draw entities
		graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
		graphics::draw(ctx, &self.entities_mesh, Point2::new(0.0, 0.0), 0.0)?;


		graphics::present(ctx);
		Ok(())
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: event::MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
		self.mouse_pos = Point2::new(x as f32, y as f32);
	}
	fn key_down_event(&mut self, ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
		match keycode {
			event::Keycode::Space => {
				if self.highlight_target == HighlightTarget::Seeds {
					self.highlight_target = HighlightTarget::Entities;
				} else {
					self.highlight_target = HighlightTarget::Seeds;
				}
			},

			_ => ()
		}
	}
}

fn main() {
	env_logger::init().expect("Failed to init logger");

	let mut c = conf::Conf::new();
	c.window_width = 1033;
	c.window_height = 800;
	let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

	let (message_sender, message_receiver) = channel();
	let (callback_sender, callback_receiver) = channel();

	let info_callback_sender = callback_sender.clone();
	let info_thread = ::std::thread::spawn(move || info_thread(info_callback_sender, message_receiver));

	// Sync with simulation on every simulation frame
	let sim_thread = ::std::thread::spawn(move || {
		let mut last_sync = ::std::time::Instant::now();
		soundlines_core::simulation::run(Some(move |entities: &HashMap<i32, models::Entity>, seeds: &HashMap<i32, models::Seed>| {
			let now = ::std::time::Instant::now();
			if now.duration_since(last_sync).as_secs() > 1 {
				callback_sender.send(Message::ReceiveSeeds(seeds.clone())).expect("Failed to send seeds after simulation step to viz thread");
				callback_sender.send(Message::ReceiveEntities(entities.clone())).expect("Failed to send entities after simulation step to viz thread");
				last_sync = now;
			}

		})).expect("Simulation failed!");
	});

	message_sender.send(Message::FetchCells);

	let state = &mut MainState::new(ctx, message_sender, callback_receiver).unwrap();

	event::run(ctx, state).unwrap();

	// TODO: Find a way to safe quit threads
	sim_thread.join();
	info_thread.join();
}

enum Message {
	FetchCells,
	ReceiveCells(Vec<models::Cell>),
	FetchSeeds,
	ReceiveSeeds(HashMap<i32, models::Seed>),
	ReceiveEntities(HashMap<i32, models::Entity>)
}

fn info_thread(callback_sender: Sender<Message>, message_receiver: Receiver<Message>) -> errors::Result<()> {
	for msg in message_receiver.iter() {
		match msg {
			Message::FetchCells => {
				let conn = db::connect();
				callback_sender.send(Message::ReceiveCells(db::cells::all_vec(&conn)?));
			},
			_  => ()
		}
	}

	Ok(())
}

fn geo_to_point(geo: &[f64; 2]) -> Point2 {
	let x = VIEWPORT_WIDTH as f64 * (geo[0] - MAP_GEO_LEFT) / (MAP_GEO_RIGHT - MAP_GEO_LEFT);
	let y = VIEWPORT_HEIGHT as f64 - VIEWPORT_HEIGHT as f64 * (geo[1] - MAP_GEO_BOTTOM) / (MAP_GEO_TOP - MAP_GEO_BOTTOM);
	return Point2::new(x as f32, y as f32);
}

fn geo_to_polygon(geo: &[[f64; 2]]) -> Vec<Point2> {
	geo.iter().map(geo_to_point).collect()
}