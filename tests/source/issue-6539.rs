pub trait Manager {
    fn attach_device(&self, seat_id: &str, sysfs_path: &str, interactive: bool) -> zbus::Result<()>;
}
