use raylib::prelude::*;

struct Paddle {
	x: f32,
	y: f32,
	vy: f32,
	width: f32,
	height: f32,
}

struct Ball {
	x: f32,
	y: f32,
	vx: f32,
	vy: f32,
	radius: f32,
}

fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
	let test_x = center.x.clamp(rec.x, rec.x + rec.width);
	let test_y = center.y.clamp(rec.y, rec.y + rec.height);
	let dist_x = center.x - test_x;
	let dist_y = center.y - test_y;
	
	(dist_x * dist_x) + (dist_y * dist_y) <= (radius * radius)
}

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(800, 600)
		.title("Flappy Pong")
		.build();

	rl.set_target_fps(60);

	let gravity = 0.6;
	let impulse = -12.0;
	let screen_h = 600.0;
	let screen_w = 800.0;

	let mut p1 = Paddle { x: 30.0, y: 250.0, vy: 0.0, width: 20.0, height: 100.0 };
	let mut p2 = Paddle { x: 750.0, y: 250.0, vy: 0.0, width: 20.0, height: 100.0 };

	let mut ball = Ball { x: 400.0, y: 300.0, vx: 5.0, vy: 5.0, radius: 10.0 };

	let mut score1 = 0;
	let mut score2 = 0;

	while !rl.window_should_close() {
		if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
			p1.vy = impulse;
		}
		if rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
			p2.vy = impulse;
		}

		p1.vy += gravity;
		p1.y += p1.vy;
		p2.vy += gravity;
		p2.y += p2.vy;

		if p1.y <= 0.0 {
			p1.y = 0.0;
			p1.vy = 0.0;
		} else if p1.y >= screen_h - p1.height {
			p1.y = screen_h - p1.height;
			p1.vy = 0.0;
		}

		if p2.y <= 0.0 {
			p2.y = 0.0;
			p2.vy = 0.0;
		} else if p2.y >= screen_h - p2.height {
			p2.y = screen_h - p2.height;
			p2.vy = 0.0;
		}

		ball.x += ball.vx;
		ball.y += ball.vy;

		if ball.y - ball.radius <= 0.0 || ball.y + ball.radius >= screen_h {
			ball.vy *= -1.0;
		}

		let p1_rect = Rectangle::new(p1.x, p1.y, p1.width, p1.height);
		let p2_rect = Rectangle::new(p2.x, p2.y, p2.width, p2.height);
		let ball_center = Vector2::new(ball.x, ball.y);

		if check_collision_circle_rec(ball_center, ball.radius, p1_rect) {
			ball.vx *= -1.0;
			ball.x = p1.x + p1.width + ball.radius;
		}

		if check_collision_circle_rec(ball_center, ball.radius, p2_rect) {
			ball.vx *= -1.0;
			ball.x = p2.x - ball.radius;
		}

		if ball.x < 0.0 {
			score2 += 1;
			ball.x = screen_w / 2.0;
			ball.y = screen_h / 2.0;
			ball.vx = 5.0;
		} else if ball.x > screen_w {
			score1 += 1;
			ball.x = screen_w / 2.0;
			ball.y = screen_h / 2.0;
			ball.vx = -5.0;
		}

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);

		d.draw_line(screen_w as i32 / 2, 0, screen_w as i32 / 2, screen_h as i32, Color::DARKGRAY);

		d.draw_rectangle(p1.x as i32, p1.y as i32, p1.width as i32, p1.height as i32, Color::WHITE);
		d.draw_rectangle(p2.x as i32, p2.y as i32, p2.width as i32, p2.height as i32, Color::WHITE);

		d.draw_circle(ball.x as i32, ball.y as i32, ball.radius, Color::WHITE);

		d.draw_text(&score1.to_string(), (screen_w / 4.0) as i32, 20, 40, Color::GRAY);
		d.draw_text(&score2.to_string(), (screen_w * 0.75) as i32, 20, 40, Color::GRAY);
	}
}