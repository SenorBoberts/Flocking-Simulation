use macroquad::prelude::*;

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
            size: 10f32,
            rad: 50f32
        }
    }

    pub fn align(&mut self, steer: Vec2){
        self.acl = steer;
    }

    pub fn check_edges(&mut self){
        if self.pos.x > screen_width() {
            self.pos.x = 0f32;
        }
        if self.pos.x < 0f32 {
            self.pos.x = screen_width();
        }
        if self.pos.y > screen_height(){
            self.pos.y = 0f32;
        }
        if self.pos.y < 0f32 {
            self.pos.y = screen_height();
        }
    }
    
    pub fn update(&mut self){
        self.check_edges();
        self.pos += self.vel;
        self.vel += self.acl;
    }
    
    pub fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, self.size, WHITE);
    }
}


    pub fn get_steer(b: &Boid, others: &Vec<Boid>) -> Vec2{
        let mut avg = Vec2::new(0f32, 0f32);
        let mut total = 0;
        for i in 0..others.len(){
            if (&others[i] != b) && (b.pos.distance(others[i].pos) < b.rad){
                avg += others[i].vel;
                total += 1;
            }
        }
        if total > 0{
            avg = avg / Vec2::new(total as f32, total as f32);
            avg -= b.vel;
            //add a limit
        }
        return avg;
    }
