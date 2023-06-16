use clap::{Parser, Subcommand};
use virt::connect::Connect;
use virt::domain::Domain;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
#[group(multiple = false)]
struct InitVMCli {
    #[command(subcommand)]
    command: InitVMCommands,
}

#[derive(Subcommand)]
enum InitVMCommands {
    /// Triggers a complete build of the elbe XML file
    #[command(author, version, about)]
    Create { 
        #[arg(long, short)]
        file: Option<PathBuf>,
    },
    /// Attach to the initvm console which is accessed via virsh
    #[command(author, version, about)]
    Attach {
        // TODO
    },
    /// Make sure an initvm is running in the background    
    #[command(author, version, about)]
    Ensure {
        // TODO
    },
    /// Start initvm in the background
    #[command(author, version, about)]
    Start {
        // TODO
    },
    /// Shutdown the running initvm
    #[command(author, version, about)]
    Stop {
        // TODO
    },
    /// Triggers a complete rebuild of an existing elbe XML file  
    #[command(author, version, about)]
    Submit {
        // TODO
    },
    /// Upload elbe version from the current working into initvm using rsync
    #[command(author, version, about)]
    Sync  {
        // TODO
    },
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
    let cli = InitVMCli::parse();
    let mut action = InitVMAction::new();

    match &cli.command {
        InitVMCommands::Create{ file } => {
            initvm_create(action.domain, file);
        }
        InitVMCommands::Attach{ } => {

        }
        InitVMCommands::Ensure{ } => {

        }
        InitVMCommands::Start{ } => {
            initvm_start();
        }
        InitVMCommands::Stop{ } => {

        }
        InitVMCommands::Submit{ } => {

        }
        InitVMCommands::Sync{ } => {

        }

    }
}

fn initvm_create(domain: Domain, file: &Option<PathBuf>) {
    if domain.get_name().unwrap() == "initvm" {
        println!("Domain `{}` already found!", domain.get_name().unwrap());
        println!("If you want to remove your old initvm from libvirt \
                  run `virsh --connect qemu:///system undefine {}`", 
                  domain.get_name().unwrap());
    }
    if file.is_none() { 
        // Use default elbe-init-with-ssh
    } else {
        // Use given xml file here     
        println!("{:?}", file);
    }
}

fn initvm_start() {
    println!("start-function");
}