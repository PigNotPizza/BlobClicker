use druid::{
    widget::{Button, Flex, Label, Widget},
    AppLauncher, Data, Lens, WindowDesc, Color, Env, EventCtx,
    FontDescriptor,
    FontFamily,
    FontWeight,
};
use rand::Rng;


#[derive(Clone, Data, Lens)]
struct BlobData {
    num: i32,
    upgrade_num: i32,
    press_num: i32,
    popup_open: bool,
    pet_multiplier: i32,
    popup_message: String,
}

fn pop_up(data: &mut BlobData) {
    if data.num >= data.upgrade_num {
        data.num -= data.upgrade_num;
        data.upgrade_num *= 8;
        data.press_num *= 2;
    }
}

fn increment(data: &mut BlobData) {
    if data.pet_multiplier != 1 {
        data.num += data.press_num * (data.pet_multiplier-1);
    } else {
        data.num += data.press_num;
    }
}

fn buy_small_egg(ctx: &mut EventCtx, data: &mut BlobData, name: &str, price: i32, min: i32, max: i32) {
    if data.num >= price {
        data.num -= price;
        let mut rng = rand::thread_rng();
        let random_num: i32 = rng.gen_range(min..=max);
        data.pet_multiplier += random_num;
        data.popup_open = true;
        data.popup_message = format!("You got a {}! Pet is {}x", name, random_num);

        let popup_window = WindowDesc::new(popup_ui_builder())
            .title(format!("{} Purchased", name))
            .window_size((300.0, 200.0))
            .resizable(false);
        ctx.new_window(popup_window);
    }
}

fn buy_big_egg(ctx: &mut EventCtx, data: &mut BlobData, name: &str, price: i32, min: i32, max: i32) {
    if data.num >= price {
        data.num -= price;
        let mut rng = rand::thread_rng();
        let random_num: i32 = rng.gen_range(min..=max);
        data.pet_multiplier += random_num;
        data.popup_open = true;
        data.popup_message = format!("You got a {}! Pet is {}x", name, random_num);

        let popup_window = WindowDesc::new(popup_ui_builder())
            .title("Pet Purchased")
            .window_size((300.0, 200.0))
            .resizable(false);

        ctx.new_window(popup_window);
    }
}
fn guide_button(ctx: &mut EventCtx, data: &mut BlobData) {
    
    
    data.popup_open = true;
    data.popup_message = format!("Pets\n\n Each pet has a unique multiplier, which increases your click multiplier.\n Each row has a unique theme that it's pets follow!\n\n\nUpgrade \n\n Upgrade increases the click multiplier by 2 and increases the cost of it by 8.");
    let popup_window = WindowDesc::new(popup_ui_builder())
        .title("Guide")
        .window_size((1000.0, 300.0))
        .resizable(false);

    ctx.new_window(popup_window);
    
}

fn ui_builder() -> impl Widget<BlobData> {
    let label = Label::new(|data: &BlobData, _env: &Env| format!("Blobs {}", data.num))
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));

    let increment_button = Button::new("+").on_click(|_ctx, data: &mut BlobData, _env| increment(data));

    let upgrade_button = Button::new(|data: &BlobData, _env: &Env| format!("Upgrade: {} blobs", data.upgrade_num))
        .on_click(|_ctx, data: &mut BlobData, _env| pop_up(data));

    let egg_label = Label::new("Eggs")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(250, 227, 225))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let smallblobcostlabel = Label::new("  $500 Blobs")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let bigblobcostlabel = Label::new("      $2500 Blobs")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let small_blob_button = Button::new("Small blob egg").on_click(|ctx, data: &mut BlobData, _env| buy_small_egg(ctx, data, "Small blob", 500, 1, 2));
    let big_blob_button = Button::new("Big blob egg").on_click(|ctx, data: &mut BlobData, _env| buy_big_egg(ctx, data, "Big blob", 2500, 3, 5));
    let smallbirdcostlabel = Label::new("  $1950 Blobs ")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let smalldogcostlabel = Label::new("      $1650 Blobs ")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let bigcatcostlabel = Label::new("     $3750 Blobs")
        .with_text_size(32.5)
        .with_text_color(Color::rgb8(194, 245, 164))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));
    let small_bird_button = Button::new("Small bird egg").on_click(|ctx, data: &mut BlobData, _env| buy_small_egg(ctx, data, "Small bird", 950, 3, 4));
    let small_dog_button = Button::new("Small dog egg").on_click(|ctx, data: &mut BlobData, _env| buy_small_egg(ctx, data, "Small dog", 1650, 2, 4));
    let big_cat_button = Button::new("Big cat egg").on_click(|ctx, data: &mut BlobData, _env| buy_big_egg(ctx, data, "Big cat", 3750, 5, 8));
    let guide_button = Button::new("Guide").on_click(|ctx, data: &mut BlobData, _env| guide_button(ctx, data));
    

    Flex::column()
        .with_child(label)
        .with_child(Flex::row().with_child(increment_button))
        .with_child(upgrade_button)
        .with_spacer(20.0)
        .with_child(egg_label)
        .with_child(Flex::row().with_child(small_blob_button).with_child(big_blob_button))
        .with_child(Flex::row().with_child(smallblobcostlabel).with_child(bigblobcostlabel))
        .with_child(Flex::row().with_child(small_bird_button).with_child(small_dog_button).with_child(big_cat_button))
        .with_child(Flex::row().with_child(smallbirdcostlabel).with_child(smalldogcostlabel).with_child(bigcatcostlabel))
        .with_spacer(360.0)
        .with_child(guide_button)
}

fn popup_ui_builder() -> impl Widget<BlobData> {
    let popup_message = Label::new(|data: &BlobData, _env: &Env| data.popup_message.clone())
        .with_text_size(18.0)
        .with_text_color(Color::rgb8(255, 255, 255))
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE).with_weight(FontWeight::BOLD));

    Flex::column()
        .with_child(popup_message)
        .with_spacer(20.0)
        .with_child(Button::new("Close").on_click(|ctx, data: &mut BlobData, _| {
            data.popup_open = false;
            ctx.window().close();
        }))
}


fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Blob Clicker")
        .window_size((690.0, 680.0))
        .resizable(false);

    let launcher = AppLauncher::with_window(main_window)
        .log_to_console();

    let initial_data = BlobData { num: 1, upgrade_num: 4, press_num: 1, popup_open: false, pet_multiplier: 1, popup_message: String::new() };

    launcher.launch(initial_data).unwrap();
}