use crate::prelude::*;
use rayon::prelude::*;
use theframework::prelude::*;

pub struct Tracer {}

#[allow(clippy::new_without_default)]
impl Tracer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, buffer: &mut TheRGBABuffer, project: &Project) {
        let _start = self.get_time();

        //let stride = buffer.stride();
        //let height = buffer.dim().height;

        let width = buffer.dim().width as usize;
        let width_f = buffer.dim().width as f32;
        let height_f = buffer.dim().height as f32;

        let ro = vec3f(0.0, 1.0, 5.0);
        let rd = vec3f(0.0, 0.0, 0.0);
        let fov = 70.0;
        let camera_mode = CameraMode::Pinhole;

        let aa = 1;
        let aa_f = aa as f32;

        let pixels = buffer.pixels_mut();
        let iso_value = 0.0001;

        pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let xx = (i % width) as f32;
                    let yy = (i / width) as f32;

                    let uv = vec2f(xx, yy);

                    let mut total = Vec4f::zero();

                    for m in 0..aa {
                        for n in 0..aa {
                            let camera = Camera::new(ro, rd, fov);
                            let camera_offset =
                                vec2f(m as f32 / aa_f, n as f32 / aa_f) - vec2f(0.5, 0.5);

                            let mut ray = if camera_mode == CameraMode::Pinhole {
                                camera.create_ray(
                                    vec2f(xx / width_f, yy / height_f),
                                    vec2f(width_f, height_f),
                                    camera_offset,
                                )
                            } else {
                                camera.create_ortho_ray(
                                    vec2f(xx / width_f, yy / height_f),
                                    vec2f(width_f, height_f),
                                    camera_offset,
                                )
                            };

                            let mut color = vec4f(0.0, 0.0, 0.0, 1.0);

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
                                color = vec4f(1.0, 1.0, 1.0, 1.0);
                                let normal = self.normal(ray.at(t));
                                color.x = normal.x;
                                color.y = normal.y;
                                color.z = normal.z;
                            }

                            total += color;
                        }
                    }

                    let aa_aa = aa_f * aa_f;
                    total[0] /= aa_aa;
                    total[1] /= aa_aa;
                    total[2] /= aa_aa;
                    total[3] /= aa_aa;

                    pixel.copy_from_slice(&TheColor::from_vec4f(total).to_u8_array());
                }
            });

        let _stop = self.get_time();
        println!("render time {:?}", _stop - _start);
    }

    pub fn distance(&self, p: Vec3f) -> f32 {
        let d = length(p - vec3f(0.0, 0.0, 0.0)) - 1.0;
        //d += clamp(sin(p.x * 20.0 - 1.0) * 0.1, 0.0, 1.0);
        d
    }

    pub fn normal(&self, p: Vec3f) -> Vec3f {
        let scale = 0.5773 * 0.0005;
        let e = vec2f(1.0 * scale, -1.0 * scale);

        // IQs normal function

        let e1 = vec3f(e.x, e.y, e.y);
        let e2 = vec3f(e.y, e.y, e.x);
        let e3 = vec3f(e.y, e.x, e.y);
        let e4 = vec3f(e.x, e.x, e.x);

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
