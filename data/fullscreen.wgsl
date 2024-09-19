struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
    @location(1) normal: vec3f,
};

@vertex
fn vs_main(
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) texcoord: vec2f,
) -> VertexOutput {
    var result: VertexOutput;

    result.texcoord = texcoord;
    result.position = vec4f(position, 1.0);
    result.normal = normal;

    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4f {
    let uv = vertex.texcoord * 4 - 1;

    var ray: Ray;
    ray.origin = vec3f(0.0, 0.0, -3.0);
    ray.dir = normalize(vec3f(uv.x, uv.y, 1.0));

    return vec4f(trace_ray(ray), 1.0);
}

struct Uniforms {
    dir: vec3f
}

struct Ray {
    origin: vec3f,
    dir: vec3f,
}

fn trace_ray(ray: Ray) -> vec3f {
    let sun_pos = normalize(vec3f(0.7, -0.2, 0.4));
    let sphere_pos = vec3f(0.0, 0.0, 0.0);
    let sphere_radius = 1.0;

    let oc = ray.origin - sphere_pos;
    let a = dot(ray.dir, ray.dir);
    let b = dot(oc, ray.dir);
    let c = dot(oc, oc) - sphere_radius * sphere_radius;

    let d = b * b - a * c;

    if d > 0.0 {
        var t: f32;

        if b > 0.0 {
            t = (-b + sqrt(d)) / a;
        } else {
            t = (-b - sqrt(d)) / a;
        }

        if t > 0.0 {
            let t_pos = ray.origin + ray.dir * t;
            let normal = normalize(sphere_pos - t_pos);
            let color = vec3f(1.0, 1.0, 1.0) * saturate(dot(sun_pos, normal));
            return color;
        }
    }

    return vec3f(0.0);
}
