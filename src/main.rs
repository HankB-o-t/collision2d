use macroquad::prelude::*;
mod player;
mod obs;

#[macroquad::main("Survive game")]
async fn main() {
    let sw: f32 = screen_width() / 2.0;
    let sh: f32 = screen_height() / 2.0;

    let obs1 = obs::Obs::new(sw + 50.0, sh + 50.0);
    let obs2 = obs::Obs::new(sw - 50.0, sh - 50.0);
    let obs3 = obs::Obs::new(sw + 50.0, sh - 50.0);
    let obs4 = obs::Obs::new(sw - 50.0, sh + 50.0);

    let obs5 = obs::Obs::new(sw - 250.0, sh + 50.0);
    let obs6 = obs::Obs::new(sw - 150.0, sh - 50.0);
    let obs7 = obs::Obs::new(sw - 250.0, sh - 50.0);
    let obs8 = obs::Obs::new(sw - 150.0, sh + 50.0);
    

    let obs9  = obs::Obs::new(sw - 50.0, sh - 250.0);
    let obs10 = obs::Obs::new(sw + 50.0, sh - 150.0);
    let obs11 = obs::Obs::new(sw + 50.0, sh - 250.0);
    let obs12 = obs::Obs::new(sw - 50.0, sh - 150.0);
    let mut player = player::Player::new();

    loop {
        clear_background(BLACK);
        player.update(get_frame_time());
        player.draw();
        obs1.draw();obs2.draw();
        obs3.draw();obs4.draw();
        obs5.draw();obs6.draw();
        obs7.draw();obs8.draw();
        obs9.draw();obs10.draw();
        obs11.draw();obs12.draw();
        
        // collision
        if player.rect.intersect(obs1.rect).is_some()
        || player.rect.intersect(obs2.rect).is_some() 
        || player.rect.intersect(obs3.rect).is_some() 
        || player.rect.intersect(obs4.rect).is_some() 
        || player.rect.intersect(obs5.rect).is_some() 
        || player.rect.intersect(obs6.rect).is_some() 
        || player.rect.intersect(obs7.rect).is_some() 
        || player.rect.intersect(obs8.rect).is_some(){ 
            player.xvel = player.xvel * -2.5;
            player.yvel = player.yvel * -2.5;
        } else {
            player.xvel = 0.0;
            player.yvel = 0.0;
        }

        if is_key_down(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
