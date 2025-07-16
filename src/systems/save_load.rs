use std::{
    any::TypeId,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use hecs::{
    Archetype, ColumnBatchBuilder, ColumnBatchType, World,
    serialize::column::{
        DeserializeContext, SerializeContext, deserialize_column, try_serialize, try_serialize_id,
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    WORLD,
    components::{
        camera::Camera,
        draw::Drawable,
        generic::{Position, Size},
        player::Player,
        tiles::{BlocksMovement, BlocksVision},
    },
};

#[allow(dead_code)]
pub fn save_to_file() -> anyhow::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let options = bincode::options();
    let mut serializer = bincode::Serializer::new(&mut buffer, options);
    WORLD.with_borrow_mut(|world| {
        hecs::serialize::column::serialize(
            &world,
            &mut SaveContextSerialize::default(),
            &mut serializer,
        )
        .unwrap();
    });
    let path = Path::new("./save.world");
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
    };
    file.write_all(&buffer)
        .unwrap_or_else(|_| panic!("Failed to write file: ./save.world"));
    println!("Saved world \'{}\' to disk.", path.display());

    Ok(())
}

#[allow(dead_code)]
pub fn load_from_file() -> anyhow::Result<World, Box<bincode::ErrorKind>> {
    // load our world from disk and deserialize it back as world:
    let path = Path::new("./save.world");
    let open = File::open(path).expect("not found!");
    let reader = BufReader::new(open);
    let options = bincode::options();
    let mut deserializer = bincode::Deserializer::with_reader(reader, options);
    println!("Load");
    hecs::serialize::column::deserialize(&mut SaveContextDeserialize::default(), &mut deserializer)
}

// Identifiers for the components we want to include in the serialization process:
#[derive(Serialize, Deserialize)]
enum ComponentId {
    // TODO: Add components that need to be serialised here
    Drawable,
    Position,
    Size,
    Player,
    BlocksMovement,
    BlocksVision,
    Camera,
}

// We need to implement context types for the hecs serialization process:
#[derive(Default)]
struct SaveContextSerialize {}
#[derive(Default)]
struct SaveContextDeserialize {
    components: Vec<ComponentId>,
}

impl DeserializeContext for SaveContextDeserialize {
    fn deserialize_component_ids<'de, A>(&mut self, mut seq: A) -> Result<ColumnBatchType, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.components.clear(); // Discard data from the previous archetype
        let mut batch = ColumnBatchType::new();
        while let Some(id) = seq.next_element()? {
            match id {
                // TODO: Add components that need to be serialised here
                ComponentId::Drawable => {
                    batch.add::<Drawable>();
                }
                ComponentId::Position => {
                    batch.add::<Position>();
                }
                ComponentId::Size => {
                    batch.add::<Size>();
                }
                ComponentId::Player => {
                    batch.add::<Player>();
                }
                ComponentId::BlocksMovement => {
                    batch.add::<BlocksMovement>();
                }
                ComponentId::BlocksVision => {
                    batch.add::<BlocksVision>();
                }
                ComponentId::Camera => {
                    batch.add::<Camera>();
                }
            }
            self.components.push(id);
        }
        Ok(batch)
    }

    fn deserialize_components<'de, A>(
        &mut self,
        entity_count: u32,
        mut seq: A,
        batch: &mut ColumnBatchBuilder,
    ) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // Decode component data in the order that the component IDs appeared
        for component in &self.components {
            match *component {
                // TODO: Add components that need to be serialised here
                ComponentId::Drawable => {
                    deserialize_column::<Drawable, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Position => {
                    deserialize_column::<Position, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Size => {
                    deserialize_column::<Size, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Player => {
                    deserialize_column::<Player, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::BlocksMovement => {
                    deserialize_column::<BlocksMovement, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::BlocksVision => {
                    deserialize_column::<BlocksVision, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Camera => {
                    deserialize_column::<Camera, _>(entity_count, &mut seq, batch)?;
                }
            }
        }
        Ok(())
    }
}

impl SerializeContext for SaveContextSerialize {
    fn component_count(&self, archetype: &Archetype) -> usize {
        archetype
            .component_types()
            // TODO: Add components that need to be serialised here
            .filter(|&t| {
                t == TypeId::of::<Drawable>()
                    || t == TypeId::of::<Position>()
                    || t == TypeId::of::<Size>()
                    || t == TypeId::of::<Player>()
                    || t == TypeId::of::<BlocksMovement>()
                    || t == TypeId::of::<BlocksVision>()
                    || t == TypeId::of::<Camera>()
            })
            .count()
    }

    fn serialize_component_ids<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        // TODO: Add components that need to be serialised here
        try_serialize_id::<Drawable, _, _>(archetype, &ComponentId::Drawable, &mut out)?;
        try_serialize_id::<Position, _, _>(archetype, &ComponentId::Position, &mut out)?;
        try_serialize_id::<Size, _, _>(archetype, &ComponentId::Size, &mut out)?;
        try_serialize_id::<Player, _, _>(archetype, &ComponentId::Player, &mut out)?;
        try_serialize_id::<BlocksMovement, _, _>(
            archetype,
            &ComponentId::BlocksMovement,
            &mut out,
        )?;
        try_serialize_id::<BlocksVision, _, _>(archetype, &ComponentId::BlocksVision, &mut out)?;
        try_serialize_id::<Camera, _, _>(archetype, &ComponentId::Camera, &mut out)?;
        out.end()
    }

    fn serialize_components<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        // TODO: Add components that need to be serialised here
        try_serialize::<Drawable, _>(archetype, &mut out)?;
        try_serialize::<Position, _>(archetype, &mut out)?;
        try_serialize::<Size, _>(archetype, &mut out)?;
        try_serialize::<Player, _>(archetype, &mut out)?;
        try_serialize::<BlocksMovement, _>(archetype, &mut out)?;
        try_serialize::<BlocksVision, _>(archetype, &mut out)?;
        try_serialize::<Camera, _>(archetype, &mut out)?;
        out.end()
    }
}
