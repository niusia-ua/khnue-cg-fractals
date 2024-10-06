use nannou::prelude::*;

fn main() {
  nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
  // Prepare to draw.
  let draw = app.draw();
  draw.background().color(WHITE);

  // Define the depth of the curve.
  let depth = 6;

  // Draw the curve.
  draw_segment(&draw, pt2(-400.0, -100.0), pt2(400.0, -100.0), depth);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn draw_segment(draw: &Draw, p1: Point2, p2: Point2, depth: i32) {
  if depth == 0 {
    draw.line().color(BLACK).points(p1, p2);
  } else {
    // Calculate the points for the four segments.
    let length = (p2 - p1).length();
    let dir = (p2 - p1).normalize();
    let p12 = p1 + dir * length / 3.0;
    let p21 = p1 + dir * length * 2.0 / 3.0;
    let p3 = p12 + dir.rotate(PI / 3.0) * length / 3.0;

    // Recursively draw the smaller segments.
    draw_segment(draw, p1, p12, depth - 1);
    draw_segment(draw, p12, p3, depth - 1);
    draw_segment(draw, p3, p21, depth - 1);
    draw_segment(draw, p21, p2, depth - 1);
  }
}
