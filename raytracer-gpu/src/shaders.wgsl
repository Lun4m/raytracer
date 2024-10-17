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

struct Uniforms {
    width: u32,
    height: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Sphere {
    center: vec3f,
    radius: f32,
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> f32 {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    let d = b * b - a * c;
    if d < 0.0 {
        return -1.0;
    }

    let d_sqrt = sqrt(d);
    let a_inv = 1.0 / a;
    let b_neg = -b;

    let t = (b_neg - d_sqrt) * a_inv;
    if t < 0.0 {
        return t;
    }
    return (b_neg + d_sqrt) * a_inv;
}

struct Ray {
    origin: vec3f,
    direction: vec3f,
}

fn sky_color(ray: Ray) -> vec3f {
    let t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - t) * vec3(1.0) + t * vec3(0.3, 0.5, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let origin = vec3f();
    let focus_distance = 1.;
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Normalize viewport coords
    var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    // Map uv from normalized viewport coords (y-down) to camera coords
    uv = (2.0 * uv - vec2(1.0)) * vec2(aspect_ratio, -1.0);

    let direction = vec3(uv, -focus_distance);
    let ray = Ray(origin, direction);
    let sphere = Sphere( /*center*/ vec3(0.0, 0.0, -1.0), /*radius*/ 0.5);
    if intersect_sphere(ray, sphere) > 0.0 {
        return vec4(1.00, 0.76, 0.03, 1.00);
    }

    return vec4f(sky_color(ray), 1.0);
}
