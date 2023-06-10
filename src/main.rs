use clap::Parser;
use virt::connect::Connect;

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
    // TODO: Parameter here
    conn: Connect,
    // initvm
    // conn
}

impl InitVMAction {
    fn new(/* node, initvmneeded */) -> Self {
        // TODO: Check if soap is local
        let conn = match Connect::open("qemu:///system") {
            Ok(conn) => conn,
            Err(e) => panic!("No conenction to hypervisor: {}", e),
        };

        println!("{:?}", conn); 
        Self { conn, /* initvm, conn */ }
    }
}

// TODO: `create` and `recreate` could be redundant, lets see...
fn main() {
    let args = InitVMArgs::parse();
    let mut action = InitVMAction::new();

    println!("{:?}", action.conn);
    assert_eq!(Ok(0), action.conn.close());


    if args.create {
        initvm_create();
    }

    if args.start {
        println!("start")
    }
}

fn initvm_create() {
    println!("create-function")
}