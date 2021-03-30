use anyhow::Result;
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, UNLIT_FRAG, UNLIT_VERT, Matrix4
};

struct MyApp {
    material: Material,
    mesh: Mesh,
    time: f32,
}

impl App for MyApp {
    const NAME: &'static str = "MyApp";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;

        let (vertices, indices) = a_paper(50, [0.0, 0.3, 1.0]);
        let mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            mesh,
            material,
            time: 0.0,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        let transform = Matrix4::new_scaling(0.001);
        //let transform = Matrix4::from_euler_angles(0.0, self.time, 0.0);
        let object = Object {
            material: self.material,
            mesh: self.mesh,
            transform,
        };
        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects: vec![object],
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
}

#[allow(unused)]
fn rainbow_cube() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        Vertex::new([-1.0, -1.0, -1.0], [0.0, 1.0, 1.0]),
        Vertex::new([1.0, -1.0, -1.0], [1.0, 0.0, 1.0]),
        Vertex::new([1.0, 1.0, -1.0], [1.0, 1.0, 0.0]),
        Vertex::new([-1.0, 1.0, -1.0], [0.0, 1.0, 1.0]),
        Vertex::new([-1.0, -1.0, 1.0], [1.0, 0.0, 1.0]),
        Vertex::new([1.0, -1.0, 1.0], [1.0, 1.0, 0.0]),
        Vertex::new([1.0, 1.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([-1.0, 1.0, 1.0], [1.0, 0.0, 1.0]),
    ];

    let indices = vec![
        3, 1, 0, 2, 1, 3, 2, 5, 1, 6, 5, 2, 6, 4, 5, 7, 4, 6, 7, 0, 4, 3, 0, 7, 7, 2, 3, 6, 2, 7,
        0, 5, 4, 1, 5, 0,
    ];

    (vertices, indices)
}

fn a_paper(depth: usize, color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::with_capacity(depth * 2);
    let mut horiz = 1.;
    let mut vert = 1. / std::f32::consts::SQRT_2;
    let mut x = 0.;//-horiz / 3.;
    let mut y = 0.;//-vert / 3.;
    let mut add_vert = |x, y| vertices.push(Vertex::new([x, 0., y], color));
    let mut cycle = false;
    for _ in 0..depth {
        if cycle {
            y += vert;
            horiz *= -2.;
            add_vert(x + horiz, y);
            add_vert(x - horiz, y);
        } else {
            x += horiz;
            vert *= -2.;
            add_vert(x, y + vert);
            add_vert(x, y - vert);
        }
        cycle = !cycle;
    }

    let indices = (0..vertices.len() as u16).collect();
    (vertices, indices)
}
