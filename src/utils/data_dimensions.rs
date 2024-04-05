pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub depth: u32,

    pub pixel_spacing: [f32; 2],
    pub distance_between_slices: f32
}
impl Dimensions {
    pub fn builder() -> DimensionsBuilder {
        DimensionsBuilder::new()
    }

    pub fn get_dimensions(&self) -> [f32; 3] {
        [self.width as f32 * self.pixel_spacing[0], self.height as f32 * self.pixel_spacing[1], self.depth as f32 * self.distance_between_slices]
    }

    pub fn get_normalized_dimensions(&self) -> [f32; 3] {
        let lengths = [
            self.width as f32 * self.pixel_spacing[0],
            self.height as f32 * self.pixel_spacing[1],
            self.depth as f32 * self.distance_between_slices
        ];

        let max_length = lengths.iter().fold(0.0f32, |acc, &x| acc.max(x));

        lengths.map(|x| x / max_length)
    }
}

pub struct DimensionsBuilder {
    height: u32,
    width: u32,
    depth: u32,
    pixel_spacing: [f32; 2],
    distance_between_slices: f32
}

impl DimensionsBuilder {
    pub fn new() -> DimensionsBuilder {
        DimensionsBuilder {
            height: 1,
            width: 1,
            depth: 1,
            pixel_spacing: [1.0, 1.0],
            distance_between_slices: 1.0,
        }
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub fn pixel_spacing(mut self, pixel_spacing: [f32; 2]) -> Self {
        self.pixel_spacing = pixel_spacing;
        self
    }

    pub fn distance_between_slices(mut self, distance_between_slices: f32) -> Self {
        self.distance_between_slices = distance_between_slices;
        self
    }

    pub fn build(&self) -> Dimensions {
        Dimensions {
            height: self.height,
            width: self.width,
            depth: self.depth,
            pixel_spacing: self.pixel_spacing,
            distance_between_slices: self.distance_between_slices
        }
    }
}