use macroquad::prelude::*;

const MAX_STEER: f32 = 0.1;
const MAX_SPEED: f32 = 4f32;

#[derive (PartialEq, Clone, Copy)]
pub struct Boid{
    pos: Vec2,
    vel: Vec2,
    acl: Vec2,
    size: f32,
    rad: f32
}

impl Boid{
    pub fn new() -> Self{
        Self{
            pos: Vec2::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())),
            vel: Vec2::new(rand::gen_range(-4f32, 4f32), rand::gen_range(-4f32, 4f32)).normalize(),
            acl: Vec2::new(0f32, 0f32),
            size: 5f32,
            rad: 25f32
        }
    }

    pub fn flock(&mut self, steer: Vec2){
        self.acl += steer;
    }

    pub fn check_edges(&mut self){
        if self.pos.x > screen_width() + self.size{
            self.pos.x = 0f32 - self.size;
        }
        if self.pos.x < 0f32 - self.size{
            self.pos.x = screen_width() + self.size;
        }
        if self.pos.y > screen_height() + self.size{
            self.pos.y = 0f32 - self.size;
        }
        if self.pos.y < 0f32 - self.size{
            self.pos.y = screen_height() + self.size;
        }
    }
    
    pub fn update(&mut self){
        self.check_edges();
        self.pos += self.vel;
        self.vel += self.acl;
        self.vel = self.vel.clamp_length_max(MAX_SPEED); 
        self.acl = Vec2::new(0f32, 0f32);
    }
    
    pub fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, self.size, WHITE);
    }
}


pub fn get_steer(b: &Boid, others: &Vec<Boid>) -> Vec2{
    let mut align = Vec2::new(0f32, 0f32);
    let mut co = Vec2::new(0f32, 0f32);
    let mut sep = Vec2::new(0f32, 0f32);
    let mut total = 0;
    let mut co_total = 0;

    for i in 0..others.len(){
        let d = b.pos.distance(others[i].pos);
        if (&others[i] != b) && (d < b.rad * 2f32){
            if d < b.rad {
                align += others[i].vel;
                let mut diff = b.pos - others[i].pos;    
                diff = diff / d.powf(2f32);
                sep += diff;
                total += 1;
            }
            co += others[i].pos;
            co_total += 1;
        }
    }
    if total > 0{
        align = align / Vec2::new(total as f32, total as f32);
        align = align.clamp_length(MAX_SPEED, MAX_SPEED);
        align -= b.vel;
        align = align.clamp_length_max(MAX_STEER);

        sep = sep / Vec2::new(total as f32, total as f32);
        sep = sep.clamp_length(MAX_SPEED,MAX_SPEED);
        sep -= b.vel;
        sep = sep.clamp_length_max(MAX_STEER);
    }

    if co_total > 0{
        co = co / Vec2::new(co_total as f32, co_total as f32);
        co = co.clamp_length(MAX_SPEED,MAX_SPEED);
        co -= b.pos;
        co -= b.vel;
        co = co.clamp_length_max(MAX_STEER);
    }
    return align + sep + co;
}

