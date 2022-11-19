const INVALID: i32 = -1;

enum State {
    END,
    BLOCK,
    RUNNING,
    READY,
    INVALID,
}

pub struct pcb_tuple {
    id: i32,
    priority: i32,
    cpu_time: i32,
    all_time: i32,
    start_block: i32,
    block_time: i32,
    state: State,
}

impl pcb_tuple {
    fn new(
        id: i32,
        priority: i32,
        cpu_time: i32,
        all_time: i32,
        start_block: i32,
        block_time: i32,
        state: State,
    ) -> Self {
        pcb_tuple {
            id,
            priority,
            cpu_time,
            all_time,
            start_block,
            block_time,
            state,
        }
    }

    fn new() -> Self {
        pcb_tuple {
            id: INVALID,
            priority: INVALID,
            cpu_time: INVALID,
            all_time: INVALID,
            start_block: INVALID,
            block_time: INVALID,
            state: State::INVALID,
        }
    }
}


pub struct ProcessManager {
    //PCB数组
    std::Vector<pcb_tuple> pcb_list_;
    //map，用于映射id和pcb

    //阻塞队列

}

impl ProcessManager {
    
}