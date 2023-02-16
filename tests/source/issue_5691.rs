/* Empty Struct With Block Comments */
struct Empty {/* comment */}

struct EmptyWithGeneric<T> {/* comment */}

// Note: `where` with no bounds are removed
struct EmptyWithGenericAndWhereWithNoBounds<T>
where
{/* comment */}

struct EmptyWithGenericAndWhereWithBounds<T>
where T: Clone
{/* comment */}

// Note: `where` with no bounds are removed
struct EmptyWithConstGenericAndWhereWithNoBounds<const C: usize>
where
{/* comment */}

struct EmptyWithConstGenericAndWhereWithBounds<const C: usize>
where [(); { num_slots!(C) }]:,
{/* comment */}


// Empty Structs With Line Comments
struct Empty {// comment
}

struct EmptyWithGeneric<T> {// comment
}

// Note: `where` with no bounds are removed
struct EmptyWithGenericAndWhereWithNoBounds<T>
where
{// comment
}

struct EmptyWithGenericAndWhereWithBounds<T>
where T: Clone
{// comment
}

// Note: `where` with no bounds are removed
struct EmptyWithConstGenericAndWhereWithNoBounds<const C: usize>
where
{// comment
}

struct EmptyWithConstGenericAndWhereWithBounds<const C: usize>
where [(); { num_slots!(C) }]:,
{// comment
}


/* Struct With Fields and Block Comments */
struct Fields {/* comment */ x: usize}

struct FieldsWithGeneric<T> {/* comment */ x: usize}

// Note: `where` with no bounds are removed
struct FieldsWithGenericAndWhereWithNoBounds<T>
where
{/* comment */ x: usize}

struct FieldsWithGenericAndWhereWithBounds<T>
where T: Clone
{/* comment */ x: usize}

// Note: `where` with no bounds are removed
struct FieldsWithConstGenericAndWhereWithNoBounds<const C: usize>
where
{/* comment */ x: usize}

struct FieldsWithConstGenericAndWhereWithBounds<const C: usize>
where [(); { num_slots!(C) }]:,
{/* comment */ x: usize}


// Struct With Fields and Line Comments
struct Fields {// comment
x: usize
}

struct FieldsWithGeneric<T> {// comment
x: usize
}

// Note: `where` with no bounds are removed
struct FieldsWithGenericAndWhereWithNoBounds<T>
where
{// comment
x: usize
}

struct FieldsWithGenericAndWhereWithBounds<T>
where T: Clone
{// comment
x: usize
}

// Note: `where` with no bounds are removed
struct FieldsWithConstGenericAndWhereWithNoBounds<const C: usize>
where
{// comment
x: usize
}

struct FieldsWithConstGenericAndWhereWithBounds<const C: usize>
where [(); { num_slots!(C) }]:,
{// comment
x: usize
}
