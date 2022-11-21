use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

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

impl PartialEq for pcb_tuple {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for pcb_tuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Eq for pcb_tuple {}

impl Ord for pcb_tuple {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.priority)
    }
}
