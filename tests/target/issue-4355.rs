impl Drop for LockGuard {
    fn drop(&mut self) {
        LockMap::unlock(&self.0.0, &self.0.1);
    }
}

fn main() {
    let _ = ((1,),).0.0;

    let t1 = (1u8, 2u8);
    let mut t2 = (t1, 3u8);
    t2.1 = t2.0.1;
}
