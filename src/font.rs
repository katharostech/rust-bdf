use std::collections::HashMap;

use {Glyph, Property, BoundingBox};

/// Size of a font.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Size {
	/// Point size of the font.
	pub pt: u16,

	/// X-axis DPI.
	pub x: u16,

	/// Y-axis DPI.
	pub y: u16,
}

impl Default for Size {
	fn default() -> Self {
		Size {
			pt: 0,

			x: 0,
			y: 0,
		}
	}
}

/// A BDF font.
#[derive(Debug)]
pub struct Font {
	format: String,

	name:    String,
	version: Option<String>,

	size:   Size,
	len:    usize,
	bounds: Option<BoundingBox>,

	comments:   Vec<String>,
	properties: HashMap<String, Property>,
	glyphs:     HashMap<char, Glyph>,
}

impl Default for Font {
	fn default() -> Self {
		Font {
			format: "2.2".to_owned(),

			name:    "--------------".to_owned(),
			version: None,

			size:   Default::default(),
			len:    0,
			bounds: Default::default(),

			comments:   Vec::new(),
			properties: HashMap::new(),
			glyphs:     HashMap::new(),
		}
	}
}

impl Font {
	/// Create a new font with the given name and content-version.
	pub fn new<T: Into<String>>(name: T, version: Option<T>) -> Self {
		Font {
			name:    name.into(),
			version: version.map(|v| v.into()),

			.. Default::default()
		}
	}

	/// Gets BDF format version.
	pub fn format(&self) -> &str {
		&self.format
	}

	/// Sets the BDF format version.
	pub fn set_format<T: Into<String>>(&mut self, format: T) {
		self.format = format.into();
	}

	/// Gets the name.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Sets the name.
	pub fn set_name<T: Into<String>>(&mut self, name: T) {
		self.name = name.into();
	}

	/// Gets the content-version.
	pub fn version(&self) -> Option<&str> {
		self.version.as_ref().map(|v| v.as_ref())
	}

	/// Sets the content-version.
	pub fn set_version<T: Into<String>>(&mut self, version: Option<T>) {
		self.version = version.map(|v| v.into());
	}

	/// Gets the size.
	pub fn size(&self) -> &Size {
		&self.size
	}

	/// Sets the size.
	pub fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	/// Gets the number of characters stored.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Sets the number of characters stored.
	pub fn set_len(&mut self, len: usize) {
		self.len = len;
	}

	/// Gets the default bounding box.
	pub fn bounds(&self) -> Option<&BoundingBox> {
		self.bounds.as_ref()
	}

	/// Sets the default bounding box.
	pub fn set_bounds(&mut self, bounds: Option<BoundingBox>) {
		self.bounds = bounds;
	}

	/// Gets the comments.
	pub fn comments(&self) -> &Vec<String> {
		&self.comments
	}

	/// Gets a mutable reference to the comments.
	pub fn comments_mut(&mut self) -> &mut Vec<String> {
		&mut self.comments
	}

	/// Gets the properties.
	pub fn properties(&self) -> &HashMap<String, Property> {
		&self.properties
	}

	/// Gets a mutable reference to the properties.
	pub fn properties_mut(&mut self) -> &mut HashMap<String, Property> {
		&mut self.properties
	}

	/// Gets the glyphs.
	pub fn glyphs(&self) -> &HashMap<char, Glyph> {
		&self.glyphs
	}

	/// Gets a mutable reference to the glyphs.
	pub fn glyphs_mut(&mut self) -> &mut HashMap<char, Glyph> {
		&mut self.glyphs
	}
}
