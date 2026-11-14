trait SpinGuardian {}

struct RwLockUpgradeableGuard<'a, T, G: SpinGuardian>(&'a T, G);

impl<T: /*?Sized*/, G: SpinGuardian> RwLockUpgradeableGuard<'_, T, G> {}
