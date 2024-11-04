const FLT_MAX: f32 = 3.40282346638528859812e+38;
const MAX_PATH_LENGTH: u32 = 15;
const EPSILON: f32 = 1e-3;

const OBJECT_COUNT: u32 = 4;
alias Scene = array<Sphere, OBJECT_COUNT>;
var<private> scene: Scene = Scene(
    Sphere(vec3(1., 0., -1.), 0.5, vec3(0.5, 0.4, 0.)),
    Sphere(vec3(-1., 0., -1.), 0.5, vec3(0.2, 0.5, 0.2)),
    Sphere(vec3(0., -1.1, -1.), 0.5, vec3(0.7, 0.4, 0.6)),
    Sphere(vec3(0., 1.1, -1.), 0.5, vec3(0.2, 0.2, 1.)),
);


alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
    vec2f(-1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, -1.0),
);

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
    return vec4f(vertices[vid], 0.0, 1.0);
}

struct CameraUniforms {
    origin: vec3f,
    // Camera basis vectors
    u: vec3f,
    v: vec3f,
    w: vec3f,
}

struct Uniforms {
    camera: CameraUniforms,
    width: u32,
    height: u32,
    frame_count: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var radiance_samples_old: texture_2d<f32>;
@group(0) @binding(2) var radiance_samples_new: texture_storage_2d<rgba32float, write>;

// XORshitf RNG
struct Rng {
    state: u32,
}
// thread local Rng
var<private> rng: Rng;

fn init_rng(pixel: vec2u) {
    // Seed PRNG with scalar index of the pixel and current frame count
    let seed = (pixel.x + pixel.y * uniforms.width) ^ jenkins_hash(uniforms.frame_count);
    rng.state = jenkins_hash(seed);
}

fn jenkins_hash(i: u32) -> u32 {
    var x = i;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}

// 32 bit "xor" function from "Xorshift RNGs"
fn xor_shift_32() -> u32 {
    var x = rng.state;
    x ^= x << 13u;
    x ^= x >> 17u;
    x ^= x << 5u;
    rng.state = x;
    return x;
}

// Returns a random float in the range [0...1].
//
// This sets the floating point exponent to zero and sets the most significant
// 23 bits of a random 32-bit unsigned integer as the mantissa.
//
// That generates a number in the range [1, 1.9999999],
// which is then mapped to [0, 0.9999999] by subtraction.
//
// See Ray Tracing Gems II, Section 14.3.4.
fn rand_f32() -> f32 {
    return bitcast<f32>(0x3f800000u | (xor_shift_32() >> 9u)) - 1.0;
}

// A ray can be described as a simple line
//     Y = P + tD
// where P is the ray origin and D the ray direction.
struct Ray {
    origin: vec3f,
    direction: vec3f,
}

fn point_on_ray(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}

struct Scatter {
    attenuation: vec3f,
    ray: Ray,
}

fn scatter(input_ray: Ray, hit: Intersection) -> Scatter {
    let reflected = reflect(input_ray.direction, hit.normal);
    let output_ray = Ray(point_on_ray(input_ray, hit.t), reflected);
    let attenuation = hit.color;
    return Scatter(attenuation, output_ray);
}

// The surface of a sphere can be described as
//     (X - C)·(X - C) = r²
// where `X` is a point on the sphere surface, `C` the sphere center,
// and `r` the radius.
struct Sphere {
    center: vec3f,
    radius: f32,
    color: vec3f,
}


struct Intersection {
    normal: vec3f,
    t: f32,
    color: vec3f,
}

fn no_intersection() -> Intersection {
    return Intersection(vec3f(), -1.0, vec3f());
}

fn is_intersection_valid(hit: Intersection) -> bool {
    return hit.t > 0.0;
}


// Substituting the ray equation inside the sphere equation we get
//     (P + tD - C)·(P + tD - C) = r²
// We need to solve for `t`:
//     V = P - C
//     (tD + V)·(tD + V) = r²
//     (D·D)t² + 2(V·D)t + (V·V) - r² = 0
// This is a second order equation with
//     a = (D·D), b = 2(V·D), c = (V·V) - r²
//
fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersection {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    // Calculate discriminant
    let d = b * b - a * c;

    // Check if equation has solutions
    if d < 0.0 {
        return no_intersection();
    }

    let d_sqrt = sqrt(d);
    let a_inv = 1.0 / a;
    let b_neg = -b;

    // Calculate closest point
    let t1 = (b_neg - d_sqrt) * a_inv;
    let t2 = (b_neg + d_sqrt) * a_inv;
    let t = select(t2, t1, t1 > EPSILON);

    // Check if closest point is behind ray origin
    if t <= EPSILON {
        return no_intersection();
    }

    let p = point_on_ray(ray, t);
    let normal = (p - sphere.center) / sphere.radius;
    return Intersection(normal, t, sphere.color);
}

fn intersect_scene(ray: Ray) -> Intersection {
    var closest_hit = Intersection(vec3f(), FLT_MAX, vec3f());
    for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
        let sphere = scene[i];

        let hit = intersect_sphere(ray, sphere);
        if hit.t > 0.0 && hit.t < closest_hit.t {
            closest_hit = hit;
        }
    }
    if closest_hit.t < FLT_MAX {
        return closest_hit;
    }
    return no_intersection();
}

fn sky_color(ray: Ray) -> vec3f {
    let t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - t) * vec3f(1.0) + t * vec3f(0.3, 0.5, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    init_rng(vec2u(pos.xy));

    let origin = uniforms.camera.origin;
    let focus_distance = 1.;
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Offset and normalize viewport coords
    let offset = vec2f(rand_f32() - 0.5, rand_f32() - 0.5);
    var uv = (pos.xy + offset) / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

    // Map uv from normalized viewport coords (y-down) to camera coords
    uv = (2.0 * uv - vec2f(1.0)) * vec2f(aspect_ratio, -1.0);

    // Compute scene-space ray direction by rotating
    // the camera-space vector into a new basis
    let camera_rotation = mat3x3(uniforms.camera.u, uniforms.camera.v, uniforms.camera.w);
    let direction = camera_rotation * vec3(uv, focus_distance);

    var ray = Ray(origin, direction);
    var throughput = vec3f(1.0);
    var radiance_sample = vec3f();

    var path_lenght = 0u;
    while path_lenght < MAX_PATH_LENGTH {
        let hit = intersect_scene(ray);
        if !is_intersection_valid(hit) {
            // If no intersection return sky color
            radiance_sample += throughput * sky_color(ray);
            break;
        }

        let scattered = scatter(ray, hit);
        throughput *= scattered.attenuation;
        ray = scattered.ray;
        path_lenght += 1u;
    }

    // Fetch old sum of radiance sample
    var old_sum: vec3f;
    if uniforms.frame_count > 1 {
        old_sum = textureLoad(radiance_samples_old, vec2u(pos.xy), 0).xyz;
    } else {
        old_sum = vec3f();
    }

    // Compute and store new sum of radiance sample
    let new_sum = radiance_sample + old_sum;
    textureStore(radiance_samples_new, vec2u(pos.xy), vec4f(new_sum, 0.0));

    // Display average
    let color = new_sum / f32(uniforms.frame_count);
    return vec4f(color, 1.0);
}
