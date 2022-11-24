use macroquad::prelude::*;
use std::f32::consts::PI;

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
            vel: Vec2::new(rand::gen_range(-4f32, 4f32), rand::gen_range(-4f32, 4f32)),
            acl: Vec2::new(0f32, 0f32),
            size: 10f32,
            rad: 15f32
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
        let test_vec = self.vel.normalize();
        let rot_vec = Vec2::from_angle(PI * (7f32/8f32)).rotate(test_vec);
        let rot_vec2 = Vec2::from_angle(PI * (9f32/8f32)).rotate(test_vec);
        draw_triangle(self.pos, self.pos + rot_vec * self.size, self.pos + rot_vec2 * self.size, WHITE);
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
        align = align * (MAX_SPEED / align.length());
        align -= b.vel;
        align = align.clamp_length_max(MAX_STEER);

        sep = sep / Vec2::new(total as f32, total as f32);
        sep = sep * (MAX_SPEED / sep.length());
        sep -= b.vel;
        sep = sep.clamp_length_max(MAX_STEER);
    }

    if co_total > 0{
        co = co / Vec2::new(co_total as f32, co_total as f32);
        co = co * (MAX_SPEED / co.length());
        co -= b.pos;
        co -= b.vel;
        co = co.clamp_length_max(MAX_STEER);
    }
    return align + sep + co;
}

