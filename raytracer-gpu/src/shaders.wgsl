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

struct Ray {
    origin: vec3f,
    direction: vec3f,
}

fn sky_color(ray: Ray) -> vec3f {
    let t = 0.5 * (normalize(ray.direction).y + 1.);
    return (1. - t) * vec3(1.) + t * vec3(0.3, 0.5, 1.);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let origin = vec3(0.);
    let focus_distance = 1.;
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

    // Normalize viewport coords
    var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    // Map uv from normalized viewport coords (y-down) to camera coords
    uv = (2. * uv - vec2(1.)) * vec2(aspect_ratio, -1.);

    let direction = vec3(uv, -focus_distance);
    let ray = Ray(origin, direction);

    return vec4f(sky_color(ray), 1.0);
}