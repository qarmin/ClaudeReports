use cargo_metadata::WorkspaceDefaultMembers;

fn main() {
    let workspace_default_members = WorkspaceDefaultMembers::default();
    let _ = workspace_default_members.get(0usize);
}
