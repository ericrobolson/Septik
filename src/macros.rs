#[macro_export]
macro_rules! init_components{
    ($i:ident, ($( $t:ty ),*) ) => {
            pub struct $i {}

            impl ComponentLinker for $i {
                fn register_components(world: &mut World) {
                    $(
                      world.register::<$t>();
                    )*
                }
            }

            $(
            impl Component for $t {
                type Storage = VecStorage<Self>;
            }
            )*
        };
}
