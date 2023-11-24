use shrs::ShellBuilder;
use shrs_insulter::InsulterPlugin;
use shrs_mux::MuxPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(InsulterPlugin::new(vec![], 1., true))
        .with_plugin(MuxPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
