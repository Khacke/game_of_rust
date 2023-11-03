use sdl2::{pixels::Color, event::Event, rect::Rect, keyboard::Keycode};

mod view;
use view::board_view::Renderer;

mod model;
use model::game::{GameState, TimeFlow};

fn main() -> Result<(), String>{
    // screen params
    let screen_width = 800;
    let screen_height = 600;

    // init sdl2 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Game of Life", screen_width, screen_height)
    .position_centered()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
    .build()
    .unwrap();

    let board_view = Renderer::new(
        Rect::new(0,0,screen_width, screen_height),
        Color::RGB(188, 188, 255)
    );
    
    let mut game_state = GameState::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                Event::MouseButtonDown {x, y , ..} => {
                    let col: usize = ((x+board_view.screen_area.w / 50/2)/(board_view.screen_area.w / 50)-1).try_into().unwrap();
                    let row: usize = ((y+board_view.screen_area.h / 50/2)/(board_view.screen_area.h / 50)-1).try_into().unwrap();
                    game_state.handle_click(row, col)
                },
                Event::KeyDown {keycode,..} => {
                    if keycode.unwrap() ==  Keycode::Space {
                        game_state.change_time_flow();
                    }
                }
                _ => {}
            }
        }
        let running = match game_state.time_flow {
            TimeFlow::Stopped => false,
            TimeFlow::Running => true
        };
        if running {
            game_state.calculate_next_state()
        }
        board_view.render(&mut canvas, &game_state.board);
        canvas.present();
    }

    Ok(())
}
