use crate::MusicToolbar;

use gtk::{Adjustment, Image, Label, Window};
use gtk::prelude::*;

#[derive(Clone)]
pub struct MusicWindow {
	pub toolbar: MusicToolbar,
	pub cover: Image,
	pub adjustment: Adjustment,
	pub duration_label: Label,
	pub play: Image,
	pub pause: Image,
	pub window: Window,
	pub is_playing: bool,
}