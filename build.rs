use std::process::Command;
use std::env;

#[cfg(not(target_os = "windows"))]
const GIT: &str = "https://github.com/keystone-engine/keystone.git";
#[cfg(target_os = "windows")]
const ZIP: &str = "https://github.com/keystone-engine/keystone/releases/download/0.9.1/keystone-0.9.1-win64.zip";

fn check_cmd(cmd: &str) -> bool {
    match Command::new(cmd).spawn() {
        Ok(_) => true,
        Err(_) => false, 
    }
}

fn check_sudo() {
    let is_sudo = match env::var("USER") {
        Ok(val) => val == "root",
        Err(_) => false,
    };
    if !is_sudo { panic!("You have to run this as root!"); }
}

fn check_dependencies() {
    if !check_cmd("brew") { panic!("Homebrew not found!"); }
    if !check_cmd("cmake") { 
        Command::new("brew")
            .args(&["install", "cmake"])
            .output()
            .expect("failed to execute process");
    }
    if !check_cmd("git") { 
        Command::new("brew")
            .args(&["install", "git"])
            .output()
            .expect("failed to execute process");
    }
}

#[cfg(target_os = "windows")]
fn download_kstool() {
    let mut output = Command::new("powershell")
            .args(&["-Command", &format!("Invoke-WebRequest {} -OutFile keystone.zip", ZIP)])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("powershell")
            .args(&["-Command", "Expand-Archive -Path keystone.zip -DestinationPath ."])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    let path = env::home_dir().unwrap();
    let cargo_bin = &format!("{}\\.cargo\\bin\\", path.display()); // just install it to .cargo/bin for now.
    output = Command::new("powershell")
            .args(&["xcopy", ".\\keystone-0.9.1-win64\\kstool.exe", cargo_bin])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("powershell")
            .args(&["rm", "-r", ".\\keystone-0.9.1-win64\\"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("powershell")
            .args(&["rm", ".\\keystone.zip"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(not(target_os = "windows"))]
fn download_kstool() {
    let mut output = Command::new("git")
            .args(&["clone", GIT])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("mkdir")
            .current_dir("keystone/")
            .args(&["build"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("sh")
            .current_dir("keystone/build/")
            .args(&["../make-share.sh"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("make")
            .current_dir("keystone/build/")
            .args(&["install"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    output = Command::new("rm")
            .args(&["-r", "-f", "keystone"])
            .output()
            .expect("failed to execute process");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    if !check_cmd("kstool") {
        if cfg!(target_os = "linux") { check_sudo(); }
        if cfg!(target_os = "macos") { check_dependencies(); }
        download_kstool();
    }
}