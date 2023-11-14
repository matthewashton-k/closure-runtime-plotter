use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb, Rgba, RgbaImage, RgbImage};
use image::error::UnsupportedErrorKind::Color;
use image::imageops::flip_vertical;
use imageproc::definitions::HasBlack;
use rusttype::{Font, Scale};
use imageproc::drawing::{draw_filled_circle, draw_filled_circle_mut, draw_line_segment_mut, draw_text_mut};

const FONT: &[u8] = include_bytes!("../Roboto-Thin.ttf");

/// this will be in charge of creating an image on the file system and plotting dots where there are data points
pub struct Plotter<'a> {
    points: &'a Vec<(u128, u128)>,
    x_scale: f64,
    y_scale: f64,
}


impl<'a> Plotter<'a> {
    /// creates new plotter from a vector of points
    pub fn new(points: &'a Vec<(u128, u128)>) -> Self {
        // finds x and y max values
        let xmax = points.iter().map(|p|p.0).max().unwrap();// the biggest x value
        let ymax = points.iter().map(|p|p.1).max().unwrap(); // the biggest y value in the data

        Self {
            points,
            x_scale: 384./xmax as f64,
            y_scale: {
                384./ymax as f64
            }
        }
    }

    /// generates an image from the data contained in the plotter
    pub fn generate_image(&self, filepath: &str) -> Result<()> {
        // graph starts at 384
        let mut img = image::RgbImage::new(410,410);
        let ymin = self.min_y()?;

        let mut prev: Option<(f32, f32)> = None;
        for (x,y) in self.points.iter().map(
            |(x, y)| ((*x as f64*self.x_scale) as f32, (((*y-ymin)) as f64*self.y_scale) as f32 )
        ) {
            if let Some((px, py)) = prev {
                // println!("drawing segment from {prev:?} to {x},{y}");
                draw_line_segment_mut(&mut img, (px, py), (x,y), Rgb([30,144,255]));
            }
            prev = Some((x,y));
        }
        let mut flipped = flip_vertical(&mut img);
        self.draw_axis(&mut flipped);

        flipped.save(filepath)?;
        Ok(())
    }

    /// private function for drawing the axis
    fn draw_axis(&self, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<()> {
        // y axis
        draw_line_segment_mut(img, (1 as f32, 1 as f32), (0., (img.height() - 1) as f32), Rgb([255u8,   255u8,   255u8]));

        //x axis
        draw_line_segment_mut(img, ((img.width() - 1) as f32, (img.height() - 1) as f32), (0., (img.height() - 1) as f32), Rgb([255u8,   255u8,   255u8]));

        for p in self.points.iter().enumerate().filter(|(i, _)| (i + 1) % (self.points.len()/10) == 0).map(|(_, v)| v) {
            draw_line_segment_mut(
                img,
                ((p.0 as f64*self.x_scale) as f32, (img.height() - 1) as f32),
                ((p.0 as f64*self.x_scale) as f32, 0.),
                Rgb([64u8,   64u8,   64u8])
            );
            draw_text_mut(
              img,
              Rgb([255u8,   255u8,   255u8]),
              (p.0 as f64*self.x_scale) as i32-10,
              (img.height() - 16) as i32,
              Scale::uniform(10.),
              &Font::try_from_bytes(FONT).unwrap(),
                &p.0.to_string()
            );
        }
        let ymax = self.max_y()?;
        let ymin = self.min_y()?;
        let yline = 410.-((ymax-ymin) as f64*self.y_scale) as f32;

        draw_line_segment_mut(
            img,
            (0., yline),
            (410., yline),
            Rgb([64u8,   64u8,   64u8])
        );

        draw_text_mut(
            img,
            Rgb([255u8,   255u8,   255u8]),
            0,
            (yline) as i32,
            Scale::uniform(15.),
            &Font::try_from_bytes(FONT).unwrap(),
            &ymax.to_string()
        );
        Ok(())
    }

    fn min_y(&self) -> Result<u128> {
        self.points.iter().map(|p|p.1).min().ok_or(anyhow!("failed to find min/max value"))
    }

    fn min_x(&self) -> Result<u128> {
        self.points.iter().map(|p|p.0).min().ok_or(anyhow!("failed to find min/max value"))
    }

    fn max_x(&self) -> Result<u128> {
        self.points.iter().map(|p|p.0).max().ok_or(anyhow!("failed to find min/max value"))
    }

    fn max_y(&self) -> Result<u128>{
        self.points.iter().map(|p|p.1).max().ok_or(anyhow!("failed to find min/max value"))
    }
}