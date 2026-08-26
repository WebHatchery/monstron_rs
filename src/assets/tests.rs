use super::*;

#[test]
fn combat_vfx_uses_the_processed_transparent_atlas() {
    assert_eq!(
        asset_bytes(VFX),
        include_bytes!("../../assets/generated/combat/combat_vfx_atlas_v1.png")
    );
    assert_ne!(
        asset_bytes(VFX),
        include_bytes!("../../assets/generated/combat/combat_vfx_atlas_chroma_v1.png")
    );
}
