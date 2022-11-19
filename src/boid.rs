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
            vel: Vec2::new(rand::gen_range(-1f32, 1f32), rand::gen_range(-1f32, 1f32)).normalize(),
            acl: Vec2::new(0f32, 0f32),
            size: 5f32,
            rad: 25f32
        }
    }

    pub fn flock(&mut self, alignment: Vec2, cohesion: Vec2, seperation: Vec2){
        self.acl += alignment;
        self.acl += cohesion;
        self.acl += seperation;
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


pub fn get_align(b: &Boid, others: &Vec<Boid>) -> Vec2{
    let mut align = Vec2::new(0f32, 0f32);
    let mut total = 0;

    for i in 0..others.len(){
        let d = b.pos.distance(others[i].pos);
        if (&others[i] != b) && (d < b.rad){
            align += others[i].vel;
            total += 1;
        }
    }
    if total > 0{
        align = align / Vec2::new(total as f32, total as f32);
        align = align.clamp_length(MAX_SPEED, MAX_SPEED);
        align -= b.vel;
        align = align.clamp_length_max(MAX_STEER);
    }
    return align;
}

pub fn get_co(b: &Boid, others: &Vec<Boid>) -> Vec2{
    let mut avg = Vec2::new(0f32, 0f32);
    let mut total = 0;
    for i in 0..others.len(){
        if(&others[i] != b) && (b.pos.distance(others[i].pos) < b.rad * 2f32){
            avg += others[i].pos;
            total += 1 
        }
    }
    if total > 0{
        avg = avg / Vec2::new(total as f32, total as f32);
        avg = avg.clamp_length(MAX_SPEED,MAX_SPEED);
        avg -= b.pos;
        avg -= b.vel;
        avg = avg.clamp_length_max(MAX_STEER);
    }
    return avg;
}

pub fn get_sep(b: &Boid, others: &Vec<Boid>) -> Vec2{
    let mut avg = Vec2::new(0f32, 0f32);
    let mut total = 0;
    for i in 0..others.len(){
        let d = b.pos.distance(others[i].pos);
        if(&others[i] != b) && d < b.rad{
            let mut diff = b.pos - others[i].pos;    
            diff = diff / d.powf(2f32);
            avg += diff;
            total += 1 
        }
    }
    if total > 0{
        avg = avg / Vec2::new(total as f32, total as f32);
        avg = avg.clamp_length(MAX_SPEED,MAX_SPEED);
        avg -= b.vel;
        avg = avg.clamp_length_max(MAX_STEER);
    }
    return avg;
}
