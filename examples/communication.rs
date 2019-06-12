use mpi::topology::Communicator;
use mpi_traffic::communication::bincode_all_gather_varcount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct A {
    pub v: Vec<i32>,
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    let local_data = A {
        v: vec![rank, rank + 1],
    };
    let data = bincode_all_gather_varcount(world, &local_data);
    println!("{:?}", data);
}
