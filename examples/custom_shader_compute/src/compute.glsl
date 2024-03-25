#version 460
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
layout(set = 0, binding = 0, rgba8) uniform writeonly image2D target;

void main() {
	const ivec2 size = imageSize(target);
	const ivec2 pix = ivec2(gl_GlobalInvocationID.xy);

	if (any(greaterThanEqual(pix, size))) {
		return;
	}

	imageStore(target, pix, vec4(1,1,0,0));
}
