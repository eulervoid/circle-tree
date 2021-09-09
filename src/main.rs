mod raycast;
mod tree;
use nannou::prelude::*;
use raycast::Segment;
use tree::*;
const FRAMES_PER_CYCLE: f32 = 1200.;

struct Model {
    pub config: RandomTreeConfig,
    pub tree_a: Tree<f32>,
    pub tree_b: Tree<f32>,
}

fn model(app: &App) -> Model {
    let _w = app
        .new_window()
        .key_pressed(key_pressed)
        .size(1920, 1080)
        .msaa_samples(4)
        .view(view)
        .build()
        .unwrap();
    let tree_config = RandomTreeConfig {
        depth: 7,
        min_children: 2,
        max_children: 3,
        max_offset: 360f32.to_radians(),
    };
    let mut rng = rand::thread_rng();
    Model {
        config: tree_config,
        tree_a: Tree::<f32>::random(&mut rng, &tree_config),
        tree_b: Tree::<f32>::random(&mut rng, &tree_config),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb(0., 0., 0.));

    let radius = app.window_rect().w().min(app.window_rect().h()) / 2. - 30.;
    let phase = (app.elapsed_frames() as f32 / FRAMES_PER_CYCLE) * PI * 2.0;
    let scale = Affine2::from_scale(Vec2::splat(radius));
    let b_scale = Affine2::from_scale(app.window_rect().wh() / 2.0 - Vec2::splat(10.0));
    let bounds: Vec<Segment> = default_bounds()
        .iter()
        .map(|s| s.transform(b_scale))
        .collect();
    let segments_a = division_lines(&model.tree_a, scale, phase, &bounds);
    // let segments_b = division_lines(&model.tree_b, scale, phase, &bounds);

    view_segments(&draw, &segments_a, GREENYELLOW);
    // view_segments(&draw, &segments_b, LIGHTCORAL);
    // view_segments(&draw, &bounds, GREENYELLOW);

    draw.to_frame(app, &frame).unwrap();

    // uncomment to record looping video frames
    if app.elapsed_frames() <= FRAMES_PER_CYCLE.floor() as u64 {
        save_frame(app, &frame);
    } else {
        app.quit();
    }
}

fn division_lines(
    tree: &Tree<f32>,
    transform: Affine2,
    phase: f32,
    bounds: &[Segment],
) -> Vec<Segment> {
    let n = tree.children.len();
    let scaling = match n {
        2 => 1. / 2.0,
        3 => 1. / (1. + 2. / 3f32.sqrt()),
        4 => 1. / (1. + 2f32.sqrt()),
        5 => 1. / (1. + (2. * (1. + 1. / 5f32.sqrt())).sqrt()),
        _ => 0.,
    };

    let mut segments = vec![];

    let transforms: Vec<Affine2> = (0..n)
        .map(|i| {
            let angle: f32 = i as f32 * (TAU / n as f32) + phase * tree.value;
            let translation = Vec2::new(1., 0.) * (1. - scaling);
            let transform = transform * Affine2::from_angle(angle);
            let segment = Segment::new((0., 0.).into(), translation).transform(transform);
            if let Some(intersection) = segment.to_ray().intersect_first(&bounds) {
                segments.push(Segment::new(segment.a, intersection.point));
            }
            let transform = transform
                * Affine2::from_angle(PI / n as f32)
                * Affine2::from_translation(translation)
                * Affine2::from_scale(Vec2::splat(scaling * 0.99));
            transform
        })
        .collect();
    let new_bounds: Vec<Segment> = segments
        .iter()
        .cloned()
        .chain(bounds.iter().cloned())
        .collect();
    for (child, transform) in tree.children.iter().zip(transforms) {
        segments.append(&mut division_lines(child, transform, phase, &new_bounds));
    }
    segments
}

fn default_bounds() -> Vec<Segment> {
    let bounds_a: Vec<(f32, f32)> = vec![(-1., -1.), (-1., 1.), (1., 1.), (1., -1.)];
    let mut bounds_b = bounds_a.clone();
    bounds_b.rotate_right(1);
    bounds_a
        .into_iter()
        .zip(bounds_b.into_iter())
        .map(|(a, b)| Segment::new(a.into(), b.into()))
        .collect()
}

fn view_segments(draw: &Draw, segments: &Vec<Segment>, color: Rgb<u8>) {
    for segment in segments.iter() {
        draw.line()
            .start(segment.a)
            .end(segment.b)
            .color(color)
            .stroke_weight(2.0);
    }
}

fn main() {
    nannou::app(model).view(view).run();
}

fn save_frame(app: &App, frame: &Frame) {
    let file_path = app
        .project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("frame-{:04}", frame.nth()))
        .with_extension("jpeg");
    app.main_window().capture_frame(file_path);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            let mut rng = rand::thread_rng();
            model.tree_a = Tree::<f32>::random(&mut rng, &model.config);
            model.tree_b = Tree::<f32>::random(&mut rng, &model.config);
        }
        Key::R => {
            //model.save_frame = !model.save_frame;
        }
        _ => {}
    }
}
