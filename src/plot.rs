use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;
// use tracing::{debug, trace};

use crate::product_symbology::RadialData;

use super::Radar;

pub fn plot(input: Radar) -> Result<(), Box<dyn std::error::Error>> {
    let image_width = 800;
    let image_height = 800;
    let logical_width = image_width as f32;
    let logical_height = image_height as f32;

    let r_max = 350.0;
    let n_bins = input.symbology_layers.first().unwrap().packet_header.num_bins as f32;
    // let n_radials = input.symbology.packet_header.num_radials as f32;
    // let range_scale = input.symbology.packet_header.range_scale;

    let xc = logical_width / 2.0;
    let yc = logical_height / 2.0;

    // define image size
    let root = BitMapBackend::new("image.png", (image_width, image_height)).into_drawing_area();

    // map image size (pixels) to logical grid of f32
    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..logical_width,
        0f32..logical_height,
        (0..image_width as i32, 0..image_height as i32),
    ));

    // let wedge = |x: f32, y: f32| {
    //     return EmptyElement::at((x, y))
    //         // + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
    //         + Text::new(
    //             format!("({:.2},{:.2})", x, y),
    //             (10, 0),
    //             ("sans-serif", 15.0).into_font(),
    //         );
    // };

    root.fill(&WHITE)?;

    // Draw an circle on the drawing area
    // let points = vec![(0.0, 0.0), (100.0, 100.0), (100.0, 0.0)];
    // root.draw(&Polygon::new(
    //     points,
    //     Into::<ShapeStyle>::into(&GREEN).filled(),
    // ))?;
    
    // root.draw(&Circle::new(
    //     (100.0, 100.0),
    //     50.0,
    //     Into::<ShapeStyle>::into(&GREEN).filled(),
    // ))?;

    // root.draw(&wedge(50.5, 70.6))?;

    input.symbology_layers.first().unwrap().radials.iter().for_each(|radial| {
        let angle = (270.0 + radial.header.angle_start as f32 / 10.0) * std::f32::consts::PI/180.0;
        let delta_angle = (radial.header.angle_delta as f32 / 10.0) * std::f32::consts::PI/180.0;
        // trace!("Angle {:?}", angle);
        
        match &radial.data {
            RadialData::Digital(data) => {
                data.iter().enumerate().for_each(|(index, value)| {

                    let radius_inner = (index as f32)/n_bins * r_max;
                    let radius_outer = (index as f32+1.0)/n_bins * r_max;
                    let p1 = (
                        radius_inner*angle.cos()+xc, 
                        radius_inner*angle.sin()+yc
                    );
                    let p2 = (
                        radius_inner*(angle+delta_angle).cos()+xc, 
                        radius_inner*(angle+delta_angle).sin()+yc
                    );
                    let p3 = (
                        radius_outer*(angle+delta_angle).cos()+xc, 
                        radius_outer*(angle+delta_angle).sin()+yc
                    );
                    let p4 = (
                        radius_outer*angle.cos()+xc, 
                        radius_outer*angle.sin()+yc
                    );
                  
                    let points = vec![p1, p2, p3, p4];
                    // debug!("{:?}", points);
                    let _ = root.draw(&Polygon::new(
                        points,
                        Into::<ShapeStyle>::into(HSLColor(*value as f64 / 256.0, 0.5, 0.5)).filled(),
                    ));
                    // let _ = root.draw(&Circle::new(
                    //     p1,
                    //     1.0,
                    //     Into::<ShapeStyle>::into(HSLColor(*value as f64 / 256.0, 0.5, 0.5)).filled(),
                    // ));

                    
                })
            },
            _ => {

            }
        }
    });

    root.present()?;
    Ok(())
}

