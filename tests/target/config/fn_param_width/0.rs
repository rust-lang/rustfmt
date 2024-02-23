// rustfmt-config: fn_param_width/0.toml

fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

fn ret(
    mut commands: Commands,
    icons: Res<Icons>,
) -> bool {
    // block
}

unsafe fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

fn lagging(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub fn public(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub(super) fn up(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub(crate) fn all(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub unsafe fn complicated(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

extern "C" {
    fn foreign(
        mut commands: Commands,
        icons: Res<Icons>,
    ) {
        // block
    }
}

fn homura<T: Deref<Target = i32>>(_: T) {}

fn generic<C>(
    query: Query<C>,
) {
    // block
}

pub fn setup_arena(
    mut commands: Commands,
    icons: Res<Icons>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //models: Res<Models>,
) {
    // block
}

fn comments(
    mut commands: Commands,
    /* intermission */ icons: Res<Icons>,
) {
    // block
}

fn spacing(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

impl Trait {
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
    );
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
    );
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    );

    fn lorem(
        mut commands: Commands,
    ) {
        // block
    }
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
    ) {
        // block
    }
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        // block
    }
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // block
    }
}

fn outer(
    mut commands: Commands,
    icons: Res<Icons>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // block

    fn inner(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        // block
    }
}

mod example {
    fn mod_func(
        mut commands: Commands,
        icons: Res<Icons>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        // block
        fn nested(
            mut commands: Commands,
            icons: Res<Icons>,
            mut meshes: ResMut<Assets<Mesh>>,
        ) {
            // block
        }
    }
}
