use egui::{
    TextureHandle,
    TextureOptions,
    Widget,
    epaint::ImageDelta,
};
use serde::Deserialize;
use std::{collections::HashMap, f32, time::Instant};
use three_d::*;

use crate::{GLOW_CONTEXT, display::Icon};

const ADJUSTMENT_ANGLE: f32 = -f32::consts::PI; // For adjusting hexsphere coordinates
const HEXSPHERE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/surface/hexsphere.json"
));

#[derive(Deserialize)]
struct Tile {
    region_id: usize,
    boundary: Vec<[f32; 3]>,
    center: [f32; 3],
    offset: [f32; 3],
}

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
            .render(&camera, &globe.globe, &[]);

        render_target.render(
            &camera,
            &globe.highlighted_tiles,
            &[],
        );

        if globe.show_clouds {
            render_target.render(&camera, &globe.clouds, &[]);
        }

        render_target.render(&camera, &globe.disasters, &[]);
        render_target.render(&camera, &globe.pings, &[]);

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

fn icon_material(
    name: &str,
    bytes: &'static [u8],
    context: &three_d::Context,
) -> ColorMaterial {
    let icon = load_texture(name, bytes, false);
    let cpu_material = CpuMaterial {
        albedo: Srgba::WHITE,
        albedo_texture: Some(icon),
        ..Default::default()
    };
    let mut material =
        ColorMaterial::new_transparent(&context, &cpu_material);
    material.render_states.blend = Blend::Enabled {
        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
        destination_rgb_multiplier:
            BlendMultiplierType::OneMinusSrcAlpha,
        source_alpha_multiplier: BlendMultiplierType::One,
        destination_alpha_multiplier:
            BlendMultiplierType::OneMinusSrcAlpha,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };
    material
}

struct Globe {
    start_time: Instant,
    hexsphere: HashMap<usize, Vec<Tile>>,
    globe: Gm<Mesh, SurfaceMaterial>,
    clouds: Gm<Mesh, CloudsMaterial>,
    highlighted_tiles: Vec<Gm<Mesh, ColorMaterial>>,
    disasters: Vec<Gm<Sprites, ColorMaterial>>,
    pings: Vec<Gm<Sprites, ColorMaterial>>,
    ping_material: ColorMaterial,
    show_clouds: bool,
}

impl Globe {
    fn new(context: &three_d::Context) -> Self {
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

        let mut hexsphere: HashMap<usize, Vec<Tile>> =
            HashMap::default();
        let tiles =
            serde_json::from_str::<Vec<Tile>>(HEXSPHERE)
                .expect("valid hexsphere data");
        for tile in tiles {
            let ts =
                hexsphere.entry(tile.region_id).or_default();
            ts.push(tile);
        }

        let ping_material = icon_material(
            "discontent",
            include_bytes!(
                "../../assets/images/icons/pips/discontent.png"
            ),
            context,
        );

        Self {
            start_time: Instant::now(),
            globe,
            clouds,
            hexsphere,
            show_clouds: true,
            disasters: vec![],
            highlighted_tiles: vec![],
            pings: vec![],
            ping_material,
        }
    }
}

pub struct GlobeView {
    globe: Globe,
    renderer: Renderer,
    rotate: bool,
    angle: f32,
    camera_pos: Option<Vec3>,
    camera_distance: f32,
    context: three_d::Context,
}
impl GlobeView {
    pub fn new(size: u32, camera_distance: f32) -> Self {
        let ctx = GLOW_CONTEXT
            .get()
            .expect("glow context initialized");
        let context =
            three_d::Context::from_gl_context(ctx.clone())
                .unwrap();

        Self {
            globe: Globe::new(&context),
            renderer: Renderer::new(&context, size),
            context,
            rotate: true,
            angle: 0.,
            camera_pos: None,
            camera_distance,
        }
    }

    pub fn hide_clouds(&mut self) {
        self.globe.show_clouds = false;
    }

    pub fn dont_rotate(&mut self) {
        self.rotate = false;
    }

    fn look_at_pos(&mut self, pos: Vec3) {
        let rotation =
            Mat4::from_angle_y(Rad(ADJUSTMENT_ANGLE));
        let target = (rotation * pos.extend(1.0)).truncate();
        let dir = target.normalize();
        let pos = dir * self.camera_distance;
        self.camera_pos = Some(pos);
    }

    pub fn highlight_region(&mut self, region_idx: usize) {
        self.globe.highlighted_tiles.clear();
        let mut points = vec![];
        if let Some(tiles) =
            self.globe.hexsphere.get(&region_idx)
        {
            for tile in tiles {
                let cpu_material = CpuMaterial {
                    albedo: Srgba::new(0xeb, 0x40, 0x34, 0xAA),
                    ..Default::default()
                };
                let material = ColorMaterial::new_transparent(
                    &self.context,
                    &cpu_material,
                );
                let mesh = generate_tile_mesh(
                    &self.context,
                    &tile.boundary,
                    material,
                );
                self.globe.highlighted_tiles.push(mesh);
                points.push(tile.center);
            }

            let pos = Vec3::new(
                points[0][0],
                points[0][1],
                points[0][2],
            );
            self.look_at_pos(pos);
        }
    }

    pub fn show_event(
        &mut self,
        region_idx: usize,
        icon: Icon,
        intensity: usize,
    ) {
        if let Some(bytes) = icon.bytes()
            && let Some(tiles) =
                self.globe.hexsphere.get(&region_idx)
        {
            if let Some(tile) = fastrand::choice(tiles) {
                let material = icon_material(
                    &icon.to_string(),
                    bytes,
                    &self.context,
                );

                let p = Vec3::new(
                    tile.center[0] + tile.offset[0],
                    tile.center[1] + tile.offset[1],
                    tile.center[2] + tile.offset[2],
                ) * 1.05;
                let rotation =
                    Mat4::from_angle_y(Rad(ADJUSTMENT_ANGLE));
                let target =
                    (rotation * p.extend(1.0)).truncate();
                let mut icon = Sprites::new(
                    &self.context,
                    &[target],
                    None,
                );
                icon.set_transformation(Mat4::from_scale(0.1));
                let sprite = Gm::new(icon, material);
                self.globe.disasters.push(sprite);

                let targets: Vec<_> = (0..intensity)
                    .map(|_| {
                        let jitter = Vec3::new(
                            (fastrand::f32() - 0.5) * 0.1,
                            (fastrand::f32() - 0.5) * 0.1,
                            (fastrand::f32() - 0.5) * 0.1,
                        );
                        target + jitter
                    })
                    .collect();
                let mut pings =
                    Sprites::new(&self.context, &targets, None);
                pings
                    .set_transformation(Mat4::from_scale(0.08));
                let sprite = Gm::new(
                    pings,
                    self.globe.ping_material.clone(),
                );
                self.globe.pings.push(sprite);
            }
        }
    }
}

impl Widget for &mut GlobeView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let elapsed =
            self.globe.start_time.elapsed().as_secs_f32();
        self.globe.clouds.material.time = elapsed * 1e3;

        if self.rotate {
            let rotation_speed = 0.1; // radians per second
            self.angle = rotation_speed * elapsed;
        }

        const UP_PER_SEC: f32 = 0.008;
        let dy = UP_PER_SEC * elapsed;
        self.globe.pings.retain_mut(|ping| {
            ping.material.color.a =
                ping.material.color.a.saturating_sub(5);
            let mat = ping.transformation();
            let mat = mat
                * Mat4::from_translation(vec3(0.0, dy, 0.0));
            ping.set_transformation(mat);

            ping.material.color.a != 0
        });

        let camera_pos = self.camera_pos.unwrap_or_else(|| {
            let rotation = Mat4::from_angle_y(Rad(self.angle));
            rotation.transform_vector(vec3(
                0.0,
                0.0,
                self.camera_distance,
            ))
        });
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

pub fn generate_tile_mesh(
    context: &Context,
    vertices: &[[f32; 3]],
    material: ColorMaterial,
) -> Gm<Mesh, ColorMaterial> {
    let positions = Positions::F32(
        vertices
            .iter()
            .map(|v| {
                let v = Vec3::new(v[0], v[1], v[2]);
                let rotation =
                    Mat4::from_angle_y(Rad(ADJUSTMENT_ANGLE));
                (rotation * v.extend(1.0)).truncate()
            })
            .collect(),
    );

    let indices = if vertices.len() > 5 {
        // hexagon
        Indices::U32(vec![0, 1, 2, 0, 2, 3, 3, 5, 0, 3, 4, 5])
    } else {
        // pentagon
        Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4])
    };

    let cpu = CpuMesh {
        positions,
        indices: indices,
        ..Default::default()
    };

    let mesh = Mesh::new(context, &cpu);
    Gm::new(mesh, material)
}
