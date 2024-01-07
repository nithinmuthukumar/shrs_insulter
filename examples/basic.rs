use shrs::ShellBuilder;
use shrs_insulter::plugin::InsulterPlugin;
use shrs_mux::MuxPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(InsulterPlugin::default())
        .with_plugin(MuxPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
