extern crate sfml;
extern crate complex;

use sfml::graphics::{Texture, Sprite, RenderWindow, RenderTarget, Image, Color, RectangleShape, Shape, Transformable};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, window_style, MouseButton};
use complex::*;
use std::env;

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

#[derive(Debug)]
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

struct ImageDim {
    width: u32,
    height: u32,
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
                    let mut img = Image::new(image_dim.width, image_dim.height).unwrap();
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
                                img.set_pixel(i, j, &set_color);
                            } else {
                                let gradient = (255*iter/max_iter) as u8;
                                non_set_color.red = gradient;
                                non_set_color.green = gradient;
                                non_set_color.blue = gradient;
                                img.set_pixel(i, j, &non_set_color);
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
                    let mut window = RenderWindow::new(VideoMode::new_init(900, 600, 32),
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
