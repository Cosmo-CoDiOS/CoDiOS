use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

pub fn emulator_main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(240, 536));

    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    let char_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)?;

    let text = "CoDiOS INIT!";

    Text::with_alignment(
        text,
        display.bounding_box().center(),
        char_style,
        Alignment::Center,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .scale(1)
        .build();

    Window::new("CoDiOS Emulator", &output_settings).show_static(&display);

    Ok(())
}
