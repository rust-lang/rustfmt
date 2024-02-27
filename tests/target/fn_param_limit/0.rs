// rustfmt-enable_fn_param_limit: true
// rustfmt-fn_param_limit: 0
// rustfmt-edition: 2018

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

fn comments(
    mut commands: Commands,
    /**/ icons: Res<Icons>,
) {
    // block
}

fn comments(
    mut commands: Commands,
    /* really loooooong intermission */ icons: Res<Icons>,
) {
    // block
}

fn generic<C>(
    query: Query<C>,
    query2: Query<C>,
) {
    // block
}

fn lorem<C>(
    query: Query<C>,
) where
    C: Add + Sub + Mul + Div,
{
    // body
}

fn lorem<C>(
    query: Query<C>,
    mut commands: Commands,
    icons: Res<Icons>,
) where
    C: Add + Sub + Mul + Div,
{
    // body
}

fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) -> bool {
    // block
}

pub fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub(crate) fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

pub(super) fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

async fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

unsafe fn lorem(
    mut commands: Commands,
    icons: Res<Icons>,
) {
    // block
}

extern "C" {
    fn lorem(
        mut commands: Commands,
        icons: Res<Icons>,
    ) {
        // block
    }
}

impl Trait {
    fn lorem(
        mut commands: Commands,
    );
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
