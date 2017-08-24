use std::cmp;
use std::collections::hash_map::{HashMap, Entry};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use cgmath::{Vector3, Transform as Transform_};
use genmesh::{Polygon, EmitTriangles, Triangulate, Vertex as GenVertex};
use genmesh::generators::{self, IndexedPolygon, SharedVertex};
use gfx;
use gfx::format::I8Norm;
use gfx::handle as h;
use gfx::traits::{Factory as Factory_, FactoryExt};
use image;
use mint;
use obj;

use camera::{Orthographic, Perspective};
use render::{BackendFactory, BackendResources, GpuData, DynamicData,
             Vertex, ShadowFormat};
use scene::{Color, Background, Group, Sprite, Material,
            AmbientLight, DirectionalLight, HemisphereLight, PointLight};
use {Hub, HubPtr, SubLight, Node, SubNode,
     LightData, Object, Scene, Camera, Mesh, DynamicMesh};


const NORMAL_Z: [I8Norm; 4] = [I8Norm(0), I8Norm(0), I8Norm(1), I8Norm(0)];

const QUAD: [Vertex; 4] = [
    Vertex {
        pos: [-1.0, -1.0, 0.0, 1.0],
        uv: [0.0, 0.0],
        normal: NORMAL_Z,
    },
    Vertex {
        pos: [1.0, -1.0, 0.0, 1.0],
        uv: [1.0, 0.0],
        normal: NORMAL_Z,
    },
    Vertex {
        pos: [-1.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
        normal: NORMAL_Z,
    },
    Vertex {
        pos: [1.0, 1.0, 0.0, 1.0],
        uv: [1.0, 1.0],
        normal: NORMAL_Z,
    },
];


impl From<SubNode> for Node {
    fn from(sub: SubNode) -> Self {
        Node {
            visible: true,
            world_visible: false,
            transform: Transform_::one(),
            world_transform: Transform_::one(),
            parent: None,
            scene_id: None,
            sub_node: sub,
        }
    }
}

impl Hub {
    fn spawn(&mut self, sub: SubNode) -> Object {
        Object {
            node: self.nodes.create(sub.into()),
            tx: self.message_tx.clone(),
        }
    }

    fn spawn_empty(&mut self) -> Object {
        self.spawn(SubNode::Empty)
    }

    fn spawn_visual(&mut self, mat: Material, gpu_data: GpuData) -> Object {
        self.spawn(SubNode::Visual(mat, gpu_data))
    }

    fn spawn_light(&mut self, data: LightData) -> Object {
        self.spawn(SubNode::Light(data))
    }
}

/// `ShadowMap` is used to render shadows from [`PointLight`](struct.PointLight.html)
/// and [`DirectionalLight`](struct.DirectionalLight.html).
#[derive(Clone, Debug)]
pub struct ShadowMap {
    resource: gfx::handle::ShaderResourceView<BackendResources, f32>,
    target: gfx::handle::DepthStencilView<BackendResources, ShadowFormat>,
}

impl ShadowMap {
    #[doc(hidden)]
    pub fn to_target(&self) -> gfx::handle::DepthStencilView<BackendResources, ShadowFormat> {
        self.target.clone()
    }

    #[doc(hidden)]
    pub fn to_resource(&self) -> gfx::handle::ShaderResourceView<BackendResources, f32> {
        self.resource.clone()
    }
}


pub type SceneId = usize;

/// `Factory` is used to instantiate game objects.
pub struct Factory {
    backend: BackendFactory,
    scene_id: SceneId,
    hub: HubPtr,
    quad_buf: gfx::handle::Buffer<BackendResources, Vertex>,
    texture_cache: HashMap<String, Texture<[f32; 4]>>,
}

impl Factory {
    #[doc(hidden)]
    pub fn new(mut backend: BackendFactory) -> Self {
        let quad_buf = backend.create_vertex_buffer(&QUAD);
        Factory {
            backend: backend,
            scene_id: 0,
            hub: Hub::new(),
            quad_buf,
            texture_cache: HashMap::new(),
        }
    }

    /// Create new empty [`Scene`](struct.Scene.html).
    pub fn scene(&mut self) -> Scene {
        self.scene_id += 1;
        let mut hub = self.hub.lock().unwrap();
        let node = hub.nodes.create(Node {
            scene_id: Some(self.scene_id),
            .. SubNode::Empty.into()
        });
        Scene {
            unique_id: self.scene_id,
            node: node,
            tx: hub.message_tx.clone(),
            hub: self.hub.clone(),
            background: Background::Color(0),
        }
    }

    /// Create new [Orthographic](https://en.wikipedia.org/wiki/Orthographic_projection) Camera.
    /// It's used basically to render 2D.
    pub fn orthographic_camera<P>(&mut self, center: P,
                               extent_y: f32, near: f32, far: f32)
                               -> Camera<Orthographic>
    where P: Into<mint::Point2<f32>>
    {
        Camera {
            object: self.hub.lock().unwrap().spawn_empty(),
            projection: Orthographic {
                center: center.into(),
                extent_y, near, far,
            },
        }
    }

    /// Create new [Perspective](https://en.wikipedia.org/wiki/Perspective_(graphical)) Camera.
    /// It's used basically to render 3D.
    pub fn perspective_camera(&mut self, fov_y: f32, near: f32, far: f32)
                              -> Camera<Perspective> {
        Camera {
            object: self.hub.lock().unwrap().spawn_empty(),
            projection: Perspective {
                fov_y, near, far,
            },
        }
    }

    /// Create empty [`Group`](struct.Group.html).
    pub fn group(&mut self) -> Group {
        Group::new(self.hub.lock().unwrap().spawn_empty())
    }

    fn mesh_vertices(shape: &GeometryShape) -> Vec<Vertex> {
        if shape.normals.is_empty() {
            shape.vertices.iter().map(|v| Vertex {
                pos: [v.x, v.y, v.z, 1.0],
                uv: [0.0, 0.0], //TODO
                normal: NORMAL_Z,
            }).collect()
        } else {
            let f2i = |x: f32| I8Norm(cmp::min(cmp::max((x * 127.) as isize, -128), 127) as i8);
            shape.vertices.iter().zip(shape.normals.iter()).map(|(v, n)| Vertex {
                pos: [v.x, v.y, v.z, 1.0],
                uv: [0.0, 0.0], //TODO
                normal: [f2i(n.x), f2i(n.y), f2i(n.z), I8Norm(0)],
            }).collect()
        }
    }

    /// Create new `Mesh` with desired `Geometry` and `Material`.
    pub fn mesh(&mut self, geometry: Geometry, mat: Material) -> Mesh {
        let vertices = Self::mesh_vertices(&geometry.base_shape);
        let cbuf = self.backend.create_constant_buffer(1);
        let (vbuf, slice) = if geometry.faces.is_empty() {
            self.backend.create_vertex_buffer_with_slice(&vertices, ())
        } else {
            let faces: &[u16] = gfx::memory::cast_slice(&geometry.faces);
            self.backend.create_vertex_buffer_with_slice(&vertices, faces)
        };
        Mesh {
            object: self.hub.lock().unwrap().spawn_visual(mat, GpuData {
                slice,
                vertices: vbuf,
                constants: cbuf,
                pending: None,
            }),
        }
    }

    /// Create a new `DynamicMesh` with desired `Geometry` and `Material`.
    pub fn mesh_dynamic(&mut self, geometry: Geometry, mat: Material) -> DynamicMesh {
        let slice = {
            let data: &[u16] = gfx::memory::cast_slice(&geometry.faces);
            gfx::Slice {
                start: 0,
                end: data.len() as u32,
                base_vertex: 0,
                instances: None,
                buffer: self.backend.create_index_buffer(data),
            }
        };
        let (num_vertices, vertices) = {
            let data = Self::mesh_vertices(&geometry.base_shape);
            let buf = self.backend.create_buffer_immutable(&data,
                gfx::buffer::Role::Vertex, gfx::memory::TRANSFER_DST).unwrap();
            (data.len(), buf)
        };
        let constants = self.backend.create_constant_buffer(1);

        DynamicMesh {
            object: self.hub.lock().unwrap().spawn_visual(mat, GpuData {
                slice,
                vertices,
                constants,
                pending: None,
            }),
            geometry,
            dynamic: DynamicData {
                num_vertices,
                buffer: self.backend.create_upload_buffer(num_vertices).unwrap(),
            },
        }
    }

    /// Create a `Mesh` sharing the geometry with another one.
    /// Rendering a sequence of meshes with the same geometry is faster.
    pub fn mesh_instance(&mut self, template: &Mesh, mat: Material) -> Mesh {
        let mut hub = self.hub.lock().unwrap();
        let gpu_data = match hub.nodes[&template.node].sub_node {
            SubNode::Visual(_, ref gpu) => GpuData {
                constants: self.backend.create_constant_buffer(1),
                .. gpu.clone()
            },
            _ => unreachable!()
        };
        Mesh {
            object: hub.spawn_visual(mat, gpu_data),
        }
    }

    /// Create new sprite from `Material`.
    pub fn sprite(&mut self, mat: Material) -> Sprite {
        Sprite::new(self.hub.lock().unwrap().spawn_visual(mat, GpuData {
            slice: gfx::Slice::new_match_vertex_buffer(&self.quad_buf),
            vertices: self.quad_buf.clone(),
            constants: self.backend.create_constant_buffer(1),
            pending: None,
        }))
    }

    /// Create new `AmbientLight`.
    pub fn ambient_light(&mut self, color: Color, intensity: f32) -> AmbientLight {
        AmbientLight::new(self.hub.lock().unwrap().spawn_light(LightData {
            color,
            intensity,
            sub_light: SubLight::Ambient,
            shadow: None,
        }))
    }

    /// Create new `DirectionalLight`.
    pub fn directional_light(&mut self, color: Color, intensity: f32) -> DirectionalLight {
        DirectionalLight::new(self.hub.lock().unwrap().spawn_light(LightData {
            color,
            intensity,
            sub_light: SubLight::Directional,
            shadow: None,
        }))
    }

    /// Create new `HemisphereLight`.
    pub fn hemisphere_light(&mut self, sky_color: Color, ground_color: Color,
                            intensity: f32) -> HemisphereLight {
        HemisphereLight::new(self.hub.lock().unwrap().spawn_light(LightData {
            color: sky_color,
            intensity,
            sub_light: SubLight::Hemisphere{ ground: ground_color },
            shadow: None,
        }))
    }

    /// Create new `PointLight`.
    pub fn point_light(&mut self, color: Color, intensity: f32) -> PointLight {
        PointLight::new(self.hub.lock().unwrap().spawn_light(LightData {
            color,
            intensity,
            sub_light: SubLight::Point,
            shadow: None,
        }))
    }

    /// Create new `ShadowMap`.
    pub fn shadow_map(&mut self, width: u16, height: u16) -> ShadowMap {
        let (_, resource, target) = self.backend.create_depth_stencil::<ShadowFormat>(
            width, height).unwrap();
        ShadowMap {
            resource,
            target,
        }
    }
}


/// A shape of geometry that is used for mesh blending.
#[derive(Clone, Debug)]
pub struct GeometryShape {
    /// Vertices.
    pub vertices: Vec<mint::Point3<f32>>,
    /// Normals.
    pub normals: Vec<mint::Vector3<f32>>,
}

impl GeometryShape {
    /// Create an empty shape.
    pub fn empty() -> Self {
        GeometryShape {
            vertices: Vec::new(),
            normals: Vec::new(),
        }
    }
}

/// A collection of vertices, their normals, and faces that defines the
/// shape of a polyhedral object.
#[derive(Clone, Debug)]
pub struct Geometry {
    /// The original shape of geometry.
    pub base_shape: GeometryShape,
    /// A map containing blend shapes and their names.
    pub shapes: HashMap<String, GeometryShape>,
    /// Faces.
    pub faces: Vec<[u16; 3]>,
}

impl Geometry {
    /// Create new `Geometry` without any data in it.
    pub fn empty() -> Self {
        Geometry {
            base_shape: GeometryShape::empty(),
            shapes: HashMap::new(),
            faces: Vec::new(),
        }
    }

    /// Create `Geometry` from vector of vertices.
    pub fn from_vertices(vertices: Vec<mint::Point3<f32>>) -> Self {
        Geometry {
            base_shape: GeometryShape {
                vertices,
                normals: Vec::new(),
            },
            .. Geometry::empty()
        }
    }

    fn generate<P, G, Fpos, Fnor>(gen: G, fpos: Fpos, fnor: Fnor) -> Self where
        P: EmitTriangles<Vertex=usize>,
        G: IndexedPolygon<P> + SharedVertex<GenVertex>,
        Fpos: Fn(GenVertex) -> mint::Point3<f32>,
        Fnor: Fn(GenVertex) -> mint::Vector3<f32>,
    {
        Geometry {
            base_shape: GeometryShape {
                vertices: gen.shared_vertex_iter()
                             .map(fpos)
                             .collect(),
                normals: gen.shared_vertex_iter()
                            .map(fnor)
                            .collect(),
            },
            shapes: HashMap::new(),
            faces: gen.indexed_polygon_iter()
                       .triangulate()
                       .map(|t| [t.x as u16, t.y as u16, t.z as u16])
                       .collect(),
        }
    }

    /// Create new Plane with desired size.
    pub fn new_plane(sx: f32, sy: f32) -> Self {
        Self::generate(generators::Plane::new(),
            |GenVertex{ pos, ..}| {
                [pos[0] * 0.5 * sx, pos[1] * 0.5 * sy, 0.0].into()
            },
            |v| v.normal.into()
        )
    }

    /// Create new Box with desired size.
    pub fn new_box(sx: f32, sy: f32, sz: f32) -> Self {
        Self::generate(generators::Cube::new(),
            |GenVertex{ pos, ..}| {
                [pos[0] * 0.5 * sx, pos[1] * 0.5 * sy, pos[2] * 0.5 * sz].into()
            },
            |v| v.normal.into()
        )
    }

    /// Create new Cylinder or Cone with desired top and bottom radius, height
    /// and number of segments.
    pub fn new_cylinder(radius_top: f32, radius_bottom: f32, height: f32,
                        radius_segments: usize) -> Self
    {
        Self::generate(generators::Cylinder::new(radius_segments),
            //Three.js has height along the Y axis for some reason
            |GenVertex{ pos, ..}| {
                let scale = (pos[2] + 1.0) * 0.5 * radius_top +
                            (1.0 - pos[2]) * 0.5 * radius_bottom;
                [pos[1] * scale, pos[2] * 0.5 * height, pos[0] * scale].into()
            },
            |GenVertex{ normal, ..}| {
                [normal[1], normal[2], normal[0]].into()
            },
        )
    }

    /// Create new Sphere with desired radius and number of segments.
    pub fn new_sphere(radius: f32, width_segments: usize,
                      height_segments: usize) -> Self
    {
        Self::generate(generators::SphereUV::new(width_segments, height_segments),
            |GenVertex{ pos, ..}| {
                [pos[0] * radius, pos[1] * radius, pos[2] * radius].into()
            },
            |v| v.normal.into()
        )
    }
}


/// An image applied (mapped) to the surface of a shape or polygon.
#[derive(Clone, Debug)]
pub struct Texture<T> {
    view: h::ShaderResourceView<BackendResources, T>,
    sampler: h::Sampler<BackendResources>,
    total_size: [u32; 2],
    tex0: [f32; 2],
    tex1: [f32; 2],
}

impl<T> Texture<T> {
    #[doc(hidden)]
    pub fn new(view: h::ShaderResourceView<BackendResources, T>,
               sampler: h::Sampler<BackendResources>,
               total_size: [u32; 2]) -> Self {
        Texture {
            view,
            sampler,
            total_size,
            tex0: [0.0; 2],
            tex1: [0.0; 2],
        }
    }

    #[doc(hidden)]
    pub fn to_param(&self) -> (h::ShaderResourceView<BackendResources, T>, h::Sampler<BackendResources>) {
        (self.view.clone(), self.sampler.clone())
    }

    /// See [`Sprite::set_texel_range`](struct.Sprite.html#method.set_texel_range).
    pub fn set_texel_range(&mut self, base: mint::Point2<i16>, size: mint::Vector2<u16>) {
        self.tex0 = [
            base.x as f32,
            self.total_size[1] as f32 - base.y as f32 - size.y as f32,
        ];
        self.tex1 = [
            base.x as f32 + size.x as f32,
            self.total_size[1] as f32 - base.y as f32,
        ];
    }

    /// Returns normalized UV rectangle (x0, y0, x1, y1) of the current texel range.
    pub fn get_uv_range(&self) -> [f32; 4] {
        [self.tex0[0] / self.total_size[0] as f32,
         self.tex0[1] / self.total_size[1] as f32,
         self.tex1[0] / self.total_size[0] as f32,
         self.tex1[1] / self.total_size[1] as f32]
    }
}


impl Factory {
    fn load_texture_impl(path_str: &str, factory: &mut BackendFactory) -> Texture<[f32; 4]> {
        use gfx::texture as t;
        use image::ImageFormat as F;

        let path = Path::new(path_str);
        let extension = path.extension()
                            .expect("no extension for an image?")
                            .to_string_lossy()
                            .to_lowercase();
        let format = match extension.as_str() {
            "png" => F::PNG,
            "jpg" | "jpeg" => F::JPEG,
            "gif" => F::GIF,
            "webp" => F::WEBP,
            "ppm" => F::PPM,
            "tiff" => F::TIFF,
            "tga" => F::TGA,
            "bmp" => F::BMP,
            "ico" => F::ICO,
            "hdr" => F::HDR,
            _ => panic!("Unrecognized image extension: {}", extension),
        };

        let file = File::open(&path)
                        .unwrap_or_else(|e| panic!("Unable to open {}: {:?}", path_str, e));
        let img = image::load(BufReader::new(file), format)
                        .unwrap_or_else(|e| panic!("Unable to decode {}: {:?}", path_str, e))
                        .flipv().to_rgba();
        let (width, height) = img.dimensions();
        let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
        let (_, view) = factory.create_texture_immutable_u8::<gfx::format::Srgba8>(kind, &[&img])
                               .unwrap_or_else(|e| panic!("Unable to create GPU texture for {}: {:?}", path_str, e));

        Texture::new(view, factory.create_sampler_linear(), [width, height])
    }

    fn request_texture(&mut self, path: &str) -> Texture<[f32; 4]> {
        match self.texture_cache.entry(path.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                let tex = Self::load_texture_impl(path, &mut self.backend);
                e.insert(tex.clone());
                tex
            }
        }
    }

    fn load_obj_material(&mut self, mat: &obj::Material, has_normals: bool, has_uv: bool) -> Material {
        let cf2u = |c: [f32; 3]| { c.iter().fold(0, |u, &v|
            (u << 8) + cmp::min((v * 255.0) as u32, 0xFF)
        )};
        match *mat {
            obj::Material { kd: Some(color), ns: Some(glossiness), .. } if has_normals =>
                Material::MeshPhong { color: cf2u(color), glossiness },
            obj::Material { kd: Some(color), .. } if has_normals =>
                Material::MeshLambert { color: cf2u(color), flat: false },
            obj::Material { kd: Some(color), ref map_kd, .. } =>
                Material::MeshBasic {
                    color: cf2u(color),
                    map: match (has_uv, map_kd) {
                        (true, &Some(ref name)) => Some(self.request_texture(name)),
                        _ => None,
                    },
                    wireframe: false,
                },
            _ => Material::MeshBasic { color: 0xffffff, map: None, wireframe: true },
        }
    }

    /// Load texture from file.
    /// Supported file formats are: PNG, JPEG, GIF, WEBP, PPM, TIFF, TGA, BMP, ICO, HDR.
    pub fn load_texture(&mut self, path_str: &str) -> Texture<[f32; 4]> {
        self.request_texture(path_str)
    }

    /// Load mesh from Wavefront Obj format.
    /// #### Note
    /// You must store `Vec<Mesh>` somewhere to keep them alive.
    pub fn load_obj(&mut self, path_str: &str) -> (HashMap<String, Group>, Vec<Mesh>) {
        use std::path::Path;
        use genmesh::{LruIndexer, Indexer, Vertices};

        info!("Loading {}", path_str);
        let obj = obj::load::<Polygon<obj::IndexTuple>>(Path::new(path_str)).unwrap();

        let hub_ptr = self.hub.clone();
        let mut hub = hub_ptr.lock().unwrap();
        let mut groups = HashMap::new();
        let mut meshes = Vec::new();
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for object in obj.object_iter() {
            let mut group = Group::new(hub.spawn_empty());
            for gr in object.group_iter() {
                let (mut num_normals, mut num_uvs) = (0, 0);
                {   // separate scope for LruIndexer
                    let f2i = |x: f32| I8Norm(cmp::min(cmp::max((x * 127.) as isize, -128), 127) as i8);
                    vertices.clear();
                    let mut lru = LruIndexer::new(10, |_, (ipos, iuv, inor)| {
                        let p: [f32; 3] = obj.position()[ipos];
                        vertices.push(Vertex {
                            pos: [p[0], p[1], p[2], 1.0],
                            uv: match iuv {
                                Some(i) => {
                                    num_uvs += 1;
                                    obj.texture()[i]
                                },
                                None => [0.0, 0.0],
                            },
                            normal: match inor {
                                Some(id) => {
                                    num_normals += 1;
                                    let n: [f32; 3] = obj.normal()[id];
                                    [f2i(n[0]), f2i(n[1]), f2i(n[2]), I8Norm(0)]
                                },
                                None => [I8Norm(0), I8Norm(0), I8Norm(0x7f), I8Norm(0)],
                            },
                        });
                    });

                    indices.clear();
                    indices.extend(gr.indices.iter().cloned()
                                             .triangulate()
                                             .vertices()
                                             .map(|tuple| lru.index(tuple) as u16));
                };

                info!("\tmaterial {} with {} normals and {} uvs", gr.name, num_normals, num_uvs);
                let material = match gr.material {
                    Some(ref rc_mat) => self.load_obj_material(&*rc_mat, num_normals!=0, num_uvs!=0),
                    None => Material::MeshBasic { color: 0xffffff, map: None, wireframe: true },
                };
                info!("\t{:?}", material);

                let (vbuf, slice) = self.backend.create_vertex_buffer_with_slice(&vertices, &indices[..]);
                let cbuf = self.backend.create_constant_buffer(1);
                let mesh = Mesh {
                    object: hub.spawn_visual(material, GpuData {
                        slice,
                        vertices: vbuf,
                        constants: cbuf,
                        pending: None,
                    }),
                };
                group.add(&mesh);
                meshes.push(mesh);
            }

            groups.insert(object.name.clone(), group);
        }

        (groups, meshes)
    }

    /// Update the geometry of `DynamicMesh`.
    pub fn mix(&mut self, mesh: &DynamicMesh, shapes: &[(&str, f32)]) {
        let f2i = |x: f32| I8Norm(cmp::min(cmp::max((x * 127.) as isize, -128), 127) as i8);

        self.hub.lock().unwrap().update_mesh(mesh);
        let shapes: Vec<_> = shapes.iter().map(|&(name, k)|
            (&mesh.geometry.shapes[name], k)
        ).collect();
        let mut mapping = self.backend.write_mapping(&mesh.dynamic.buffer).unwrap();

        for i in 0 .. mesh.geometry.base_shape.vertices.len() {
            let (mut pos, ksum) = shapes.iter().fold((Vector3::new(0.0, 0.0, 0.0), 0.0), |(pos, ksum), &(ref shape, k)| {
                let p: [f32; 3] = shape.vertices[i].into();
                (pos + k * Vector3::from(p), ksum + k)
            });
            if ksum != 1.0 {
                let p: [f32; 3] = mesh.geometry.base_shape.vertices[i].into();
                pos += (1.0 - ksum) * Vector3::from(p);
            }
            let normal = if mesh.geometry.base_shape.normals.is_empty() {
                NORMAL_Z
            } else {
                let n = mesh.geometry.base_shape.normals[i];
                [f2i(n.x), f2i(n.y), f2i(n.z), I8Norm(0)]
            };
            mapping[i] = Vertex {
                pos: [pos.x, pos.y, pos.z, 1.0],
                uv: [0.0, 0.0], //TODO
                normal,
            };
        }
    }
}
