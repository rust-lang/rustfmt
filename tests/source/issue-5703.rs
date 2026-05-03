struct PubCrate(
    pub(crate) std::collections::HashMap<some::module::path::TypeNameA, some::module::path::TypeNameB>,
);

struct PubSuper(
    pub(super) std::collections::HashMap<some::module::path::TypeNameA, some::module::path::TypeNameB>,
);

struct PubInPath(
    pub(in some::module) std::collections::HashMap<some::module::path::TypeNameA, some::module::path::TypeNameB>,
);

struct PubPlain(
    pub std::collections::HashMap<some::module::path::TypeNameAaaaaaaaaaa, some::module::path::TypeNameB>,
);

struct NamedPubCrate {
    pub(crate) field: std::collections::HashMap<some::module::path::TypeNameA, some::module::path::TypeNameB>,
}

struct ShortPubCrate(pub(crate) u32);
