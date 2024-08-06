use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    text::Text,
};
use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window};

pub async fn emulator_main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(570, 240));
    let output_settings = OutputSettingsBuilder::new().theme(BinaryColorTheme::OledBlue).build();
    let mut window = Window::new("CoDiOS Emulator", &output_settings);
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let text = Text::new("Hello World!", Point::zero(), text_style);
    let background = Rectangle::new(Point::new(0, 0), Size::new(570, 240))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off));
    background.draw(&mut display)?;
    text.draw(&mut display)?;
    window.update(&display);

    Ok(())
}
