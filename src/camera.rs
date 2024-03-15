use crate::prelude::*;
use std::f64::consts::PI;
use theframework::prelude::*;

/// Camera
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Camera {
    pub origin: Vec3d,
    pub center: Vec3d,
    pub fov: f64,

    // For orbit
    pub distance: f64,

    pub forward: Vec3d,
    pub up: Vec3d,
    pub right: Vec3d,

    pub orbit_x: f64,
    pub orbit_y: f64,
}

impl Camera {
    pub fn new(origin: Vec3d, center: Vec3d, fov: f64) -> Self {
        Self {
            origin,
            center,
            fov,

            distance: 2.0,

            forward: Vec3d::new(0.0, 0.0, -1.0),
            up: Vec3d::new(0.0, 1.0, 0.0),
            right: Vec3d::new(1.0, 0.0, 0.0),

            orbit_x: 0.0,
            orbit_y: -90.0,
        }
    }

    /// Set the camera's origin and center based on the top-down angle (in degrees)
    pub fn set_top_down_angle(&mut self, angle_deg: f64, distance: f64, look_at: Vec3d) {
        let angle_rad = angle_deg.to_radians();
        let height = distance * angle_rad.sin();
        let horizontal_distance = distance * angle_rad.cos();

        self.center = look_at;

        // Assuming the camera looks along the negative z-axis by default
        self.origin = Vec3d {
            x: look_at.x,
            y: look_at.y + height,
            z: look_at.z - horizontal_distance,
        };
    }

    /// Zoom the camera by a given factor
    pub fn zoom(&mut self, delta: f64) {
        let direction = normalize(self.center - self.origin);

        self.origin += direction * delta;
        self.center += direction * delta;
    }

    // Move the camera by a given displacement
    pub fn move_by(&mut self, x_offset: f64, y_offset: f64) {
        // self.origin += Vec3d::new(x_offset, y_offset, 0.0);
        // self.center += Vec3d::new(x_offset, y_offset, 0.0);

        let direction = normalize(self.center - self.origin);
        let up_vector = vec3d(0.0, 1.0, 0.0);
        let right_vector = cross(direction, up_vector);

        let displacement = right_vector * x_offset + up_vector * y_offset;

        self.origin += displacement;
        self.center += displacement;

        /*
        let direction = normalize(self.center - self.origin);
        let up_vector = Vec3d(0.0, 1.0, 0.0);
        let right_vector = cross(direction, up_vector);

        self.origin += direction * y_offset + right_vector * x_offset;
        self.center += direction * y_offset + right_vector * x_offset;*/
    }

    /// Pan the camera horizontally and vertically
    pub fn pan(&mut self, horizontal: f64, vertical: f64) {
        let w = normalize(self.origin - self.center);
        let up_vector = vec3d(0.0, 1.0, 0.0);
        let u = cross(up_vector, w);
        let v = cross(w, u);

        self.center += u * horizontal + v * vertical;
    }

    /// Rotate the camera around its center
    pub fn rotate(&mut self, yaw: f64, pitch: f64) {
        fn magnitude(vec: Vec3d) -> f64 {
            (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt()
        }

        let radius = magnitude(self.origin - self.center);

        let mut theta = ((self.origin.z - self.center.z) / radius).acos();
        let mut phi = ((self.origin.x - self.center.x) / (radius * theta.sin())).acos();

        theta += pitch.to_radians();
        phi += yaw.to_radians();

        theta = theta.max(0.1).min(PI - 0.1);

        self.origin.x = self.center.x + radius * theta.sin() * phi.cos();
        self.origin.y = self.center.y + radius * theta.cos();
        self.origin.z = self.center.z + radius * theta.sin() * phi.sin();
    }

    /// Create a pinhole ray
    pub fn create_ray(&self, uv: Vec2d, screen: Vec2d, offset: Vec2d) -> Ray {
        let ratio = screen.x / screen.y;
        let pixel_size = vec2d(1.0 / screen.x, 1.0 / screen.y);

        let half_width = (self.fov.to_radians() * 0.5).tan();
        let half_height = half_width / ratio;

        let up_vector = vec3d(0.0, 1.0, 0.0);

        let w = normalize(self.origin - self.center);
        let u = cross(up_vector, w);
        let v = cross(w, u);

        let lower_left = self.origin - u * half_width - v * half_height - w;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        let mut dir = lower_left - self.origin;

        dir += horizontal * (pixel_size.x * offset.x + uv.x);
        dir += vertical * (pixel_size.y * offset.y + uv.y);

        Ray::new(self.origin, normalize(dir))
    }

    pub fn create_ortho_ray(&self, uv: Vec2d, screen: Vec2d, offset: Vec2d) -> Ray {
        let ratio = screen.x / screen.y;
        let pixel_size = Vec2d::new(1.0 / screen.x, 1.0 / screen.y);

        let cam_origin = self.origin;
        let cam_look_at = self.center;

        let half_width = ((self.fov + 100.0).to_radians() * 0.5).tan();
        let half_height = half_width / ratio;

        let up_vector = Vec3d::new(0.0, 1.0, 0.0);

        let w = normalize(cam_origin - cam_look_at);
        let u = cross(up_vector, w);
        let v = cross(w, u);

        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;

        let mut out_origin = cam_origin;
        out_origin += horizontal * (pixel_size.x * offset.x + uv.x - 0.5);
        out_origin += vertical * (pixel_size.y * offset.y + uv.y - 0.5);

        Ray::new(out_origin, normalize(-w))
    }

    /// Computes the orbi camera vectors. Based on https://www.shadertoy.com/view/ttfyzN
    pub fn compute_orbit(&mut self, mouse_delta: Vec2d) {
        #[inline(always)]
        pub fn mix(a: &f64, b: &f64, v: f64) -> f64 {
            (1.0 - v) * a + b * v
        }

        let min_camera_angle = 0.01;
        let max_camera_angle = std::f64::consts::PI - 0.01;

        self.orbit_x += mouse_delta.x;
        self.orbit_y += mouse_delta.y;

        let angle_x = -self.orbit_x;
        let angle_y = mix(&min_camera_angle, &max_camera_angle, self.orbit_y);

        let mut camera_pos = Vec3d::zero();

        camera_pos.x = sin(angle_x) * sin(angle_y) * self.distance;
        camera_pos.y = -cos(angle_y) * self.distance;
        camera_pos.z = cos(angle_x) * sin(angle_y) * self.distance;

        camera_pos += self.center;

        self.origin = camera_pos;
        self.forward = normalize(self.center - camera_pos);
        self.right = normalize(cross(vec3d(0.0, 1.0, 0.0), -self.forward));
        self.up = normalize(cross(-self.forward, self.right));
    }

    /// Create an orbit camera ray
    pub fn create_orbit_ray(&self, uv: Vec2d, screen_dim: Vec2d, offset: Vec2d) -> Ray {
        let camera_pos = self.origin;
        let camera_fwd = self.forward;
        let camera_up = self.up;
        let camera_right = self.right;

        let uv_jittered = (uv * screen_dim + (offset - 0.5)) / screen_dim;
        let mut screen = uv_jittered * 2.0 - 1.0;

        let aspect_ratio = screen_dim.x / screen_dim.y;
        screen.y /= aspect_ratio;

        let camera_distance = tan(self.fov * 0.5 * std::f64::consts::PI / 180.0);
        let mut ray_dir = vec3d(screen.x, screen.y, camera_distance);
        ray_dir = normalize(Mat3d::from((camera_right, camera_up, camera_fwd)) * ray_dir);

        Ray::new(camera_pos, ray_dir)
    }
}
