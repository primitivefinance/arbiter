struct SimulationEnvironment<'a> {
    evm: EVM<CacheDB<EmptyDB>>,
    event_buffer: Arc<RwLock<Vec<Log>>>, // TODO: This should probably just store head
    writer_thread: Option<thread::JoinHandle<()>>,
    agents: HashMap<&'a str, Box<dyn Agent>>,
}

struct SimulationContract {}

impl SimulationEnvironment {
    fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.database(db);

        Self {
            evm,
            event_buffer: Arc::new(RwLock::new(Vec::<Log>::new())),
            writer_thread: Some(thread::spawn(|| {})),
            agents: HashMap::new(),
        }        
    }

    fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;

        let execution_result = match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        };

        self.echo_logs(execution_result.logs());

        execution_result
    }
    
    fn echo_logs(&mut self, logs: Vec<Log>) {
        if let Some(handle) = self.writer_thread.take() {
            handle.join().unwrap();
        }

        self.event_buffer.write().unwrap().clear();

        logs.into_iter().for_each(|log| {
            self.event_buffer.write().unwrap().push(log);
        });
    }
}