use vizia::prelude::*;
use vizia::vg;
#[derive(Debug)]
pub enum AppEvent {
    SetHistogramTime(f32),
}

#[derive(Lens)]
pub struct AppData {
    histogram_data: HistogramData,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::SetHistogramTime(histogram_time) => {
                self.histogram_data.histogram_time= *histogram_time;
            }
        });
    }
}

#[derive(Clone, Data, Lens)]
pub struct HistogramData {
    histogram_time: f32,
}


pub struct HistogramGraph<HistogramDataL: Lens<Target = HistogramData>>{
    histogram_data: HistogramDataL,
}

impl<HistogramDataL: Lens<Target = HistogramData>> HistogramGraph<HistogramDataL> {
    pub fn new(cx: &mut Context, histogram_data: HistogramDataL) -> Handle<Self> {
        Self {
            histogram_data,
        }.build(cx, |cx|{
            // If we want the view to contain other views we can build those here.
        })
        // Redraw when lensed data changes
         .bind(histogram_data, |mut handle, _| handle.needs_redraw())
    }
}


impl<HistogramDataL: Lens<Target = HistogramData>> View for HistogramGraph<HistogramDataL> {
    // for css:
    fn element(&self) -> Option<&'static str> {
        Some("HistogramGraph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        // Get the bounding box of the current view.
        let bounds = cx.bounds();
        // let histogram_time = self.histogram_data.get(cx).histogram_time;
        // let bins = self.histogram_data.get(cx).bins;

        let mut bins: [f32; NR_BINS] = [0.0; NR_BINS];

        let nr_tests = NR_BINS * 32;
        let db_min = DB_MIN - 1.0;
        let db_max = DB_MAX + 1.0;


        let mut rng = XorShiftRng::new(42); // Initialize with a seed

        for _ in 0..nr_tests {
            let db_value = db_min + rng.next() * (db_max - db_min);
            let bin_index = find_bin(db_to_linear(db_value));
            bins[bin_index] += 1.0; // Increment the count for the bin
        }

        let largest = bins.iter().fold(std::f32::MIN, |a,b| a.max(*b));
        for i in 0..NR_BINS {
            bins[i] /= largest;
        }

        let line_width = 2.5;

        // Create a new `Path` from the `vg` module.
        let mut path = vg::Path::new();
        let x = bounds.x + line_width/2.0;
        let y = bounds.y + line_width/2.0;
        let w = bounds.w - line_width;
        let h = bounds.h - line_width;

        // Add a rectangle to the path with the dimensions of the view bounds.
        path.rect(x, y, w, h);
        canvas.fill_path(&mut path, &vg::Paint::color(Color::white().into()));
        canvas.stroke_path(
            &{
                let mut path = vg::Path::new();
                // start of the graph
                path.move_to(x+bins[0]*w, y);
                for i in 1..NR_BINS {
                    path.line_to(x+bins[i]*w,y+h*i as f32/(NR_BINS-1) as f32);
                }
                path
            },
            &vg::Paint::color(Color::black().into())
                .with_line_width(line_width),
        );
    }
}

const DB_MIN: f32 = -96.0;
const DB_MAX: f32 = 24.0;
const NR_BINS: usize = 255;
// const NR_BINS: usize = 32;

fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

// Function to find the bin for a given linear audio value
fn find_bin(value: f32) -> usize {
    // calculate the linear edges from DB_MIN to DB_MAX, evenly spaced in the db domain
    const NR_EDGES: usize = NR_BINS - 1;
    let edges: [f32; NR_EDGES] = (0..NR_EDGES)
        .map(|x| db_to_linear(DB_MIN + x as f32 * ((DB_MAX - DB_MIN) / (NR_EDGES as f32 - 1.0))))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // Check if the value is smaller than the first edge
    if value < edges[0] {
        return 0;
    }
    // Check if the value is larger than the last edge
    if value > *edges.last().unwrap() {
        return edges.len() as usize;
    }
    // Binary search to find the bin for the given value
    let mut left = 0;
    let mut right = edges.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if value >= edges[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    // Return the bin index
    left as usize
}

struct XorShiftRng {
    state: u64,
}
// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
//
impl XorShiftRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> f32 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        (self.state & 0x7fffffff as u64) as f32 % 33.33 / 33.33
    }
}

fn main() {
    Application::new(|cx| {
        VStack::new(cx, |cx|{
            // Slider::new(cx, AppData::histogram_data.then(HistogramData::histogram_time))
            // .range(0.0..50.0)
            // .on_changing(|cx, val| {cx.emit(AppEvent::SetHistogramTime(val));
            // });
            // Label::new(cx, AppData::histogram_data.then(HistogramData::histogram_time).map(|val| format!("{:.2}", val)));
            HistogramGraph::new(cx, AppData::histogram_data);
        });

    })
        .run();
}
