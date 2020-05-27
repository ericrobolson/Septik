macro_rules! enum_with_type_conversion {
    (type=$ty:ty; $enum_name:ident [ $( $y:ident ),* ];) => {
        #[derive(Copy, Clone, Debug)]
        pub enum $enum_name {
            $(
                $y,
            )*
        }

        impl $enum_name {
            pub fn to_base_type(&self) -> $ty{
                *self as $ty
            }

            pub fn from_base_type(value: $ty) -> Self{
                unimplemented!()
            }
        }
    };
}

pub type VoxelMaterialType = u8;

enum_with_type_conversion!(
    type = u8;
    VoxelMaterials [
        Dirt,
        Grass,
        Water,
        Rock,
        PoisonGas,
        // Organic
        Muscle,
        Bone,
        Skin,
        Brain,
        Hair,
        Shell
    ];
);
