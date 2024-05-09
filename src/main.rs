use std::fmt;
use std::fmt::{Display, Formatter, Error};

enum VertDir {
    Up,
    Down
}

enum HoriDir {
    Left,
    Right
}

struct Ball {
    x : i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HoriDir,
}

struct Frame {
   width: i32,
   height: i32, 
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(width: i32, height: i32) -> Game {
        Game {
            frame: Frame {
                width,
                height,
            },
            ball: Ball{
                x: width/2,
                y: height/2,
                vert_dir: VertDir::Down,
                horiz_dir: HoriDir::Right
            }
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HoriDir::Right;
        } else if frame.width <= self.x {
            self.horiz_dir = HoriDir::Left;
        } else if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        } else {

        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HoriDir::Right => self.x +=1,
            HoriDir::Left => self.x -=1,
        }

        match self.vert_dir {
            VertDir::Down => self.y +=1,
            VertDir::Up => self.y -=1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "+");
        for _ in 0..(self.frame.width+1) {
            write!(f, "-");
        }
        write!(f, "+\n");

        for y in 0..(self.frame.height+1) {
            write!(f, "|");
            for x in 0..(self.frame.width+1) {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(f, "O");
                } else  {
                    write!(f, " ");
                }
            }
            write!(f, "|\n");
        }

        write!(f, "+");
        for _ in 0..(self.frame.width+1) {
            write!(f, "-");
        }
        write!(f, "+\n")
    }
}


fn main() {
    let mut new_game = Game::new(63, 31);
    let sleep_time = std::time::Duration::from_millis(10);

    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", new_game.ball.x, new_game.ball.y);
    }
}
