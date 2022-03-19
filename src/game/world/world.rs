use std::{cell::RefCell, collections::{HashMap, HashSet}, sync::mpsc::{channel, Receiver, Sender, TryRecvError}, thread};

use engine::{input::InputState, noise::Noise};

use crate::game::entity::player::Player;

use super::{
    block::block::{Block, BlockCoordinate},
    chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE},
};

const CHUNK_LOAD_RADIUS: i32 = 16;

pub struct World {
    pub chunks: HashMap<ChunkCoordinate, RefCell<Chunk>>,
    player: Player,
    chunk_receiver: Receiver<Chunk>,
    chunk_coordinate_sender: Sender<ChunkCoordinate>,
    chunk_generation_in_progress: HashSet<ChunkCoordinate>,
}

impl World {
    pub fn new() -> Self {
        let (chunk_sender, chunk_receiver) = channel::<Chunk>();
        let (chunk_coordinate_sender, chunk_coordinate_receiver) = channel::<ChunkCoordinate>();

        let noise = Noise::new(16, 1.8, 1.0 / 32.0);

        thread::spawn(move || {
            loop {
                let chunk_coordinate = chunk_coordinate_receiver.recv().unwrap();
                let chunk = Chunk::new(chunk_coordinate, &noise);
                chunk_sender.send(chunk).unwrap();
            }
        });

        World {
            chunks: HashMap::new(),
            player: Player::new(),
            chunk_receiver,
            chunk_coordinate_sender,
            chunk_generation_in_progress: HashSet::new(),
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        self.player.tick(input_state);

        let x0 = (self.player.get_position().x / (CHUNK_SIZE as f32)) as i32;
        let z0 = (self.player.get_position().z / (CHUNK_SIZE as f32)) as i32;

        for x in (x0 - CHUNK_LOAD_RADIUS)..(x0 + CHUNK_LOAD_RADIUS) {
            for z in (z0 - CHUNK_LOAD_RADIUS)..(z0 + CHUNK_LOAD_RADIUS) {
                let chunk_coordinate = ChunkCoordinate{ x, z };
                if !self.chunks.contains_key(&chunk_coordinate) && !self.chunk_generation_in_progress.contains(&chunk_coordinate) {
                    self.chunk_coordinate_sender.send(chunk_coordinate).unwrap();
                    self.chunk_generation_in_progress.insert(chunk_coordinate);
                }
            }
        }

        loop {
            match self.chunk_receiver.try_recv() {
                Ok(chunk) => {
                    self.chunk_generation_in_progress.remove(&chunk.position);
                    self.chunks.insert(chunk.position, RefCell::new(chunk));
                },
                Err(err) => match err {
                    TryRecvError::Empty => break,
                    TryRecvError::Disconnected => panic!("something's gone wrong..."),
                },
            }
        }
    }

    pub fn get_block(&self, block_coordinate: BlockCoordinate) -> Block {
        let chunk_coordinate = block_coordinate.to_chunk_coordinate();

        match self.chunks.get(&chunk_coordinate) {
            Some(chunk) => {
                let chunk_block_coordinate = block_coordinate.to_chunk_block_coordinate();
                chunk.borrow().get_block(chunk_block_coordinate)
            }
            None => Block::Air,
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
}
