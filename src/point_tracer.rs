use crate::prelude::*;
use rayon::prelude::*;
use theframework::prelude::*;

pub struct PointTracer {}

#[allow(clippy::new_without_default)]
impl PointTracer {
    pub fn new() -> Self {
        Self {
            expr: None,
            //script: None,
        }
    }

    pub fn render(&mut self, buffer: &mut TheRGBABuffer, project: &Project) {
        let _start = self.get_time();

        //let stride = buffer.stride();
        //let height = buffer.dim().height;

        let width = buffer.dim().width as usize;
        let width_f = buffer.dim().width as f64;
        let height_f = buffer.dim().height as f64;

        let ro = vec3d(0.0, 1.0, 5.0);
        let rd = vec3d(0.0, 0.0, 0.0);
        let fov = 70.0;
        let camera_mode = CameraMode::Pinhole;

        let aa = 2;
        let aa_f = aa as f64;

        let pixels = buffer.pixels_mut();
        let iso_value = 0.0001_f64;

        pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let xx = (i % width) as f64;
                    let yy = (i / width) as f64;

                    let uv = vec2d(xx, yy);

                    let mut total = Vec4d::zero();

                    for m in 0..aa {
                        for n in 0..aa {
                            let camera = Camera::new(ro, rd, fov);
                            let camera_offset =
                                vec2d(m as f64 / aa_f, n as f64 / aa_f) - vec2d(0.5, 0.5);

                            let mut ray = if camera_mode == CameraMode::Pinhole {
                                camera.create_ray(
                                    vec2d(xx / width_f, yy / height_f),
                                    vec2d(width_f, height_f),
                                    camera_offset,
                                )
                            } else {
                                camera.create_ortho_ray(
                                    vec2d(xx / width_f, yy / height_f),
                                    vec2d(width_f, height_f),
                                    camera_offset,
                                )
                            };

                            let mut color = vec4d(0.0, 0.0, 0.0, 1.0);

                            let mut t = iso_value;
                            let t_max = 10.0;

                            let mut hit = false;

                            for _ in 0..100 {
                                let p = ray.at(t);

                                let d = self.distance(p);

                                t += d;

                                if d < iso_value {
                                    hit = true;
                                    break;
                                } else if t > t_max {
                                    break;
                                }
                            }

                            if hit {
                                let normal = self.normal(ray.at(t));
                                color.x = normal.x;
                                color.y = normal.y;
                                color.z = normal.z;
                                color.z = 1.0;
                            }

                            total += color;
                        }
                    }

                    let aa_aa = aa_f * aa_f;
                    total[0] /= aa_aa;
                    total[1] /= aa_aa;
                    total[2] /= aa_aa;
                    total[3] /= aa_aa;

                    //pixel.copy_from_slice(&TheColor::from_vec4f(total).to_u8_array());
                    let out = [
                        (total[0] * 255.0) as u8,
                        (total[1] * 255.0) as u8,
                        (total[2] * 255.0) as u8,
                        (total[3] * 255.0) as u8,
                    ];
                    pixel.copy_from_slice(&out);
                }
            });

        let _stop = self.get_time();
        println!("render time {:?}", _stop - _start);
    }

    pub fn distance(&self, p: Vec3d) -> f64 {
        let mut d = length(p - vec3d(0.0, 0.0, 0.0)) - 1.0;
        // d += clamp(sin(p.x * 20.0 - 1.0) * 0.1, 0.0, 1.0);
        if let Some(expr) = &self.expr {
            if let Ok(v) = expr.eval(&[p.x]) {
                d += v;
            }
        }

        d
    }

    pub fn normal(&self, p: Vec3d) -> Vec3d {
        let scale = 0.5773 * 0.0005;
        let e = vec2d(1.0 * scale, -1.0 * scale);

        // IQs normal function

        let e1 = vec3d(e.x, e.y, e.y);
        let e2 = vec3d(e.y, e.y, e.x);
        let e3 = vec3d(e.y, e.x, e.y);
        let e4 = vec3d(e.x, e.x, e.x);

        let n = e1 * self.distance(p + e1)
            + e2 * self.distance(p + e2)
            + e3 * self.distance(p + e3)
            + e4 * self.distance(p + e4);
        normalize(n)
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        let time;
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            time = t.as_millis();
        }
        #[cfg(target_arch = "wasm32")]
        {
            time = web_sys::window().unwrap().performance().unwrap().now() as u128;
        }
        time
    }
}
