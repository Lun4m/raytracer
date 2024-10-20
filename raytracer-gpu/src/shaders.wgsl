const FLT_MAX: f32 = 3.40282346638528859812e+38;

const OBJECT_COUNT: u32 = 2;
alias Scene = array<Sphere, OBJECT_COUNT>;
var<private> scene: Scene = Scene(
    Sphere( /*center*/ vec3(0.0, 0.0, -1.0), /*radius*/ 0.5),
    Sphere( /*center*/ vec3(0.0, -100.5, -1.0), /*radius*/ 100.0),
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

struct Uniforms {
    width: u32,
    height: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

// The surface of a sphere can be described as
//     (X - C)·(X - C) = r²
// where `X` is a point on the sphere surface, `C` the sphere center,
// and `r` the radius.
struct Sphere {
    center: vec3f,
    radius: f32,
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
fn intersect_sphere(ray: Ray, sphere: Sphere) -> f32 {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    // Calculate discriminant
    let d = b * b - a * c;

    // Check if equation has solutions
    if d < 0.0 {
        return -1.0;
    }

    let d_sqrt = sqrt(d);
    let a_inv = 1.0 / a;
    let b_neg = -b;

    // Calculate closest point
    let t = (b_neg - d_sqrt) * a_inv;

    // Return if closest point is in front of ray origin
    if t > 0.0 {
        return t;
    }
    return (b_neg + d_sqrt) * a_inv;
}

// Our ray can be described as a simple line
//     Y = P + tD
// where P is the ray origin and D the ray direction.
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

    var closest_t = FLT_MAX;
    for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
        let t = intersect_sphere(ray, scene[i]);
        if t > 0.0 && t < closest_t {
            closest_t = t;
        }
    }
    if closest_t < FLT_MAX {
        return vec4(saturate(closest_t) * 0.5);
    }

    return vec4f(sky_color(ray), 1.0);
}
