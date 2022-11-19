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
        let cflock = flock.clone();
        for i in 0..NUM_AGENTS{
            let align = boid::get_align(&flock[i], &cflock);
            let co = boid::get_co(&flock[i], &cflock);
            let sep = boid::get_sep(&flock[i], &cflock);
            flock[i].flock(align, co, sep);
            flock[i].update();
            flock[i].draw();
        }
        next_frame().await
    }
}


