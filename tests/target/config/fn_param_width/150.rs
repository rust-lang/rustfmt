// rustfmt-config: fn_param_width/150.toml

fn lorem(mut commands: Commands) {
    // block
}
fn lorem(mut commands: Commands, icons: Res<Icons>) {
    // block
}
fn lorem(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>) {
    // block
}
fn lorem(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // block
}

fn comments(mut commands: Commands, /**/ icons: Res<Icons>) {
    // block
}

fn comments(mut commands: Commands, /* really loooooong intermission */ icons: Res<Icons>) {
    // block
}

impl Trait {
    fn lorem(mut commands: Commands);
    fn lorem(mut commands: Commands, icons: Res<Icons>);
    fn lorem(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>);
    fn lorem(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>);
}

fn outer(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>) {
    // block

    fn inner(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>) {
        // block
    }
}

mod example {
    fn mod_func(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>) {
        // block
        fn nested(mut commands: Commands, icons: Res<Icons>, mut meshes: ResMut<Assets<Mesh>>) {
            // block
        }
    }
}
