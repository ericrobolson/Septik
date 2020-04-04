#[macro_export]
macro_rules! init_components{
    ($( $t:ty ),* ) => {
        /// Link the given components to the world
        pub fn register(world: &mut World){
                    $(
                      world.register::<$t>();
                    )*
            }

            $(
            impl Component for $t {
                type Storage = VecStorage<Self>;
            }
            )*
        };
}
