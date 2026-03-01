use std::{collections::HashMap, fs::{File, OpenOptions, create_dir_all}, hash::{DefaultHasher, Hash, Hasher}, io::{BufReader, Error, Read, Result, Seek, SeekFrom, Write}, sync::{Arc, Mutex}};
use serde::{Serialize, Deserialize};

const STORAGE_FOLDER : &'static str = "data/db/";

#[allow(dead_code)]
enum DbType {
    Player,
    Inventory
}

pub struct DbManager {
    #[allow(dead_code)]
    instances: HashMap<DbType, Arc<Mutex<OxideDb>>>,
}

impl DbManager {
    pub fn new() -> Self {
        Self { instances: HashMap::new() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DbHeader {
    data_length: u16
}

#[derive(Copy, Clone, Debug)]
pub struct DbReferences {
    offset : u64, // For the moment, byte offset,
    length : u16, // Size in bytes
}

pub struct OxideDb {
    memory_index : HashMap<u64, DbReferences>,
    data_handler : File,
    indexer : File
}

impl OxideDb {
    pub fn new(db_name: &str) -> OxideDb {
        if let Err(e) = create_dir_all(format!("{}/", STORAGE_FOLDER)) {
            eprintln!("{}", e);
        }
        
        let data_path = format!("{}/{}.db", STORAGE_FOLDER, db_name);
        let index_path = format!("{}/{}.index", STORAGE_FOLDER, db_name);

        let writer = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&data_path)
            .expect("Impossible d'ouvrir le fichier de données");
        
        let indexer = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&index_path)
            .expect("Impossible d'ouvrir index");

        let mut db = OxideDb {
            memory_index : HashMap::new(),
            data_handler: writer,
            indexer
        };

        match db.load_from_index() {
            Ok(()) => println!("load sucessfull"),
            Err(e) => println!("Load ERROR : {}", e)
        }
        db
    }

    fn load_from_index(&mut self) -> Result<()> {
        self.indexer.seek(SeekFrom::Start(0))?;
        let mut reader = BufReader::new(&self.indexer);
        let mut buffer = [0u8; 18];
        
        while reader.read_exact(&mut buffer).is_ok() {
            let hash = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
            let offset = u64::from_be_bytes(buffer[8..16].try_into().unwrap());
            let length = u16::from_be_bytes(buffer[16..18].try_into().unwrap());

            self.memory_index.insert(hash, DbReferences {
                offset,
                length,
            });
        }
        Ok(())
    }

    fn read(&self, reference : &DbReferences) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.data_handler);
        
        reader.seek(SeekFrom::Start(reference.offset))?;
        let mut buffer = vec![0u8; reference.length as usize];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    fn get_from_string(&mut self, id : String) -> Option<DbReferences> {
        self.memory_index.get(&hash_id(&id)).copied()
    }

    fn read_from_string(&mut self, id : String)  -> Result<Vec<u8>> {
        if let Some(reference) = self.get_from_string(id) {
            return self.read(&reference);
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found"))
    }

    fn write_with_key<T : AsRef<[u8]>>(&mut self, id: &String, value : T) -> Result<()> {
        let hashed_id = hash_id(id);
        let data_bytes = value.as_ref();
        let data_length = data_bytes.len() as u16;

        let header = DbHeader { data_length };
        let header_bytes = bincode::serialize(&header)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        self.data_handler.write_all(&header_bytes)?; // Header
        let offset = self.data_handler.seek(SeekFrom::End(0))?;
        self.data_handler.write_all(data_bytes)?; // Data
        
        self.indexer.write_all(&hashed_id.to_be_bytes())?;
        self.indexer.write_all(&offset.to_be_bytes())?;
        self.indexer.write_all(&data_length.to_be_bytes())?;

        let reference: DbReferences = DbReferences {offset, length: data_length };
        self.memory_index.insert(hashed_id.clone(), reference);

        Ok(())

    }

    pub fn get<T: for<'a> Deserialize<'a>, S:Into<String>>(&mut self, id: S) -> Result<T> {
        // 1. On récupère les octets (ton code actuel)
        let bytes = self.read_from_string(id.into())?;
        
        // 2. On transforme les octets en l'objet voulu
        bincode::deserialize(&bytes)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn put<T: Serialize, S:Into<String>>(&mut self, id: S, value: &T) -> Result<()> {
        let bytes = bincode::serialize(value)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        self.write_with_key(&id.into(), bytes)
    }
}

fn hash_id(id: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}

pub fn create_db<S: Into<String>>(name : S) -> OxideDb {
    let name_string = name.into();
    OxideDb::new(&name_string)
}

// fn get_or_open(name : DbType) {
//     if()
// }