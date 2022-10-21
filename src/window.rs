// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data};
// 
// pub fn main_window() -> Result<(), PlatformError> {
//     let main_window = WindowDesc::new(ui_builder);
//     let data = 0_u32;
//     AppLauncher::with_window(main_window)
//         .use_simple_logger()
//         .launch(data)
// }
// 
// fn ui_builder() -> impl Widget<u32> {
//     // The label text will be computed dynamically based on the current locale and count
//     let text =
//         LocalizedString::new("Welcome to a 1 vs 1 Wordle Game").with_arg("count", |data: &u32, _env| (*data).into());
//     let label = Label::new(text).padding(5.0).center();
//     let button = Button::new("increment")
//         .on_click(|_ctx, data, _env| *data += 1)
//         .padding(5.0);
// 
//     Flex::column().with_child(label).with_child(button)
// }
