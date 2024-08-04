#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, Data, Env, Lens, Widget, WindowDesc, EventCtx,
};

#[derive(Clone, Data, Lens)]
struct BlobData {
    num: i32,
    upgrade_num: i32,
    press_num: i32,
    popup_open: bool,
}

fn pop_up(ctx: &mut EventCtx, data: &mut BlobData) {
    if data.num >= data.upgrade_num && !data.popup_open {
        data.num -= data.upgrade_num;
        let popup_window = WindowDesc::new(popup_ui_builder(data.upgrade_num))
            .title("Upgraded!")
            .window_size((350.0, 350.0));
        ctx.new_window(popup_window);
        println!("Upgraded {} blobs", data.upgrade_num);
        data.popup_open = true;
        data.upgrade_num *= 2;
        data.press_num *= 2;
    }
}

fn increment(data: &mut BlobData) {
    data.num += data.press_num;
}



fn ui_builder() -> impl Widget<BlobData> {
    let label = Label::new(|data: &BlobData, _: &Env| format!("{} Blobs", data.num));

    let increment_button = Button::new("+").on_click(|_ctx, data: &mut BlobData, _env| increment(data));
    

    let upgrade_button = Button::new(|data: &BlobData, _: &Env| format!("Upgrade: {} blobs", data.upgrade_num))
        .on_click(|ctx, data: &mut BlobData, _env| pop_up(ctx, data));

    Flex::column()
        .with_child(label)
        .with_child(Flex::row().with_child(increment_button))
        .with_child(upgrade_button)
}

fn popup_ui_builder(upgrade_num: i32) -> impl Widget<BlobData> {
    let message = format!("Successfully upgraded! {} Blobs.", upgrade_num);

    Flex::column()
        .with_child(Label::new(message))
        .with_spacer(20.0)
        .with_child(Button::new("Close").on_click(|ctx, data: &mut BlobData, _| {
            data.popup_open = false;
            ctx.window().close();
        }))
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Blob Clicker");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(BlobData { num: 0, upgrade_num: 4, press_num: 1, popup_open: false })
        .unwrap();
}
