use crate::*;
use core_util::cross;
use image::GenericImageView;
use vector::*;

pub fn gen_mips(mipmaps_enabled: bool, image: &image::DynamicImage) -> Vec<Vec<u8>> {
	let size: vec2u = image.dimensions().into();
	assert!(size.iter().all(|dim| dim.is_power_of_two()));

	let mut mips = vec![image.to_rgba8().to_vec()];

	if !mipmaps_enabled {
		return mips;
	}

	let mut size = size;
	let mut image = image.clone();

	while size.iter().all(|dim| dim > 1) {
		size = size / 2;
		// TODO: scale with wrapping, make sure SRGB is respected
		image = image.resize_exact(size.x(), size.y(), image::imageops::FilterType::Triangle);
		mips.push(image.to_rgba8().to_vec());
	}

	mips
}

pub fn gen_normal_mips(mipmaps_enabled: bool, normal_map: &Img<vec3f>) -> Vec<Vec<u8>> {
	let size: vec2u = normal_map.size();
	assert!(size.iter().all(|dim| dim.is_power_of_two()));

	let mut mips = vec![encode_normal_map(normal_map).into_raw()];

	if !mipmaps_enabled {
		return mips;
	}

	let mut normal_map = normal_map.clone();

	//LOG.write(format!("gen_normal_mips {}x{}", size.x(), size.y()));
	while size.iter().all(|dim| dim > 1) {
		normal_map = average_normals_2x2(&normal_map);
		mips.push(encode_normal_map(&normal_map).into_raw());
	}

	mips
}

fn average_normals_2x2(normal_map: &Img<vec3f>) -> Img<vec3f> {
	let orig_size = normal_map.size();
	let mut scaled = Img::<vec3f>::new(normal_map.size() / 2);

	for (x, y) in cross(0..orig_size.x(), 0..orig_size.y()) {
		let xy = vec2u(x, y);
		*scaled.at_mut(xy / 2) += normal_map.at(xy);
	}

	for n in scaled.pixels_mut() {
		n.normalize();
	}

	scaled
}
