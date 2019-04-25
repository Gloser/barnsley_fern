use raylib::prelude::*;
use rand::Rng;


fn main() {
        let rl = raylib::init().size(600, 600).title("Barnsley Fern").msaa_4x().build();
    
        let mut rng = rand::thread_rng();
    
        let width: i32 = rl.get_screen_width();
        let height: i32 = rl.get_screen_height() ;
    
        let mut x: f32 = 0.;
        let mut y: f32 = 0.;
    
        let mut next_x: f32 = 0.0;
        let mut next_y: f32 = 0.0;
    
        //rl.set_target_fps(100);
    
        rl.clear_background(Color::new(0, 0, 0, 255));
    
        while !rl.window_should_close() {
        
            if rng.gen_range(0, 100) < 1 {
            
                next_x = 0.0;
                next_y = 0.16 * y as f32;
        } else if rng.gen_range(0, 100) < 86 {
            
            next_x = 0.85 * x as f32 + 0.04 * y as f32;
                next_y = -0.04 * x as f32 + 0.85 * y as f32 + 1.6;
        } else if rng.gen_range(0, 100) < 93 {
            
            next_x = 0.2 * x as f32 - 0.26 * y as f32;
                next_y = 0.23 * x as f32 + 0.22 * y as f32 + 1.6;
        } else {
            
            next_x = -0.15 * x as f32 + 0.28 * y as f32;
                next_y = 0.26 * x as f32 + 0.24 * y as f32 + 0.44;
        }
        
        rl.begin_drawing();
        
            rl.draw_pixel((50.0 * next_x as f32 + width as f32 / 2.0).round() as i32, ((50.0 * next_y as f32) + 50.) as i32, Color::new(112, 139, 93, 255) );
        
            rl.end_drawing();
        
            x = next_x;
            y = next_y;
        
    }
}
