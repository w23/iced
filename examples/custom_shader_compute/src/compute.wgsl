@group(0) @binding(0) var output: texture_storage_2d<bgra8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3u) {
	let size = textureDimensions(output);
	let pix = vec2u(global_id.xy);

	if (pix.x >= size.x || pix.y >= size.y) {
		return;
	}

	textureStore(output, pix.xy, vec4f(0., 0., 1., 0.));
}
