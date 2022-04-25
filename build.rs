// build.rs
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

fn main() {
    //Create Folder Stucture
    let mut mkdir = Command::new("mkdir");
    mkdir.arg("target/public");
    let _com = mkdir.output().expect("failed to execute process");

    let mut mkdir = Command::new("mkdir");
    mkdir.arg("target/public/app.app");
    let _com = mkdir.output().expect("failed to execute process");

    let mut mkdir = Command::new("mkdir");
    mkdir.arg("target/public/app.app/Contents");
    let _com = mkdir.output().expect("failed to execute process");

    let mut mkdir = Command::new("mkdir");
    mkdir.arg("target/public/app.app/Contents/Resources");
    let _com = mkdir.output().expect("failed to execute process");

    let mut mkdir = Command::new("mkdir");
    mkdir.arg("target/public/app.app/Contents/Macos");
    let _com = mkdir.output().expect("failed to execute process");

    //Copy target release to Macos Folder
    let mut cp = Command::new("cp");
    cp.arg("target/debug/sandbox")
        .arg("target/public/app.app/Contents/Macos/");
    let _com = cp.output().expect("failed to execute process");

    //Copy icon to Resource Folder
    let mut cp = Command::new("cp");
    cp.arg("resources/folder2.ico")
        .arg("target/public/app.app/Contents/Resources/");
    let _com = cp.output().expect("failed to execute process");

    //Create Info.Plist
    let plist = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
        <key>CFBundleDisplayName</key>
        <string>sandbox</string>
        <key>CFBundleName</key>
        <string>sandbox</string>
        <key>CFBundleExecutable</key>
        <string>sandbox</string>
        <key>CFBundleIconFile</key>
        <string>folder2.ico</string>
    </dict>
    </plist>";

    let x = write_plist(plist);
    match x {
        Ok(_) => {
            println!("All good today!");
        }
        Err(e) => eprintln!("Error 1023::{}", e),
    }

    // Create .dmg file
    // hdiutil create name.dmg -srcfolder target/public/app.app -ov
    //
    let mut hdiutil = Command::new("hdiutil");
    hdiutil
        .arg("create")
        .arg("name.dmg")
        .arg("-srcfolder target/public/app.app")
        .arg("-ov");
    let com = hdiutil.output().expect("failed to execute process");
    eprintln!("hdiutil output:: {:?}", com);
}

fn write_plist(buff: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("target/public/app.app/Contents/Info.plist")?;
    file.write_all(buff)?;
    Ok(())
}
