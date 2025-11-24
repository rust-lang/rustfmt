fn foo() {
    unprivileged_content.start_all::<script_layout_interface::message::Msg, layout_thread::LayoutThread, script::script_thread::ScriptThread>(true);
}
