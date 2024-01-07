use crossterm::style::{Color, ContentStyle, Stylize};
use shrs::{
    anyhow,
    prelude::{styled, AfterCommandCtx, Context, Plugin, Runtime, Shell},
};

use crate::state::InsulterState;

pub struct InsulterPlugin {
    insults: Vec<String>,
    freq: f32,
    include_default: bool,
    style: ContentStyle,
}

impl InsulterPlugin {
    pub fn new(
        insults: Vec<String>,
        freq: f32,
        include_default: bool,
        style: ContentStyle,
    ) -> Self {
        Self {
            insults,
            freq,
            include_default,
            style,
        }
    }
}

impl Plugin for InsulterPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.hooks.insert(insult_hook);
        shell.state.insert(InsulterState::new(
            self.insults.clone(),
            self.freq,
            self.include_default,
            self.style,
        ));
        Ok(())
    }
}
impl Default for InsulterPlugin {
    fn default() -> Self {
        Self::new(vec![], 1., true, ContentStyle::default().with(Color::Red))
    }
}
fn insult_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if !ctx.cmd_output.status.success() {
        if let Some(state) = sh_ctx.state.get_mut::<InsulterState>() {
            if state.should_insult() {
                sh_ctx.out.print_buf(
                    styled!("󱃋 ", state.rand_insult(), " 󱃋\n").apply_styles(state.style),
                )?;
            }
        }
    }

    Ok(())
}
