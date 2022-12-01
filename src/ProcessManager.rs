use std::collections::*;


const INVALID: i32 = -1;

enum State {
    END,
    BLOCK,
    RUNNING,
    READY,
    INVALID,
}

pub struct ProcessManager {
    //PCB数组
    pcb_list_: Vec<pcb_tuple>,
    //map，用于映射id和pcb
    pcb_map_: HashMap<i32, pcb_tuple>,
    //阻塞队列
    block_queue_:LinkedList<pcb_tuple>,
    //ready队列,底层是红黑树，在插入和删除的时候动态排序
    ready_queue_:BTreeSet<pcb_tuple>,
    //end队列
    end_queue:LinkedList<pcb_tuple>,
    //时间记录器
    count_:i32;
    //当前正在执行的任务
    running_prog_:pcb_tuple,
}

impl ProcessManager {
    
    fn new(pcb_list: Vec<pcb_tuple>) -> ProcessManager {
        ProcessManager {
            pcb_list_,

            
        }
    }
}