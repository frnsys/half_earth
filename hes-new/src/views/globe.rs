use egui::{
    TextureHandle,
    TextureOptions,
    Widget,
    epaint::ImageDelta,
};
use hexasphere::shapes::IcoSphere;
use std::{sync::Arc, time::Instant};
use three_d::*;

struct Renderer {
    color_texture: Texture2D,
    depth_texture: DepthTexture2D,
    texture_id: Option<TextureHandle>,
}
impl Renderer {
    fn new(context: &three_d::Context, size: u32) -> Self {
        let color_texture = Texture2D::new_empty::<[u8; 4]>(
            context,
            size,
            size,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let depth_texture = DepthTexture2D::new::<f32>(
            context,
            size,
            size,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        Self {
            color_texture,
            depth_texture,
            texture_id: None,
        }
    }

    fn render(
        &mut self,
        ctx: &egui::Context,
        camera_pos: Vector3<f32>,
        globe: &Globe,
    ) -> &TextureHandle {
        let color = self.color_texture.as_color_target(None);
        let depth = self.depth_texture.as_depth_target();
        let render_target = RenderTarget::new(color, depth);

        let camera = Camera::new_orthographic(
            render_target.viewport(),
            camera_pos,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            0.01,
            0.0,
            1e4,
        );

        render_target
            .clear(ClearState::color_and_depth(
                0.0, 0.0, 0.0, 0.0, 1.0,
            ))
            .render(&camera, &globe.globe, &[])
            .render(&camera, &globe.clouds, &[]);
        // .render(&camera, &globe.sprite, &[]);

        let data = render_target
            .read_color::<[u8; 4]>()
            .into_flattened();

        let size = [
            render_target.width() as usize,
            render_target.height() as usize,
        ];
        let img = egui::ColorImage::from_rgba_unmultiplied(
            size, &data,
        );
        if self.texture_id.is_none() {
            self.texture_id = Some(ctx.load_texture(
                "globe-render-texture",
                img,
                TextureOptions::NEAREST,
            ));
        } else {
            ctx.tex_manager().write().set(
                self.texture_id.as_ref().unwrap().id(),
                ImageDelta::full(img, TextureOptions::NEAREST),
            );
        }
        self.texture_id.as_ref().expect("handle exists")
    }
}

fn load_texture(
    name: &str,
    bytes: &'static [u8],
    flip: bool,
) -> CpuTexture {
    let bytes = std::io::Cursor::new(bytes);
    let mut img = image::ImageReader::new(bytes)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    if flip {
        img = img.flipv().fliph();
    }
    let img = img.to_rgba8();
    let (w, h) = img.dimensions();
    CpuTexture {
        name: name.into(),
        data: TextureData::RgbaU8(
            img.as_raw()
                .chunks_exact(4)
                .map(|chunk| {
                    [chunk[0], chunk[1], chunk[2], chunk[3]]
                })
                .collect(),
        ),
        width: w,
        height: h,
        min_filter: Interpolation::Nearest,
        mag_filter: Interpolation::Nearest,
        mipmap: None,
        wrap_s: Wrapping::ClampToEdge,
        wrap_t: Wrapping::ClampToEdge,
    }
}

struct Globe {
    start_time: Instant,
    globe: Gm<Mesh, SurfaceMaterial>,
    clouds: Gm<Mesh, CloudsMaterial>,
    sprite: Gm<Sprites, ColorMaterial>,
}

impl Globe {
    fn new(context: &three_d::Context) -> Self {
        let sphere = IcoSphere::new(32, |_| ());
        let icon = load_texture(
            "icon-test",
            include_bytes!(
                "../../assets/globe/icons/wildfires.png"
            ),
            false,
        );
        let cpu_material = CpuMaterial {
            albedo: Srgba::WHITE,
            albedo_texture: Some(icon),
            ..Default::default()
        };
        let mut material = ColorMaterial::new_transparent(
            &context,
            &cpu_material,
        );
        material.render_states.write_mask = WriteMask::COLOR;
        material.render_states.depth_test =
            DepthTest::LessOrEqual;
        material.render_states.blend = Blend::TRANSPARENCY;
        let points: Vec<_> = sphere
            .raw_points()
            .into_iter()
            .enumerate()
            .filter_map(|(i, pt)| {
                if i % 127 == 0 {
                    let scale = 1.1;
                    Some(vec3(
                        pt.x * scale,
                        pt.y * scale,
                        pt.z * scale,
                    ))
                } else {
                    None
                }
            })
            .collect();

        let mut sprites = Sprites::new(&context, &points, None);
        sprites.set_transformation(Mat4::from_scale(0.1));
        let sprite = Gm::new(sprites, material);

        let shadows = Texture2D::new(
            &context,
            &load_texture(
                "shadows",
                include_bytes!(
                    "../../assets/globe/shadows.png"
                ),
                true,
            ),
        );
        let satellite = Texture2D::new(
            &context,
            &load_texture(
                "satellite",
                include_bytes!(
                    "../../assets/globe/satellite.bw.jpg"
                ),
                true,
            ),
        );
        let biomes = Texture2D::new(
            &context,
            &load_texture(
                "biomes",
                include_bytes!(
                    "../../assets/globe/static_surface.png"
                ),
                true,
            ),
        );
        let mesh = uv_sphere(32, 1.);
        let mesh = Mesh::new(&context, &mesh);
        let material = SurfaceMaterial {
            shadows,
            satellite,
            biomes,
        };
        let globe = Gm::new(mesh, material);

        let material = CloudsMaterial { time: 0.0 };
        let mesh = uv_sphere(32, 1.02);
        let mesh = Mesh::new(&context, &mesh);
        let clouds = Gm::new(mesh, material);

        Self {
            start_time: Instant::now(),
            globe,
            clouds,
            sprite,
        }
    }
}

pub struct GlobeView {
    globe: Globe,
    renderer: Renderer,
}
impl GlobeView {
    pub fn new(
        ctx: Arc<eframe::glow::Context>,
        size: u32,
    ) -> Self {
        let context =
            three_d::Context::from_gl_context(ctx).unwrap();
        Self {
            globe: Globe::new(&context),
            renderer: Renderer::new(&context, size),
        }
    }
}
impl Widget for &mut GlobeView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let elapsed =
            self.globe.start_time.elapsed().as_secs_f32();
        self.globe.clouds.material.time = elapsed * 1e3;

        let rotation_speed = 0.1; // radians per second
        let angle = rotation_speed * elapsed;
        let rotation = Mat4::from_angle_y(Rad(angle));
        let camera_pos =
            rotation.transform_vector(vec3(0.0, 0.0, 200.));
        let texture = self.renderer.render(
            ui.ctx(),
            camera_pos,
            &self.globe,
        );
        let resp = ui.image(texture);
        ui.ctx().request_repaint();
        resp
    }
}

struct CloudsMaterial {
    time: f32,
}
impl Material for CloudsMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(1)
    }

    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
    ) -> String {
        include_str!("../../assets/globe/shaders/clouds.frag")
            .to_string()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _viewer: &dyn Viewer,
        _lights: &[&dyn Light],
    ) {
        program.use_uniform("time", &self.time);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}

struct SurfaceMaterial {
    shadows: Texture2D,
    satellite: Texture2D,
    biomes: Texture2D,
}

impl Material for SurfaceMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(0)
    }

    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
    ) -> String {
        include_str!("../../assets/globe/shaders/surface.frag")
            .to_string()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _viewer: &dyn Viewer,
        _lights: &[&dyn Light],
    ) {
        program.use_texture("shadows", &self.shadows);
        program.use_texture("satTexture", &self.satellite);
        program.use_texture("biomesTexture", &self.biomes);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR_AND_DEPTH,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

fn uv_sphere(subdiv: usize, scale: f32) -> CpuMesh {
    let stacks = subdiv;
    let slices = subdiv * 2;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for stack in 0..=stacks {
        let phi = std::f32::consts::PI * (stack as f32)
            / (stacks as f32);
        let y = phi.cos();
        let r = phi.sin();

        for slice in 0..=slices {
            let theta =
                2.0 * std::f32::consts::PI * (slice as f32)
                    / (slices as f32);
            let x = r * theta.cos();
            let z = r * theta.sin();

            positions.push(Vector3::new(
                x * scale,
                y * scale,
                z * scale,
            ));
            normals.push(Vector3::new(x, y, z).normalize());

            // U = [0, 1], V = [0, 1]
            let u = (slice as f32) / (slices as f32);
            let v = 1.0 - (stack as f32) / (stacks as f32);
            uvs.push(Vector2::new(u, v));
        }
    }

    // Generate triangle indices
    for stack in 0..stacks {
        for slice in 0..slices {
            let first = stack * (slices + 1) + slice;
            let second = first + slices + 1;

            indices.push(first as u32);
            indices.push(second as u32);
            indices.push((first + 1) as u32);

            indices.push(second as u32);
            indices.push((second + 1) as u32);
            indices.push((first + 1) as u32);
        }
    }

    CpuMesh {
        positions: Positions::F32(positions),
        normals: Some(normals),
        uvs: Some(uvs),
        indices: Indices::U32(indices),
        ..Default::default()
    }
}
