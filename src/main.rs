extern crate sfml;
extern crate complex;

use sfml::graphics::{Texture, Sprite, RenderWindow, RenderTarget, Image, Color, RectangleShape, Shape, Transformable};
use sfml::system::{Clock, Time, Vector2f};
use sfml::window::{ContextSettings, VideoMode, event, window_style, MouseButton};
use std::ops::Add;
use complex::*;
use std::mem::transmute;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(900, 600, 32),
                                   "Mandelbrot",
                                   window_style::CLOSE,
                                   &ContextSettings::default()).expect("Couldn't create RenderWindow");
    window.set_framerate_limit(30);

    let mut redraw = false; //should we redraw the set ?
    let mut plan = Plan {up: -1.0, left: -2.0, width: 3.0, height: 2.0};
    let image_dim = ImageDim {width: 900, height: 600};
    let max_iter = 100;
    let mut img = draw_mandelbrot(&Color::new_rgb(0, 0, 0),
                    &plan,
                    &image_dim,
                    max_iter);
    let mut tex = Texture::new_from_image(&img).unwrap();

    let mut zoom_lvl = 0;
    let mut rect = RectangleShape::new_init(&Vector2f::new(90., 60.)).unwrap();
    rect.set_origin(&Vector2f::new(45., 30.));
    rect.set_fill_color(&Color::new_rgba(0, 0, 0, 0));
    rect.set_outline_color(&Color::new_rgb(255, 255, 255));
    rect.set_outline_thickness(1.);

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed               => window.close(),
                event::MouseButtonPressed {
                    button, x, y
                }                           => {
                                                   if button == MouseButton::Right {
                                                       zoom_lvl = 0;
                                                       redraw = true;
                                                       plan = Plan {up: -1.0, left: -2.0, width: 3.0, height: 2.0}; //valeurs par défaut
                                                   } else if button == MouseButton::Left {
                                                       redraw = true;
                                                       //zoom sur un rectangle de 90*60 centré sur la souris
                                                       let left = x - 45;
                                                       let up = y - 30;
                                                       let up_left = scale(left, up, &image_dim, &plan);
                                                       let width_height = scale(90, 60, &image_dim, &Plan {up: 0., left: 0., width: plan.width, height: plan.height});
                                                       plan = Plan {up: up_left.im(), left: up_left.re(), width: width_height.re(), height: width_height.im()};
                                                       zoom_lvl += 1;
                                                       println!("ZOOM: {} sur ({:.5}; {:.5})", zoom_lvl, plan.left+plan.width/2., plan.up+plan.height/2.);
                                                   }
                                               },
                event::MouseMoved {
                    x, y
                }                           => rect.set_position(&Vector2f::new(x as f32, y as f32)),
                _                           => {},
            }
        }

        if redraw {
            img = draw_mandelbrot(&Color::new_rgb(0, 0, 0),
                            &plan,
                            &image_dim,
                            max_iter*zoom_lvl+60);
            tex = Texture::new_from_image(&img).unwrap();
            //sprite = Sprite::new_with_texture(&tex).unwrap();
            redraw = false;
        }

        window.clear(&Color::black());
        window.draw(&Sprite::new_with_texture(&tex).unwrap());
        window.draw(&rect);
        window.display();
    }
}

#[derive(Debug)]
struct Plan {
    up: f64,
    left: f64,
    width: f64,
    height: f64,
}

struct ImageDim {
    width: u32,
    height: u32,
}

fn draw_mandelbrot(set_color: &Color, plan: &Plan, image_dim: &ImageDim, max_iter: u32) -> Image {
    let mut img = Image::new(image_dim.width, image_dim.height).unwrap();
    let mut non_set_color = Color::new_rgb(0, 0, 0);
    //println!("0, 0 : {:?}", scale(0, 0, &image_dim, &plan));
    //println!("{}, {} : {:?}", image_dim.width-1, image_dim.height-1, scale(image_dim.width-1, image_dim.height-1, &image_dim, &plan));
    let mut percent = 0;
    for i in 0..image_dim.width {
        for j in 0..image_dim.height {
            let c = scale(i as i32, j as i32, &image_dim, &plan);
            let mut z = c64::new(0.0, 0.0);
            let mut iter = 0;
            while z.re()*z.re()+z.im()*z.im() < 4.0 && iter < max_iter {
                iter += 1;
                z = z*z + c;
            }
            if iter == max_iter {
                img.set_pixel(i, j, &set_color);
            } else {
                /*non_set_color = Color::new_rgb(0, 0, 0);
                let gradient = (2* 255 * iter / max_iter) as u32;
                /*let gradient = unsafe { transmute::<u32, [u8; 4]>(gradient) };
                non_set_color.red = gradient[0];
                non_set_color.green = gradient[0];
                non_set_color.blue = gradient[0];*/
                if gradient > 255 {
                    non_set_color.red = (gradient-255) as u8;
                    non_set_color.green = non_set_color.red / 2;
                } else {
                    non_set_color.blue = gradient as u8;
                }*/
                let gradient = (255*iter/max_iter) as u8;
                non_set_color.red = gradient;
                non_set_color.green = gradient;
                non_set_color.blue = gradient;
                img.set_pixel(i, j, &non_set_color);
            }
        }
        let cur = 100*i/image_dim.width;
        if cur-percent >= 1 {
            percent = cur;
        }
    }
    img
}

fn scale(x: i32, y: i32, image_dim: &ImageDim, plan: &Plan) -> c64 {
    c64::new(x as f64 * plan.width / image_dim.width as f64 + plan.left, y as f64 * plan.height / image_dim.height as f64 + plan.up)
}
