struct Uniforms {
	aspect: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexIn {
	@builtin(vertex_index) vertex_index: u32,
}

struct VertexOut {
	@builtin(position) position: vec4f,
	@location(0) uv: vec2f,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
	let uv = vec2f(vec2u((in.vertex_index << 1) & 2, in.vertex_index & 2));
	let position = vec4f(uv * 2. - 1., 0., 1.);
	return VertexOut(position, uv * vec2f(uniforms.aspect, 1.));
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
	var p = in.uv;
	var i: i32 = 0;
	for (; i < 100; i = i + 1) {
		let d = p * p;
		if (d.x + d.y > 4.) {
			break;
		}

		let next = vec2f(d.x - d.y + in.uv.x, 2. * p.x * p.y + in.uv.y);
		p = next;
	}

	return vec4f(vec3f(f32(i) / 100.), 0.);
}
