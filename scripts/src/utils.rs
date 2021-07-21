use gdnative::prelude::*;

#[inline]
pub fn normalized(vector_to_normalize: Vector2) -> Vector2 {
    let option = Vector2::try_normalize(vector_to_normalize);
    match option {
        None => Vector2::zero(),
        Some(vector2) => vector2,
    }
}

#[inline]
// Scene loading helper function
pub fn load_scene(path: &str) -> Option<Ref<PackedScene, Shared>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;
    let scene = unsafe { scene.assume_unique().into_shared() };
    scene.cast::<PackedScene>()
}
