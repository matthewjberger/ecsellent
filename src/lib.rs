pub mod prelude {
    pub use crate::{has_component, query, system, world, Entity, Stream};
}

pub type Entity = usize;

pub type Stream<T> = Vec<Option<T>>;

#[macro_export]
macro_rules! world {
    (
        $world_type:ident { $($name:ident[$type:ty]),* $(,)? },
        $world_resources_type:ident { $($resource_name:ident: $resource_type:ty),* $(,)? }
    ) => {
        #[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $world_resources_type {
            $(pub $resource_name: $resource_type,)*
        }

        #[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $world_type {
            pub last_entity: Entity,
            pub resources: $world_resources_type,
            $(pub $name: $crate::Stream<$type>,)*
        }

        pub fn resize_components(world: &mut $world_type) {
            $(world.$name.resize(world.last_entity, None);)*
        }

        pub fn clear_entity(world: &mut $world_type, entity: $crate::Entity) {
            assert!(entity < world.last_entity);
            $(world.$name[entity] = None;)*
        }
    };
}

#[macro_export]
macro_rules! system {
    (
        ($world_type:ident, $world_resources_type:ident) {
            pub fn $name:ident(
                for $entity:ident in $world:ident,
                read [ $($immut_name:ident : $immut_type:ty => $immut_stream:ident),* $(,)? ],
                write [ $($mut_name:ident : $mut_type:ty => $mut_stream:ident),* $(,)? ],
                resources [ $($res_name:ident),* $(,)? ],
                input [ $($input_name:ident : $input_type:ty),* $(,)? ],
                output [ $($output_name:ident = $output_value:expr),* $(,)? ],
            ) $body:block
        }
    ) => {
        pub fn $name(
            $world: &mut $world_type,
        ) {
            $(let mut $output_name = $output_value;)*
            let $world_type {
                last_entity,
                $($immut_stream,)*
                $($mut_stream,)*
                resources: $world_resources_type {
                    $($res_name,)*
                    ..
                },
                ..
            } = $world;
            (0..*last_entity)
                .into_iter()
                .for_each(|$entity| {
                $(
                    let Some(Some($immut_name)) = $immut_stream.get($entity) else {
                        return;
                    };
                )*
                $(
                    let Some(Some($mut_name)) = $mut_stream.get_mut($entity) else {
                        return;
                    };
                )*
                $body
            });
        }
    };
}

#[macro_export]
macro_rules! query {
    (
        ($world_type:ident, $world_resources_type:ident) {
            pub fn $name:ident(
                for $entity:ident in $world:ident,
                read [ $($immut_name:ident : $immut_type:ty => $immut_stream:ident),* $(,)? ],
                write [ $($mut_name:ident : $mut_type:ty => $mut_stream:ident),* $(,)? ],
                resources [ $($res_name:ident),* $(,)? ],
                input [ $($input_name:ident : $input_type:ty),* $(,)? ],
                output [ $output:ident = $initial_value:expr ],
            ) -> $return_type:ty $body:block
        }
    ) => {
        pub fn $name(
            $world: &mut $world_type,
            $($input_name: $input_type),*
        ) -> $return_type {
            let mut $output = $initial_value;
            let $world_type {
                last_entity,
                $($immut_stream,)*
                    resources: $world_resources_type {
                    $($res_name,)*
                    ..
                },
                ..
            } = $world;
            (0..*last_entity)
                .into_iter()
                .for_each(|$entity| {
                    $(
                        let Some(Some($immut_name)) = $immut_stream.get($entity) else {
                            return;
                        };
                    )*
                    $(
                        let Some(Some($mut_name)) = $mut_stream.get_mut($entity) else {
                            return;
                        };
                    )*
                    $body
                });
            $output
        }
    };
}

#[macro_export]
macro_rules! has_component {
    ($world:expr, $component:ident, $entity:expr) => {
        matches!($world.$component.get($entity), Some(Some(_)))
    };
}
