use shrs::prelude::ShellBuilder;

use shrs_insulter::plugin::InsulterPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(InsulterPlugin::default())
        .build()
        .unwrap();

    myshell.run();
}
