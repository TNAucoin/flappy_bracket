use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 30.0 / 1000.0;

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

struct Obstacle {
    x: i32,
    width: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            width: random.range(3, 5 + score / 5),
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Draw top half of obstacle
        for y in 0..self.gap_y - half_size {
            for width in 0..self.width {
                ctx.set(screen_x + width, y, WHITE, BLACK, to_cp437('|'));
            }
            ctx.set(screen_x, y, WHITE, BLACK, to_cp437('|'));
        }

        // Draw bottom half of obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            for width in 0..self.width {
                ctx.set(screen_x + width, y, WHITE, BLACK, to_cp437('|'));
            }
        }
    }

    fn hit_test(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        player.x <= self.x + self.width
            && player.x >= self.x
            && (player.y < self.gap_y - half_size || player.y > self.gap_y + half_size)
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    score: i32,
    obstacle: Obstacle,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }
    fn restart_game(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy Bracket");
        ctx.print_centered(7, "Press [Enter] to start");
        ctx.print_centered(8, "Press [Q] to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.restart_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn playing(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press [Space] to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        // if the player passed the obstacle, increase the score and spawn a new obstacle
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        // if the player hits the obstacle or the ground, end the game
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_test(&self.player) {
            self.mode = GameMode::GameOver;
        }
    }
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(7, "Press [Enter] to restart");
        ctx.print_centered(8, "Press [Q] to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.restart_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(5, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity <= 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::GameOver => self.game_over(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Bracket")
        .build()?;
    main_loop(context, State::new())
}
