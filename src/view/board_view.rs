use sdl2::{
    rect::{
        Rect,
        Point,
    },
    pixels::Color,
    render::Canvas,
    video::Window
};
use crate::model::game::Cell;
pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color
}

impl Renderer {
    pub fn new(screen_area: Rect, clear_color: Color) -> Self {
        Renderer { screen_area, clear_color}
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, board: &[[Cell; 50]; 50]) {
        // background
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        // cell outline color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let cell_width = self.screen_area.w / 50;
        let cell_height = self.screen_area.h / 50;
        
        // draw 
        draw_outline(canvas, &self, cell_width, cell_height, 50);
        draw_cell(canvas, cell_width, cell_height, 50, board)
    }
}
fn draw_outline(canvas: &mut Canvas<Window>,renderer: &Renderer ,width: i32, height: i32, cell_number: i32) {
    for i in 0..cell_number{
        canvas.draw_line(
            Point::new(width / 2, height / 2 + i*height),
            Point::new(renderer.screen_area.w - width / 2, height / 2 + i*height)
        ).ok().unwrap_or_default();
        canvas.draw_line(
            Point::new(width/2 + i*width, height/2),
            Point::new(width/2 + i*width, renderer.screen_area.h - height / 2)
        ).ok().unwrap_or_default();
    }
}
fn draw_cell(canvas: &mut Canvas<Window>, width: i32, height: i32, cell_number: i32, board:&[[Cell; 50]; 50]) {
    for i in 0..cell_number-1 {
        let row:usize = i.try_into().unwrap();
        for j in 0..cell_number-1 {
            let col: usize = j.try_into().unwrap();
            let color = match board[row][col] {
                Cell::Dead => Color::WHITE,
                Cell::Alive => Color::BLACK,
            };

            let rect = Rect::new(
                width / 2 + width * j + 1, height / 2 + height * i + 1,
                (width-1).try_into().unwrap(), (height-1).try_into().unwrap()
            );
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).ok().unwrap_or_default();
        }
    }
}
