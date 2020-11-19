use image::{DynamicImage, GenericImageView, Rgba};

fn main() {
    let img = image::open("src/images/testImage.png").unwrap();
    let threshold: f32 = 20.0;

    let mut circles: Vec<(u32, u32, f32)> = Vec::new();

    for index in 0..10 {
        let current_circle = img
            .pixels()
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
                                pixel_a.0,
                                pixel_a.1,
                            )
                        })
                        .filter(|pixel| pixel.0 > threshold)
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
            .fold((0, 0, 0.0), |highest, pixel| {
                if highest.0 <= pixel.0 {
                    return pixel;
                } else {
                    return highest;
                }
            });

        circles.push(current_circle);
        println!("{}{:?}", index, current_circle);
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

fn do_circles_intersect(pixel_a: (u32, u32, f32), pixel_b: (u32, u32, f32)) -> bool {
    (((pixel_a.0 - pixel_b.0).pow(2) + (pixel_a.1 - pixel_b.1).pow(2)) as f32).sqrt()
        <= (pixel_a.2) + (pixel_b.2)
}

fn is_point_inside_circle(a: u32, b: u32, x: u32, y: u32, r: f32) -> bool {
    let dist_points = (a - x) * (a - x) + (b - y) * (b - y);
    if dist_points < (r * r) as u32 {
        return true;
    }
    return false;
}
