use macroquad::prelude::*;

mod boid;

const NUM_AGENTS : usize = 100;

#[macroquad::main("flock")]
async fn main() {
    //let mut s = Simulation::new();
    let mut flock = vec!();
    for _i in 0..NUM_AGENTS{
        flock.push(boid::Boid::new());
    }
    loop{    
        for i in 0..NUM_AGENTS{
            let steer = boid::get_steer(&flock[i], &flock);
            flock[i].align(steer);
            flock[i].update();
            flock[i].draw();
        }
        next_frame().await
    }
}


