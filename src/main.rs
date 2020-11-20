use image::{DynamicImage, GenericImageView, Rgba};

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

use serde::*;
use serde_derive::*;
use serde_json::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let img = image::open("src/images/trees.jpg").unwrap();
    let threshold: f32 = 100.0;

    let mut circles: Vec<(u32, u32, f32, (usize, usize, usize, usize))> = Vec::new();

    for index in 0..10000 {
        let current_circle = img
            .pixels()
            .par_bridge()
            .filter(|pixel| {
                circles
                    .iter()
                    .map(|circle| {
                        is_point_inside_circle(pixel.0, pixel.1, circle.0, circle.1, circle.2)
                    })
                    .any(|circle| circle == true)
                    == false
            })
            .map(|pixel_a| {
                (
                    pixel_a.0,
                    pixel_a.1,
                    img.pixels()
                        .map(move |pixel_b| {
                            (
                                compute_color_distance(pixel_a, pixel_b),
                                compute_location_distance(pixel_a, pixel_b),
                            )
                        })
                        .filter(|pixel| pixel.0 > threshold && pixel.1 > 1.0)
                        .map(|pixel| pixel.1)
                        .fold(1000.0, |lowest: f32, pixel: f32| {
                            if lowest >= pixel {
                                return pixel;
                            } else {
                                return lowest;
                            }
                        }),
                )
            })
            .filter(|pixel| {
                circles
                    .iter()
                    .map(|circle| do_circles_intersect(*pixel, *circle))
                    .any(|circle| circle == true)
                    == false
            })
            .reduce(
                || (0, 0, 0.0),
                |highest, pixel| {
                    if highest.0 <= pixel.0 {
                        return pixel;
                    } else {
                        return highest;
                    }
                },
            );

        let color_added = img
            .pixels()
            .filter(|pixel| {
                is_point_inside_circle(
                    pixel.0,
                    pixel.1,
                    current_circle.0,
                    current_circle.1,
                    current_circle.2,
                ) == true
            })
            .fold((0, 0, image::Rgba([0, 0, 0, 0])), |color, pixel| {
                (
                    pixel.0,
                    pixel.1,
                    image::Rgba([
                        color.2 .0[0] as usize + pixel.2 .0[0] as usize,
                        color.2 .0[1] as usize + pixel.2 .0[1] as usize,
                        color.2 .0[2] as usize + pixel.2 .0[2] as usize,
                        color.2 .0[3] as usize + pixel.2 .0[3] as usize,
                    ]),
                )
            });

        let mut count = img
            .pixels()
            .filter(|pixel| {
                is_point_inside_circle(
                    pixel.0,
                    pixel.1,
                    current_circle.0,
                    current_circle.1,
                    current_circle.2,
                )
            })
            .count();

        if count == 0 {
            count = 1
        }

        let color = (
            color_added.2 .0[0] as usize / count,
            color_added.2 .0[1] as usize / count,
            color_added.2 .0[2] as usize / count,
            color_added.2 .0[3] as usize / count,
        );

        circles.push((
            current_circle.0,
            current_circle.1,
            round_to_golden(current_circle.2),
            color,
        ));
        println!("{}{:?}{:?}", index, current_circle, color);

        File::create("./result.json")
            .expect("Unable to create file")
            .write_all(
                serde_json::to_string(&circles)
                    .expect("Unable to write file")
                    .as_bytes(),
            )
            .expect("Unable to write data");
    }
}

fn compute_location_distance(
    pixel_a: (u32, u32, image::Rgba<u8>),
    pixel_b: (u32, u32, image::Rgba<u8>),
) -> f32 {
    (((pixel_a.0 as i32 - pixel_b.0 as i32).pow(2) + (pixel_a.1 as i32 - pixel_b.1 as i32).pow(2))
        as f32)
        .sqrt()
}

fn compute_color_distance(
    pixel_a: (u32, u32, image::Rgba<u8>),
    pixel_b: (u32, u32, image::Rgba<u8>),
) -> f32 {
    (((pixel_a.2 .0[0] as i32 - pixel_b.2 .0[0] as i32).pow(2)
        + (pixel_a.2 .0[1] as i32 - pixel_b.2 .0[1] as i32).pow(2)
        + (pixel_a.2 .0[2] as i32 - pixel_b.2 .0[2] as i32).pow(2)) as f32)
        .sqrt()
}

fn do_circles_intersect(
    pixel_a: (u32, u32, f32),
    pixel_b: (u32, u32, f32, (usize, usize, usize, usize)),
) -> bool {
    (((pixel_a.0 - pixel_b.0).pow(2) + (pixel_a.1 - pixel_b.1).pow(2)) as f32).sqrt()
        <= (pixel_a.2) + (pixel_b.2)
}

fn is_point_inside_circle(a: u32, b: u32, x: u32, y: u32, r: f32) -> bool {
    let dist_points = (a - x).pow(2) + (b - y).pow(2);
    if dist_points < (r * r) as u32 {
        return true;
    }
    return false;
}

fn round_to_golden(radius: f32) -> f32 {
    if radius <= 2.0 {
        return 1.0;
    } else if radius <= 3.0 {
        return 2.0;
    } else if radius <= 5.0 {
        return 3.0;
    } else if radius <= 8.0 {
        return 5.0;
    } else if radius <= 13.0 {
        return 8.0;
    } else if radius <= 13.0 {
        return 13.0;
    } else if radius <= 21.0 {
        return 13.0;
    } else if radius <= 34.0 {
        return 21.0;
    } else if radius <= 55.0 {
        return 34.0;
    } else if radius <= 89.0 {
        return 55.0;
    }
    return 89.0;
}
