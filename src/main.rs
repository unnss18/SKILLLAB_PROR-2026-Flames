use raylib::prelude::*;

// ─── Constants ────────────────────────────────────────────────────────────────

const SCREEN_W: f32 = 1280.0;
const SCREEN_H: f32 = 720.0;
const TRAIL_LEN: usize = 12;
const SPEED_BOOST_PER_HIT: f32 = 0.18;
const MAX_BALL_SPEED: f32 = 22.0;
const SAMPLE_RATE: u32 = 44100;

// ─── Difficulty ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct DifficultySettings {
    gravity: f32,
    impulse: f32,
    ball_speed: f32,
    paddle_height: f32,
}

impl DifficultySettings {
    fn from(d: Difficulty) -> Self {
        match d {
            Difficulty::Easy => Self {
                gravity: 0.35,
                impulse: -9.0,
                ball_speed: 4.0,
                paddle_height: 120.0,
            },
            Difficulty::Medium => Self {
                gravity: 0.6,
                impulse: -12.0,
                ball_speed: 5.5,
                paddle_height: 90.0,
            },
            Difficulty::Hard => Self {
                gravity: 0.9,
                impulse: -15.0,
                ball_speed: 8.0,
                paddle_height: 60.0,
            },
        }
    }
}

// ─── Game state ───────────────────────────────────────────────────────────────

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}

// ─── Entities ─────────────────────────────────────────────────────────────────

struct Paddle {
	x: f32,
	y: f32,
	vy: f32,
	width: f32,
	height: f32,
	is_left: bool,
	swing: f32,
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    trail: Vec<(f32, f32)>,
    hit_count: u32,
}

impl Ball {
    fn speed(&self) -> f32 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }
    fn push_trail(&mut self) {
        self.trail.insert(0, (self.x, self.y));
        if self.trail.len() > TRAIL_LEN {
            self.trail.pop();
        }
    }
}

// ─── Procedural WAV helpers ───────────────────────────────────────────────────

fn write_u16_le(buf: &mut Vec<u8>, v: u16) {
    buf.push((v & 0xff) as u8);
    buf.push((v >> 8) as u8);
}

fn write_u32_le(buf: &mut Vec<u8>, v: u32) {
    buf.push((v & 0xff) as u8);
    buf.push(((v >> 8) & 0xff) as u8);
    buf.push(((v >> 16) & 0xff) as u8);
    buf.push((v >> 24) as u8);
}

fn gen_sine(freq: f32, dur_s: f32, vol: f32) -> Vec<i16> {
    let n = (SAMPLE_RATE as f32 * dur_s) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            let env = 1.0 - t / dur_s;
            let s = (2.0 * std::f32::consts::PI * freq * t).sin() * vol * env;
            (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
        })
        .collect()
}

fn gen_sweep(f0: f32, f1: f32, dur_s: f32, vol: f32) -> Vec<i16> {
    let n = (SAMPLE_RATE as f32 * dur_s) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            let freq = f0 + (f1 - f0) * (t / dur_s);
            let env = 1.0 - t / dur_s;
            let s = (2.0 * std::f32::consts::PI * freq * t).sin() * vol * env;
            (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
        })
        .collect()
}

fn samples_to_wav(samples: &[i16]) -> Vec<u8> {
    let data_size = (samples.len() * 2) as u32;
    let mut buf: Vec<u8> = Vec::with_capacity(44 + data_size as usize);
    buf.extend_from_slice(b"RIFF");
    write_u32_le(&mut buf, 36 + data_size);
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    write_u32_le(&mut buf, 16);
    write_u16_le(&mut buf, 1);           // PCM
    write_u16_le(&mut buf, 1);           // mono
    write_u32_le(&mut buf, SAMPLE_RATE);
    write_u32_le(&mut buf, SAMPLE_RATE * 2); // byte rate
    write_u16_le(&mut buf, 2);           // block align
    write_u16_le(&mut buf, 16);          // bits per sample
    buf.extend_from_slice(b"data");
    write_u32_le(&mut buf, data_size);
    for &s in samples {
        write_u16_le(&mut buf, s as u16);
    }
    buf
}

// ─── Collision ────────────────────────────────────────────────────────────────

fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    let test_x = center.x.clamp(rec.x, rec.x + rec.width);
    let test_y = center.y.clamp(rec.y, rec.y + rec.height);
    let dx = center.x - test_x;
    let dy = center.y - test_y;
    dx * dx + dy * dy <= radius * radius
}

// ─── Draw helpers ─────────────────────────────────────────────────────────────

fn rot(px: f32, py: f32, cx: f32, cy: f32, a: f32) -> (f32, f32) {
	let (s, c) = a.sin_cos();
	let dx = px - cx;
	let dy = py - cy;
	(cx + dx * c - dy * s, cy + dx * s + dy * c)
}

fn draw_rotated_rect(
	d: &mut RaylibDrawHandle,
	rx: f32, ry: f32, rw: f32, rh: f32,
	ox: f32, oy: f32, angle: f32, color: Color,
) {
	let tl = rot(rx, ry, ox, oy, angle);
	let tr = rot(rx + rw, ry, ox, oy, angle);
	let bl = rot(rx, ry + rh, ox, oy, angle);
	let br = rot(rx + rw, ry + rh, ox, oy, angle);
	d.draw_triangle(
		Vector2::new(tl.0, tl.1),
		Vector2::new(bl.0, bl.1),
		Vector2::new(tr.0, tr.1),
		color,
	);
	d.draw_triangle(
		Vector2::new(tr.0, tr.1),
		Vector2::new(bl.0, bl.1),
		Vector2::new(br.0, br.1),
		color,
	);
}

fn draw_bat(d: &mut RaylibDrawHandle, paddle: &Paddle, color: Color) {
	let cx = paddle.x + paddle.width / 2.0;
	let cy = paddle.y + paddle.height / 2.0;

	let base_tilt: f32 = if paddle.is_left { 0.22 } else { -0.22 };
	let angle = base_tilt + paddle.swing;
	let angle_deg = angle.to_degrees();

	let head_r = paddle.width * 1.6;
	let head_local_y = paddle.y + head_r;
	let (hcx, hcy) = rot(cx, head_local_y, cx, cy, angle);

	d.draw_circle(hcx as i32, hcy as i32, head_r + 3.0, Color::new(25, 25, 25, 255));
	d.draw_circle(hcx as i32, hcy as i32, head_r, color);

	let rubber_base = if paddle.is_left { 270.0 } else { 90.0 };
	d.draw_circle_sector(
		Vector2::new(hcx, hcy),
		head_r - 2.0,
		rubber_base + angle_deg,
		rubber_base + angle_deg + 180.0,
		16,
		Color::new(190, 40, 40, 230),
	);
	d.draw_circle_sector(
		Vector2::new(hcx, hcy),
		head_r - 6.0,
		rubber_base + angle_deg,
		rubber_base + angle_deg + 180.0,
		16,
		Color::new(210, 60, 60, 200),
	);

	let hlx = cx + if paddle.is_left { 6.0 } else { -6.0 };
	let hly = head_local_y - head_r * 0.35;
	let (ghx, ghy) = rot(hlx, hly, cx, cy, angle);
	d.draw_circle(ghx as i32, ghy as i32, 5.0, Color::new(255, 255, 255, 60));
	d.draw_circle(ghx as i32, (ghy + 8.0) as i32, 3.0, Color::new(255, 255, 255, 35));

	let handle_w = paddle.width * 0.5;
	let handle_h = (paddle.height - head_r * 2.0 + 8.0).max(14.0);
	let handle_x = cx - handle_w / 2.0;
	let handle_y = head_local_y + head_r - 6.0;

	draw_rotated_rect(
		d, handle_x, handle_y, handle_w, handle_h,
		cx, cy, angle, Color::new(160, 100, 40, 255),
	);

	let grip_h = 3.0;
	for i in 0..3 {
		let gy = handle_y + 6.0 + i as f32 * 7.0;
		if gy + grip_h < handle_y + handle_h - 2.0 {
			draw_rotated_rect(
				d, handle_x, gy, handle_w, grip_h,
				cx, cy, angle, Color::new(90, 50, 10, 200),
			);
		}
	}

	let knob_x = cx;
	let knob_y = handle_y + handle_h;
	let (kx, ky) = rot(knob_x, knob_y, cx, cy, angle);
	d.draw_circle(kx as i32, ky as i32, handle_w * 0.4, Color::new(130, 80, 30, 255));
}

fn draw_trail(d: &mut RaylibDrawHandle, ball: &Ball) {
    for (i, &(tx, ty)) in ball.trail.iter().enumerate() {
        let alpha = (255.0 * (1.0 - i as f32 / TRAIL_LEN as f32) * 0.5) as u8;
        let r = (ball.radius * (1.0 - i as f32 / TRAIL_LEN as f32 * 0.6)).max(1.0);
        d.draw_circle(tx as i32, ty as i32, r, Color::new(255, 200, 60, alpha));
    }
}

fn draw_center_dashes(d: &mut RaylibDrawHandle) {
    let x = SCREEN_W as i32 / 2;
    let mut y = 0;
    while y < SCREEN_H as i32 {
        d.draw_rectangle(x - 2, y, 4, 18, Color::DARKGRAY);
        y += 30;
    }
}

// ─── Reset helpers ────────────────────────────────────────────────────────────

fn reset_ball(ball: &mut Ball, s: &DifficultySettings, go_left: bool) {
    ball.x = SCREEN_W / 2.0;
    ball.y = SCREEN_H / 2.0;
    ball.vx = if go_left { -s.ball_speed } else { s.ball_speed };
    ball.vy = s.ball_speed * 0.6;
    ball.trail.clear();
    ball.hit_count = 0;
}

fn reset_paddles(p1: &mut Paddle, p2: &mut Paddle, s: &DifficultySettings) {
	p1.height = s.paddle_height;
	p1.y = (SCREEN_H - s.paddle_height) / 2.0;
	p1.vy = 0.0;
	p1.swing = 0.0;
	p2.height = s.paddle_height;
	p2.y = (SCREEN_H - s.paddle_height) / 2.0;
	p2.vy = 0.0;
	p2.swing = 0.0;
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W as i32, SCREEN_H as i32)
        .title("Flappy Pong")
        .build();
    rl.set_target_fps(60);

    // Audio device + procedural sounds
    let mut audio = RaylibAudio::init_audio_device().expect("Failed to init audio device");

    let wav_hit   = samples_to_wav(&gen_sine(480.0, 0.08, 0.65));
    let wav_wall  = samples_to_wav(&gen_sine(220.0, 0.09, 0.45));
    let wav_score = samples_to_wav(&gen_sweep(280.0, 900.0, 0.35, 0.7));
    let wav_menu  = samples_to_wav(&gen_sine(360.0, 0.05, 0.30));

    let wave_hit   = audio.new_wave_from_memory(".wav", &wav_hit).expect("hit wave");
    let wave_wall  = audio.new_wave_from_memory(".wav", &wav_wall).expect("wall wave");
    let wave_score = audio.new_wave_from_memory(".wav", &wav_score).expect("score wave");
    let wave_menu  = audio.new_wave_from_memory(".wav", &wav_menu).expect("menu wave");

    let snd_hit   = audio.new_sound_from_wave(&wave_hit).expect("hit snd");
    let snd_wall  = audio.new_sound_from_wave(&wave_wall).expect("wall snd");
    let snd_score = audio.new_sound_from_wave(&wave_score).expect("score snd");
    let snd_menu  = audio.new_sound_from_wave(&wave_menu).expect("menu snd");

    // Game state
    let mut state = GameState::Menu;
    let mut selected_diff = Difficulty::Medium;
    let mut settings = DifficultySettings::from(selected_diff);

    let mut p1 = Paddle { x: 30.0,  y: 250.0, vy: 0.0, width: 20.0, height: settings.paddle_height, is_left: true, swing: 0.0 };
    let mut p2 = Paddle { x: 1230.0, y: 250.0, vy: 0.0, width: 20.0, height: settings.paddle_height, is_left: false, swing: 0.0 };

    let mut ball = Ball {
        x: SCREEN_W / 2.0, y: SCREEN_H / 2.0,
        vx: settings.ball_speed, vy: settings.ball_speed * 0.6,
        radius: 10.0,
        trail: Vec::with_capacity(TRAIL_LEN),
        hit_count: 0,
    };

    let mut score1: i32 = 0;
    let mut score2: i32 = 0;
    let mut menu_tick: f32 = 0.0;

    // ── Main loop ──────────────────────────────────────────────────────────────
    while !rl.window_should_close() {

        // ── Update ─────────────────────────────────────────────────────────────
        match state {

            GameState::Menu => {
                menu_tick += 0.04;

                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A) {
                    selected_diff = match selected_diff {
                        Difficulty::Medium => Difficulty::Easy,
                        Difficulty::Hard   => Difficulty::Medium,
                        Difficulty::Easy   => Difficulty::Easy,
                    };
                    snd_menu.play();
                }
                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || rl.is_key_pressed(KeyboardKey::KEY_D) {
                    selected_diff = match selected_diff {
                        Difficulty::Easy   => Difficulty::Medium,
                        Difficulty::Medium => Difficulty::Hard,
                        Difficulty::Hard   => Difficulty::Hard,
                    };
                    snd_menu.play();
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    settings = DifficultySettings::from(selected_diff);
                    score1 = 0;
                    score2 = 0;
                    reset_paddles(&mut p1, &mut p2, &settings);
                    reset_ball(&mut ball, &settings, false);
                    snd_menu.play();
                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
                if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
                    p1.vy = settings.impulse;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
                    p2.vy = settings.impulse;
                }

                // Paddle gravity + swing decay
                for p in [&mut p1, &mut p2] {
                    p.vy += settings.gravity;
                    p.y  += p.vy;
                    p.y   = p.y.clamp(0.0, SCREEN_H - p.height);
                    if p.y == 0.0 || p.y == SCREEN_H - p.height {
                        p.vy = 0.0;
                    }
                    p.swing *= 0.86;
                    if p.swing.abs() < 0.01 { p.swing = 0.0; }
                }

                // Ball
                ball.push_trail();
                ball.x += ball.vx;
                ball.y += ball.vy;

                // Wall bounces
                if ball.y - ball.radius <= 0.0 {
                    ball.vy =  ball.vy.abs();
                    ball.y  = ball.radius;
                    snd_wall.play();
                } else if ball.y + ball.radius >= SCREEN_H {
                    ball.vy = -ball.vy.abs();
                    ball.y  = SCREEN_H - ball.radius;
                    snd_wall.play();
                }

                // Paddle hit with spin + rally speed escalation
                let p1_rect  = Rectangle::new(p1.x, p1.y, p1.width, p1.height);
                let p2_rect  = Rectangle::new(p2.x, p2.y, p2.width, p2.height);
                let ball_ctr = Vector2::new(ball.x, ball.y);

                if check_collision_circle_rec(ball_ctr, ball.radius, p1_rect) && ball.vx < 0.0 {
                    ball.hit_count += 1;
                    let offset    = (ball.y - (p1.y + p1.height / 2.0)) / (p1.height / 2.0);
                    let new_speed = (settings.ball_speed
                        * (1.0 + SPEED_BOOST_PER_HIT * ball.hit_count as f32))
                        .min(MAX_BALL_SPEED);
                    ball.vx = new_speed;
                    ball.vy = offset * new_speed * 0.75;
                    ball.x  = p1.x + p1.width + ball.radius + 1.0;
                    p1.swing = -0.45;
                    snd_hit.play();
                }

                if check_collision_circle_rec(ball_ctr, ball.radius, p2_rect) && ball.vx > 0.0 {
                    ball.hit_count += 1;
                    let offset    = (ball.y - (p2.y + p2.height / 2.0)) / (p2.height / 2.0);
                    let new_speed = (settings.ball_speed
                        * (1.0 + SPEED_BOOST_PER_HIT * ball.hit_count as f32))
                        .min(MAX_BALL_SPEED);
                    ball.vx = -new_speed;
                    ball.vy = offset * new_speed * 0.75;
                    ball.x  = p2.x - ball.radius - 1.0;
                    p2.swing = 0.45;
                    snd_hit.play();
                }

                // Scoring
                if ball.x < 0.0 {
                    score2 += 1;
                    reset_ball(&mut ball, &settings, false);
                    snd_score.play();
                } else if ball.x > SCREEN_W {
                    score1 += 1;
                    reset_ball(&mut ball, &settings, true);
                    snd_score.play();
                }

                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    state = GameState::Menu;
                }
            }
        }

        // ── Draw ───────────────────────────────────────────────────────────────
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        match state {

            GameState::Menu => {
                // Animated bg blobs
                for i in 0..6i32 {
                    let ox = ((menu_tick * 0.7 + i as f32 * 1.1).sin() * 120.0 + SCREEN_W / 2.0) as i32;
                    let oy = ((menu_tick * 0.5 + i as f32 * 0.9).cos() *  80.0 + SCREEN_H / 2.0) as i32;
                    d.draw_circle(ox, oy, 65.0, Color::new(20, 40, 90, 55));
                }

                d.draw_text("FLAPPY PONG", 395, 80, 66, Color::WHITE);

                // Controls panel
                d.draw_text("Controls",                                    50, 190, 22, Color::LIGHTGRAY);
                d.draw_text("P1 (blue bat)   :  TAB to flap",             50, 216, 18, Color::GRAY);
                d.draw_text("P2 (green bat)  :  LEFT SHIFT to flap",      50, 238, 18, Color::GRAY);
                d.draw_text("Ball speeds up with every rally — survive!", 50, 264, 17, Color::new(255, 175, 50, 255));

                // Difficulty selector
                d.draw_text("Difficulty  <-- / -->",                       50, 310, 22, Color::LIGHTGRAY);

                let diffs: &[(Difficulty, &str, i32)] = &[
                    (Difficulty::Easy,   "EASY",   358),
                    (Difficulty::Medium, "MEDIUM", 553),
                    (Difficulty::Hard,   "HARD",   748),
                ];

                for &(diff, label, bx) in diffs {
                    let selected = selected_diff == diff;
                    let bg  = if selected { Color::new(30, 100, 200, 210) } else { Color::new(35, 35, 35, 180) };
                    let col = if selected { Color::WHITE } else { Color::DARKGRAY };
                    d.draw_rectangle(bx, 342, 175, 55, bg);
                    if selected {
                        d.draw_rectangle_lines(bx, 342, 175, 55, Color::WHITE);
                    }
                    let tx = bx + 87 - (label.len() as i32 * 7);
                    d.draw_text(label, tx, 360, 22, col);
                }

                // Difficulty stat readout
                let ds = DifficultySettings::from(selected_diff);
                let stats = format!(
                    "gravity {:.2}  |  impulse {:.1}  |  ball speed {:.1}  |  paddle h {}",
                    ds.gravity, -ds.impulse, ds.ball_speed, ds.paddle_height as i32
                );
                d.draw_text(&stats, 50, 408, 15, Color::GRAY);

                // Pulsing start prompt
                let alpha = (185.0 + ((menu_tick * 3.0).sin() * 0.5 + 0.5) * 70.0) as u8;
                d.draw_text("PRESS ENTER to Start",     468, 468, 26, Color::new(255, 255, 255, alpha));
                d.draw_text("ESC in-game returns here", 510, 500, 15, Color::DARKGRAY);
            }

            GameState::Playing => {
                draw_center_dashes(&mut d);
                draw_trail(&mut d, &ball);

                // Ball with speed-based glow
                let t = (ball.speed() / MAX_BALL_SPEED).min(1.0);
                let gr = (255.0 * t) as u8;
                let gg = (200.0 * (1.0 - t)) as u8;
                d.draw_circle(ball.x as i32, ball.y as i32, ball.radius + 4.0, Color::new(gr, gg, 0, 55));
                d.draw_circle(ball.x as i32, ball.y as i32, ball.radius, Color::WHITE);

                // Table-tennis bats
                draw_bat(&mut d, &p1, Color::new(60, 150, 255, 255));  // blue
                draw_bat(&mut d, &p2, Color::new(60, 210,  90, 255));  // green

                // Scores
                d.draw_text(&score1.to_string(), (SCREEN_W / 4.0)  as i32, 18, 52, Color::GRAY);
                d.draw_text(&score2.to_string(), (SCREEN_W * 0.75) as i32, 18, 52, Color::GRAY);

                // Rally heat indicator
                if ball.hit_count >= 4 {
                    let label = format!("RALLY x{}", ball.hit_count);
                    let lx = SCREEN_W as i32 / 2 - (label.len() as i32 * 5);
                    d.draw_text(&label, lx, SCREEN_H as i32 - 30, 18, Color::new(255, 140, 0, 220));
                }

                d.draw_text("ESC = Menu", 8, SCREEN_H as i32 - 22, 14, Color::new(90, 90, 90, 180));
            }
        }
    }
}