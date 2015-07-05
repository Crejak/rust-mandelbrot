extern crate sfml;
extern crate complex;

use sfml::graphics::{Texture, Sprite, RenderWindow, RenderTarget, Image, Color, RectangleShape, Shape, Transformable};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style, MouseButton};
use complex::*;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
//use std::mem::transmute;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print_help("");
    } else {
        match &(*args[1]) {
            "generate" => generate_mandelbrot(args),
            "draw"     => draw_main(args),
            "help"     => {
                if args.len() >= 3 {
                    print_help(&(*args[2]));
                } else {
                    print_help("");
                }
            },
            _          => print_help(""),
        }
    }
    println!("");
}

#[derive(Debug, Clone)]
struct Plan {
    up: f64,
    left: f64,
    width: f64,
    height: f64,
}

impl Plan {
    //"2,1,3,4"
    fn from_string(s: &String) -> Option<Plan> {
        if &(*s) == "?" {
            return Some(Plan {
                up: -1.,
                left: -2.,
                width: 3.,
                height: 2.,
            });
        }
        let mut plan = Plan {
            up: 0.,
            left: 0.,
            width: 0.,
            height: 0.,
        };
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 4 {
            println!("Error : invalid Plan format, it must match 'x,y,w,h'.");
            return None;
        } else {
            plan.up = match coords[0].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
            plan.left = match coords[1].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
            plan.width = match coords[2].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
            plan.height = match coords[3].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
        }
        Some(plan)
    }
}

#[derive(Debug, Clone)]
struct ImageDim {
    width: usize,
    height: usize,
}

impl ImageDim {
    fn from_string(s: &String) -> Option<ImageDim> {
        if &(*s) == "?" {
            return Some(ImageDim {
                width: 900,
                height: 600,
            });
        }
        let mut img = ImageDim {
            width: 0,
            height: 0,
        };
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 2 {
            println!("Error : invalid Image Dim format, it must match 'w,h'.");
            return None;
        } else {
            img.width = match coords[0].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
            img.height = match coords[1].parse() {
                Ok(coord) => coord,
                Err(_)    => {
                    println!("Error : invalid Plan format, only numbers are accepted.");
                    return None;
                }
            };
        }
        Some(img)
    }
}

#[derive(Debug)]
struct PixelArrayBuffer {
    buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl PixelArrayBuffer {
    fn with_size(width: usize, height: usize) -> PixelArrayBuffer {
        PixelArrayBuffer {
            buffer : vec![0; width*height*4],
            width: width,
            height: height,
        }
    }

    fn at(&self, x: usize, y: usize) -> Color {
        let color_index = 4*(x*self.height + y);
        Color::new_rgb(
            self.buffer[color_index],
            self.buffer[color_index+1],
            self.buffer[color_index+2]
        )
    }

    fn get(&self) -> &[u8] {
        &self.buffer
    }

    fn set(&mut self, x: usize, y: usize, color: &Color) {
        let color_index = 4*(x*self.height + y);
        self.buffer[color_index] = color.red;
        self.buffer[color_index+1] = color.green;
        self.buffer[color_index+2] = color.blue;
    }
}

fn draw_mandelbrot(set_color: &Color, plan: &Plan, image_dim: &ImageDim, max_iter: u32) -> Image {
    let mut pixel_buffer = Arc::new(Mutex::new(PixelArrayBuffer::with_size(image_dim.width, image_dim.height)));

    let half_width = image_dim.width/2;
    let half_height = image_dim.height/2;

    let dim_1 = image_dim.clone();
    let plan_1 = plan.clone();
    let set_color_1 = *set_color;
    let buffer_1 = pixel_buffer.clone();
    let quarter_1 = thread::spawn(move || {
        let mut non_set_color = Color::new_rgb(0, 0, 0);
        for i in 0..half_width {
            for j in 0..half_height {
                let c = scale(i as i32, j as i32, &dim_1, &plan_1);
                let mut z = c64::new(0.0, 0.0);
                let mut iter = 0;
                while z.re()*z.re()+z.im()*z.im() < 4.0 && iter < max_iter {
                    iter += 1;
                    z = z*z + c;
                }
                let mut pixel_buffer = buffer_1.lock().unwrap();
                if iter == max_iter {
                    pixel_buffer.set(i, j, &set_color_1);
                } else {
                    let ratio = iter as f32/max_iter as f32;
                    if ratio > 0.666 {
                        let gradient = (255.*ratio) as u8;
                        non_set_color.red   = 255;
                        non_set_color.green = 255-gradient;
                        non_set_color.blue  = 0;
                    } else if ratio > 0.333 {
                        let gradient = (3.*255.*(ratio-0.333)) as u8;
                        non_set_color.red   = gradient;
                        non_set_color.green = gradient;
                        non_set_color.blue  = 255-gradient;
                    } else {
                        let gradient = (3.*255.*ratio) as u8;
                        non_set_color.red   = 0;
                        non_set_color.green = 0;
                        non_set_color.blue  = gradient;
                    }
                    /*non_set_color.red = gradient;
                    non_set_color.green = gradient;
                    non_set_color.blue = gradient;*/
                    pixel_buffer.set(i, j, &non_set_color);
                }
            }
        }
    });

    let dim_2 = image_dim.clone();
    let plan_2 = plan.clone();
    let set_color_2 = *set_color;
    let buffer_2 = pixel_buffer.clone();
    let quarter_2 = thread::spawn(move || {
        let mut non_set_color = Color::new_rgb(0, 0, 0);
        for i in half_width..dim_2.width {
            for j in 0..half_height {
                let c = scale(i as i32, j as i32, &dim_2, &plan_2);
                let mut z = c64::new(0.0, 0.0);
                let mut iter = 0;
                while z.re()*z.re()+z.im()*z.im() < 4.0 && iter < max_iter {
                    iter += 1;
                    z = z*z + c;
                }
                let mut pixel_buffer = buffer_2.lock().unwrap();
                if iter == max_iter {
                    pixel_buffer.set(i, j, &set_color_2);
                } else {
                    let ratio = iter as f32/max_iter as f32;
                    if ratio > 0.666 {
                        let gradient = (255.*ratio) as u8;
                        non_set_color.red   = 255;
                        non_set_color.green = 255-gradient;
                        non_set_color.blue  = 0;
                    } else if ratio > 0.333 {
                        let gradient = (3.*255.*(ratio-0.333)) as u8;
                        non_set_color.red   = gradient;
                        non_set_color.green = gradient;
                        non_set_color.blue  = 255-gradient;
                    } else {
                        let gradient = (3.*255.*ratio) as u8;
                        non_set_color.red   = 0;
                        non_set_color.green = 0;
                        non_set_color.blue  = gradient;
                    }
                    /*non_set_color.red = gradient;
                    non_set_color.green = gradient;
                    non_set_color.blue = gradient;*/
                    pixel_buffer.set(i, j, &non_set_color);
                }
            }
        }
    });

    let dim_3 = image_dim.clone();
    let plan_3 = plan.clone();
    let set_color_3 = *set_color;
    let buffer_3 = pixel_buffer.clone();
    let quarter_3 = thread::spawn(move || {
        let mut non_set_color = Color::new_rgb(0, 0, 0);
        for i in 0..half_width {
            for j in half_height..dim_3.height {
                let c = scale(i as i32, j as i32, &dim_3, &plan_3);
                let mut z = c64::new(0.0, 0.0);
                let mut iter = 0;
                while z.re()*z.re()+z.im()*z.im() < 4.0 && iter < max_iter {
                    iter += 1;
                    z = z*z + c;
                }
                let mut pixel_buffer = buffer_3.lock().unwrap();
                if iter == max_iter {
                    pixel_buffer.set(i, j, &set_color_3);
                } else {
                    let ratio = iter as f32/max_iter as f32;
                    if ratio > 0.666 {
                        let gradient = (255.*ratio) as u8;
                        non_set_color.red   = 255;
                        non_set_color.green = 255-gradient;
                        non_set_color.blue  = 0;
                    } else if ratio > 0.333 {
                        let gradient = (3.*255.*(ratio-0.333)) as u8;
                        non_set_color.red   = gradient;
                        non_set_color.green = gradient;
                        non_set_color.blue  = 255-gradient;
                    } else {
                        let gradient = (3.*255.*ratio) as u8;
                        non_set_color.red   = 0;
                        non_set_color.green = 0;
                        non_set_color.blue  = gradient;
                    }
                    /*non_set_color.red = gradient;
                    non_set_color.green = gradient;
                    non_set_color.blue = gradient;*/
                    pixel_buffer.set(i, j, &non_set_color);
                }
            }
        }
    });

    let dim_4 = image_dim.clone();
    let plan_4 = plan.clone();
    let set_color_4 = *set_color;
    let buffer_4 = pixel_buffer.clone();
    let quarter_4 = thread::spawn(move || {
        let mut non_set_color = Color::new_rgb(0, 0, 0);
        for i in half_width..dim_4.width {
            for j in half_height..dim_4.height {
                let c = scale(i as i32, j as i32, &dim_4, &plan_4);
                let mut z = c64::new(0.0, 0.0);
                let mut iter = 0;
                while z.re()*z.re()+z.im()*z.im() < 4.0 && iter < max_iter {
                    iter += 1;
                    z = z*z + c;
                }
                let mut pixel_buffer = buffer_4.lock().unwrap();
                if iter == max_iter {
                    pixel_buffer.set(i, j, &set_color_4);
                } else {
                    let ratio = iter as f32/max_iter as f32;
                    if ratio > 0.666 {
                        let gradient = (255.*ratio) as u8;
                        non_set_color.red   = 255;
                        non_set_color.green = 255-gradient;
                        non_set_color.blue  = 0;
                    } else if ratio > 0.333 {
                        let gradient = (3.*255.*(ratio-0.333)) as u8;
                        non_set_color.red   = gradient;
                        non_set_color.green = gradient;
                        non_set_color.blue  = 255-gradient;
                    } else {
                        let gradient = (3.*255.*ratio) as u8;
                        non_set_color.red   = 0;
                        non_set_color.green = 0;
                        non_set_color.blue  = gradient;
                    }
                    /*non_set_color.red = gradient;
                    non_set_color.green = gradient;
                    non_set_color.blue = gradient;*/
                    pixel_buffer.set(i, j, &non_set_color);
                }
            }
        }
    });

    quarter_1.join().unwrap();
    quarter_2.join().unwrap();
    quarter_3.join().unwrap();
    quarter_4.join().unwrap();
    let pixel_buffer = pixel_buffer.lock().unwrap();
    println!("Fini : pixel 0; 0 : {:?}; pixel 300; 108 {:?}", pixel_buffer.at(0, 0), pixel_buffer.at(300, 108));
    //let img = Image::create_from_pixels(pixel_buffer.width as u32, pixel_buffer.height as u32, pixel_buffer.get()).unwrap();
    let img = Image::create_from_pixels(2, 2, &[0, 0, 255, 255, 0, 255, 255, 255, 255, 255, 0, 255, 255, 0, 0, 255]).unwrap();
    img
}

fn scale(x: i32, y: i32, image_dim: &ImageDim, plan: &Plan) -> c64 {
    c64::new(x as f64 * plan.width / image_dim.width as f64 + plan.left, y as f64 * plan.height / image_dim.height as f64 + plan.up)
}

fn print_help(category: &str) {
    println!("");
    match category {
        "generate" => {
            println!("RUST-MANDELBROT : GENERATE");
            println!("--------------------------\n");
            println!("Synopsis : generate [plan] [image] [max_iter] [file]\n");
            println!("  plan     : the frame of the mandelbrot set you want to draw. It must be of the form 'up,left,width,height'. If you want the default settings (that are '-1,-2,3,2'), just type '?'.");
            println!("  image    : the image size, in pixels. It must match the following pattern : 'width,height'. If you want the default size (that is '900, 600'), type '?'.");
            println!("  max_iter : the max iterations used to determine the set's points. If you don't know which value you should use, prefer a number around 100.");
            println!("  file     : the output file to write the image. The format will be guessed from the extension. Supported fromats are : bmp, png, tga and jpg.");
            println!("--------------------------");
        },
        "draw" => {
            println!("RUST-MANDELBROT : DRAW");
            println!("----------------------\n");
            println!("Synopsis : draw [plan] [window] [max_iter]\n");
            println!("  plan     : the frame of the mandelbrot set you want to use as default view. When you will right-click, it will bring you to this view. It must be of the form 'up,left,width,height'. If you want the default settings (that are '-1,-2,3,2'), just type '?'.");
            println!("  window   : the size of the window, in pixels. It must match the following pattern : 'width,height'. If you want the default size (that is '900, 600'), type '?'.");
            println!("  max_iter : the max iterations used to determine the set's points. If you don't know which value you should use, prefer a number around 100.");
            println!("--------------------------");
        }
        _ => {
            println!("RUST-MANDELBROT : USE");
            println!("---------------------\n\nYou must specify a command while calling the programm :");
            println!("  help      Print this help.");
            println!("  generate  Generate a Mandelbrot set and save it to an image.");
            println!("  draw      Launch the interactive drawer.");
            println!("---------------------\nType `help [command]` to get more specific help about a command.");
            println!("\nYou don't know about Mandelbrot's set ? Just run with arguments 'draw ? ? 100' :)");
        }
    }
}

fn generate_mandelbrot(args: Vec<String>) {
    if args.len() != 6 {
        println!("Error : the `generate` command requires 4 arguments");
        println!("See `help generate` to get specific help");
    } else {
        let option_plan = Plan::from_string(&args[2]);
        if let Some(plan) = option_plan {
            let option_image = ImageDim::from_string(&args[3]);
            if let Some(image_dim) = option_image {
                //Dessin :)
                if let Ok(max_iter) = args[4].parse() {
                    let mut img = Image::new(image_dim.width as u32, image_dim.height as u32).unwrap();
                    let set_color = Color::black();
                    let mut non_set_color = Color::white();
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
                                img.set_pixel(i as u32, j as u32, &set_color);
                            } else {
                                let ratio = iter as f32/max_iter as f32;
                                if ratio > 0.666 {
                                    let gradient = (255.*ratio) as u8;
                                    non_set_color.red   = 255;
                                    non_set_color.green = 255-gradient;
                                    non_set_color.blue  = 0;
                                } else if ratio > 0.333 {
                                    let gradient = (3.*255.*(ratio-0.333)) as u8;
                                    non_set_color.red   = gradient;
                                    non_set_color.green = gradient;
                                    non_set_color.blue  = 255-gradient;
                                } else {
                                    let gradient = (3.*255.*ratio) as u8;
                                    non_set_color.red   = 0;
                                    non_set_color.green = 0;
                                    non_set_color.blue  = gradient;
                                }
                                /*let gradient = (255*iter/max_iter) as u8;
                                non_set_color.red = gradient;
                                non_set_color.green = gradient;
                                non_set_color.blue = gradient;*/
                                img.set_pixel(i as u32, j as u32, &non_set_color);
                            }
                        }
                    }
                    img.save_to_file(&(*args[5]));
                }
            }
        }
    }
}

fn draw_main(args: Vec<String>) {
    if args.len() != 5 {
        println!("");
        println!("Error : the `draw` command requires 3 arguments");
        println!("See `help draw` to get specific help");
    } else {
        let option_plan = Plan::from_string(&args[2]);
        if let Some(mut plan) = option_plan {
            let option_image = ImageDim::from_string(&args[3]);
            if let Some(mut image_dim) = option_image {
                //Dessin :)
                if let Ok(mut max_iter) = args[4].parse::<u32>() {
                    let mut window = RenderWindow::new(VideoMode::new_init(image_dim.width as u32, image_dim.height as u32, 32),
                                                   "Mandelbrot",
                                                   window_style::CLOSE,
                                                   &ContextSettings::default()).expect("Couldn't create RenderWindow");
                    window.set_framerate_limit(30);

                    let mut redraw = false; //should we redraw the set ?
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
                                event::Closed => window.close(),
                                event::MouseButtonPressed {
                                    button, x, y
                                } => {
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
                                } => rect.set_position(&Vector2f::new(x as f32, y as f32)),
                                _ => {},
                            }
                        }

                        if redraw {
                            img = draw_mandelbrot(&Color::new_rgb(0, 0, 0),
                                            &plan,
                                            &image_dim,
                                            max_iter*(zoom_lvl+1));
                            tex = Texture::new_from_image(&img).unwrap();
                            redraw = false;
                        }

                        window.clear(&Color::black());
                        window.draw(&Sprite::new_with_texture(&tex).unwrap());
                        window.draw(&rect);
                        window.display();
                    }
                }
            }
        }
    }
}
