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
    frame_count: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

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

// The surface of a sphere can be described as
//     (X - C)·(X - C) = r²
// where `X` is a point on the sphere surface, `C` the sphere center,
// and `r` the radius.
struct Sphere {
    center: vec3f,
    radius: f32,
}


struct Intersection {
    normal: vec3f,
    t: f32,
}

fn no_intersection() -> Intersection {
    return Intersection(vec3f(), -1.0);
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
    let t = select(t2, t1, t1 > 0);

    // Check if closest point is behind ray origin
    if t <= 0.0 {
        return no_intersection();
    }

    let p = point_on_ray(ray, t);
    let normal = (p - sphere.center) / sphere.radius;
    return Intersection(normal, t);
}


fn sky_color(ray: Ray) -> vec3f {
    let t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - t) * vec3f(1.0) + t * vec3f(0.3, 0.5, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let origin = vec3f();
    let focus_distance = 1.;
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Normalize viewport coords
    var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    // Map uv from normalized viewport coords (y-down) to camera coords
    uv = (2.0 * uv - vec2f(1.0)) * vec2f(aspect_ratio, -1.0);

    let direction = vec3f(uv, -focus_distance);
    let ray = Ray(origin, direction);

    var closest_hit = Intersection(vec3f(), FLT_MAX);
    for (var i = 0u; i < OBJECT_COUNT; i += 1u) {

        var sphere = scene[i];
        sphere.radius += sin(f32(uniforms.frame_count) * 0.02) * 0.2;

        let hit = intersect_sphere(ray, sphere);
        if hit.t > 0.0 && hit.t < closest_hit.t {
            closest_hit = hit;
        }
    }

    if closest_hit.t < FLT_MAX {
        // Color according to normal (shifting range from [0, 1] to [-1, 1])
        return vec4f(0.5 * closest_hit.normal + vec3f(0.5), 1.0);
    }

    return vec4f(sky_color(ray), 1.0);
}
