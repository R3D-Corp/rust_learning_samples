use core::hash;
use std::{collections::HashMap, fs::{File, OpenOptions, create_dir_all}, hash::{DefaultHasher, Hash, Hasher}, io::{BufRead, BufReader, Read, Result, Seek, SeekFrom, Write}, sync::{Arc, Mutex, mpsc}, thread, time::Instant};


use serde::{Serialize, Deserialize};
use rand::{self, Rng};

const ID_SIZE : u8 = 30;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456489";

// TO DELETE
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static A: TrackingAllocator = TrackingAllocator;

fn get_current_mem() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

#[derive(Serialize, Deserialize, Debug)]

struct DbHeader {
    data_length: u16
}

#[derive(Copy, Clone, Debug)]
struct DbReferences {
    file_index : u32, // If one file is completed then the db while create an another one.
    offset : u64, // For the moment, byte offset,
    length : u16, // Size in bytes
}

struct OxideDb {
    memory_index : HashMap<u64, DbReferences>,
    active_file : String,
    data_handler : File,
    indexer : File
}

impl OxideDb {
    fn new(active_file: &'static str) -> OxideDb {

        if let Err(e) = create_dir_all(&active_file) {
            eprintln!("{}", e);
        }
        
        let data_path = format!("{}/test_{}.db", active_file, 1);
        let index_path = format!("{}/test_{}.index", active_file, 1);

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
            active_file : data_path,
            data_handler: writer,
            indexer
        };

        match db.load_from_index() {
            Ok(()) => println!("load sucessfull"),
            Err(e) => println!("Load ERROR : {}", e)
        }
        db
    }

    fn generate_id(&self) -> String {
        let mut rng = rand::thread_rng();
        loop {
            let id : String = (0..ID_SIZE).map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }).collect();
    
            if !self.memory_index.contains_key(&hash_id(&id)) {
                return id;
            }
        }
    }
    
    fn get_all(&self) -> HashMap<u64, DbReferences> {
        return self.memory_index.clone();
    }
    
    fn get_from_string(&mut self, id : String) -> Option<DbReferences> {
        self.memory_index.get(&hash_id(&id)).copied()
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
                file_index: 0,
                offset,
                length,
            });
        }
        Ok(())
    }

    fn unload(&mut self) {
        self.memory_index.clear();
    }

    fn write<T : AsRef<[u8]>>(&mut self, value : T) -> Result<String> {
        let data_bytes = value.as_ref();
        let string_id = self.generate_id();
        let hashed_id = hash_id(&string_id);
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

        let reference: DbReferences = DbReferences { file_index: 0, offset, length: data_length };
        self.memory_index.insert(hashed_id.clone(), reference);

        Ok(string_id)
    }

    fn read(&self, reference : &DbReferences) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.data_handler);
        
        reader.seek(SeekFrom::Start(reference.offset))?;
        let mut buffer = vec![0u8; reference.length as usize];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    fn get_storage_stats(&self) -> Result<()> {
        let data_size = self.data_handler.metadata()?.len();
        let index_file_size = self.indexer.metadata()?.len();
        let ram_usage = get_current_mem() as u64; // Utilise ton TrackingAllocator

        println!("\n--- RENTABILITÉ DU STOCKAGE ---");
        println!("Fichier Données (.db) : {:.2} Mo", data_size as f64 / 1_048_576.0);
        println!("Fichier Index (.index) : {:.2} Mo", index_file_size as f64 / 1_048_576.0);
    
        if ram_usage > 0 {
            let ratio = data_size as f64 / ram_usage as f64;
            println!("Ratio Données/RAM      : {:.2}x", ratio);
            
            if ratio > 1.0 {
                println!("Statut : Rentable (L'index est plus léger que les données)");
            } else {
                println!("Statut : Coûteux (L'index pèse plus lourd que les données)");
            }
        }
        Ok(())
    }
}

fn hash_id(id: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}