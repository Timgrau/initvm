use clap::Parser;
use virt::connect::Connect;
use virt::domain::Domain;

#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help = true)]
#[group(multiple = false)]
struct InitVMArgs {
    /// Attach to the initvm console which is accessed via virsh
    #[arg(long)]
    attach: bool,
    /// Triggers a complete build of the elbe XML file
    #[arg(long)]
    create: bool,
    /// Make sure an initvm is running in the background    
    #[arg(long)]
    ensure: bool,
    /// Start initvm in the background
    #[arg(long)]
    start: bool,
    /// Shutdown the running initvm
    #[arg(long)]
    stop: bool, 
    /// Triggers a complete rebuild of an existing elbe XML file  
    #[arg(long)]
    submit: bool, 
    /// Upload elbe version from the current working into initvm using rsync
    #[arg(long)]
    sync : bool,
}

struct InitVMAction {
    conn: Connect,
    domain: Domain,
    // TODO: node
}

impl InitVMAction {
    fn new(/* node, initvmneeded */) -> Self {
        // TODO: Check if soap is local
        let mut conn = match Connect::open("qemu:///system") {
            Ok(conn) => conn,
            Err(e) => panic!("No conenction to hypervisor: {}", e),
        };
        let d = match conn.list_all_domains(1 | 2) {
            Ok(d) => d,
            Err(e) => panic!("Error {}", e),
        };
            
        println!("{:?}", d[0].as_ptr().is_null());
        
        // TODO: Normally error messages and reconnection here 
        let domain = match Domain::lookup_by_name(&conn, "initvm") {
            Ok(domain) => domain,
            Err(e) => {
                let _ = conn.close();
                panic!("Domain cannot be found {}", e)
            },
        };

        Self { conn, domain /*, node */ }
    }
}

// TODO: `create` and `recreate` could be redundant, lets see...
fn main() {
    let args = InitVMArgs::parse();
    let mut action = InitVMAction::new();

    println!("{:?}", action.conn);
    assert_eq!(Ok(0), action.conn.close());
    //println!("{:?}", action.domain);


    if args.create {
        initvm_create(action.domain);
    }

    if args.start {
        initvm_start();
    }
}

fn initvm_create(domain: Domain) {
    if !domain.as_ptr().is_null() {
        print!("Domain {:?}:{:?} already found!", domain.get_hostname(1 | 2), domain.get_id());
    }

    println!("create-function")
}

fn initvm_start() {
    println!("start-function")
}