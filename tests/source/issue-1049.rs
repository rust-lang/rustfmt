// Test issue-1049
pub unsafe fn reborrow_mut(&mut X: Abcde) -> Handle<NodeRef<marker::Mut, K, V, NodeType>, HandleType> {
}

pub fn merge(mut X: Abcdef) -> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
}

impl Handle {
    pub fn merge(a: Abcd) -> Handle<NodeRef<marker::Mut<'a>, K, V, marker::Internal>, marker::Edge> {
    }
}
