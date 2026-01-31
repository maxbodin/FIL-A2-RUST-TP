use macroquad::prelude::*;
use macroquad::rand::gen_range;

const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 120.0;
const PADDLE_SPEED: f32 = 10.0;

struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
}

impl Ball {
    fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            radius,
            color,
        }
    }

    fn update(&mut self, scores: &mut Vec2) {
        self.position += self.velocity;

        if self.position.x >= screen_width() - self.radius {
            scores.x += 1.0;
            self.reset()
        }

        if self.position.x <= 0.0 {
            scores.y += 1.0;
            self.reset()
        }

        if self.position.y <= 0.0 || self.position.y >= screen_height() - self.radius {
            self.velocity.y *= -1.0
        }
    }

    fn reset(&mut self) {
        self.position = vec2(screen_width() / 2.0, screen_height() / 2.0);
        self.velocity = vec2(gen_range(-5., 5.), gen_range(-5., 5.));
    }
}

#[macroquad::main("Mon Pong!")]
async fn main() {
    let mut l_y = screen_height() / 2.0 - PADDLE_HEIGHT / 2.0;
    let mut r_y = screen_height() / 2.0 - PADDLE_HEIGHT / 2.0;

    let mut balls = Vec::new();
    balls.push(Ball::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        vec2(gen_range(-5., 5.), gen_range(-5., 5.)),
        15.0,
        BLUE,
    ));

    let mut scores: Vec2 = Default::default();

    loop {
        clear_background(LIGHTGRAY);

        // Inputs
        if is_key_down(KeyCode::W) { l_y -= PADDLE_SPEED; }
        if is_key_down(KeyCode::S) { l_y += PADDLE_SPEED; }
        if is_key_down(KeyCode::Up) { r_y -= PADDLE_SPEED; }
        if is_key_down(KeyCode::Down) { r_y += PADDLE_SPEED; }

        r_y = r_y.clamp(0.0, screen_height() - PADDLE_HEIGHT);
        l_y = l_y.clamp(0.0, screen_height() - PADDLE_HEIGHT);

        for ball in &mut balls {
            ball.update(&mut scores);
        }

        // Paddles
        let left_paddle_x = 40.0;
        let right_paddle_x = screen_width() - 40.0 - PADDLE_WIDTH;

        let left_paddle_rect = Rect::new(left_paddle_x, l_y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let right_paddle_rect = Rect::new(right_paddle_x, r_y, PADDLE_WIDTH, PADDLE_HEIGHT);

        // Balls
        let mut new_balls = Vec::new();
        let balls_len = balls.len();

        for ball in &mut balls {
            let ball_rect = Rect::new(
                ball.position.x - ball.radius,
                ball.position.y - ball.radius,
                ball.radius * 2.0,
                ball.radius * 2.0,
            );

            if left_paddle_rect.overlaps(&ball_rect) || right_paddle_rect.overlaps(&ball_rect) {
                ball.velocity.x *= -1.3;

                if balls_len < 100 {
                    new_balls.push(Ball::new(
                        vec2(screen_width() / 2.0, screen_height() / 2.0),
                        vec2(gen_range(-5., 5.), gen_range(-5., 5.)),
                        15.0,
                        RED,
                    ));
                }
            }

            draw_circle(ball.position.x, ball.position.y, ball.radius, ball.color);
        }

        balls.append(&mut new_balls);

        // Drawing
        draw_rectangle(left_paddle_x, l_y, PADDLE_WIDTH, PADDLE_HEIGHT, BLACK);
        draw_rectangle(right_paddle_x, r_y, PADDLE_WIDTH, PADDLE_HEIGHT, BLACK);

        draw_text("MON PONG!", 20.0, 40.0, 40.0, DARKGRAY);
        draw_text(&format!("Score gauche: {}", scores.x), 20.0, 80.0, 40.0, DARKGRAY);
        draw_text(&format!("Score droite: {}", scores.y), 20.0, 120.0, 40.0, DARKGRAY);
        draw_text(&format!("Balls: {}", balls.len()), screen_width() - 200.0, 40.0, 40.0, DARKGRAY);

        next_frame().await
    }
}