use ecsellent::prelude::*;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Spawned;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Position(pub f32, pub f32);

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Parent(pub Entity);

world! {
    World {
        spawned[Spawned],
        parents[Parent],
        positions[Position],
    },
    Resources {
        delta_time: f32,
    }
}

system!(
    pub fn example_system<World, Resources>(
        for entity in world,
        read [ _spawned: Spawned => spawned ],
        write [ ],
        resources [ ],
        input [ ],
        output [ entities = () ],
    ) -> () {
        println!("Hello, world! Entity {entity} is spawned!");
    }
);

system!(
    pub fn query_spawned_entities<World, Resources>(
        for entity in world,
        read [ _spawned: Spawned => spawned],
        write [ ],
        resources [ ],
        input [ ],
        output [ entities = Vec::new() ],
    ) -> Vec<Entity> {
        entities.push(entity);
    }
);

system!(
    pub fn query_despawned_entities<World, Resources>(
        for entity in world,
        read [ ],
        write [ ],
        resources [ ],
        input [ ],
        output [ entities = Vec::new() ],
    ) -> Vec<Entity> {
        if !has_component!(world, spawned, entity) {
            entities.push(entity);
        }
    }
);

pub fn main() {
    let mut world = World::default();
    dbg!(query_spawned_entities(&mut world));
    let entities = spawn_entities_command(&mut world, 3);
    example_system(&mut world);
    dbg!(query_spawned_entities(&mut world));
    despawn_entities_command(&mut world, &entities);
    dbg!(query_spawned_entities(&mut world));
}

pub fn spawn_entities_command(world: &mut World, count: usize) -> Vec<Entity> {
    world.last_entity += count;
    resize_components(world);
    (0..count)
        .map(|_| {
            if let Some(entity) = query_despawned_entities(world).first() {
                clear_entity(world, *entity);
                world.spawned[*entity] = Some(Spawned);
                return *entity;
            }
            let entity = world.spawned.len();
            world.spawned.push(Some(Spawned));
            entity
        })
        .collect()
}

pub fn despawn_entities_command(world: &mut World, entities: &[Entity]) {
    entities.iter().for_each(|entity| {
        if let Some(spawned) = world.spawned.get_mut(*entity) {
            *spawned = None;
        }
        query_descendants(world, *entity)
            .into_iter()
            .for_each(|descendant| {
                if let Some(spawned) = world.spawned.get_mut(*entity) {
                    *spawned = None;
                }
                world.spawned[descendant] = None;
            });
    });
}

pub fn query_descendants(world: &mut World, parent: Entity) -> Vec<Entity> {
    let mut entities = Vec::new();
    query_spawned_entities(world).iter().for_each(|entity| {
        let World {
            resources: Resources { .. },
            ..
        } = world;
        let Some(Some(Parent(parent_entity))) = world.parents.get(*entity) else {
            return;
        };
        if *parent_entity != parent {
            return;
        }
        entities.push(*entity);
        entities.extend(&query_descendants(world, *entity));
    });
    entities
}
