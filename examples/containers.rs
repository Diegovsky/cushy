use cushy::figures::units::Lp;
use cushy::figures::{Lp2D, Point};
use cushy::reactive::value::{Destination, Dynamic};
use cushy::widget::{MakeWidget, WidgetInstance};
use cushy::widgets::container::ContainerShadow;
use cushy::window::ThemeMode;
use cushy::Run;

fn main() -> cushy::Result {
    let theme_mode = Dynamic::default();
    set_of_containers(3, theme_mode.clone())
        .centered()
        .into_window()
        .themed_mode(theme_mode)
        .run()
}

fn set_of_containers(repeat: usize, theme_mode: Dynamic<ThemeMode>) -> WidgetInstance {
    let inner = if let Some(remaining_iters) = repeat.checked_sub(1) {
        set_of_containers(remaining_iters, theme_mode)
    } else {
        "Toggle Theme Mode"
            .into_button()
            .on_click(move |_| {
                theme_mode.map_mut(|mut mode| mode.toggle());
            })
            .make_widget()
    };
    "Lowest"
        .and(
            "Low"
                .and(
                    "Mid"
                        .and(
                            "High"
                                .and(
                                    "Highest"
                                        .and(inner)
                                        .into_rows()
                                        .contain()
                                        .shadow(drop_shadow()),
                                )
                                .into_rows()
                                .contain()
                                .shadow(drop_shadow()),
                        )
                        .into_rows()
                        .contain()
                        .shadow(drop_shadow()),
                )
                .into_rows()
                .contain()
                .shadow(drop_shadow()),
        )
        .into_rows()
        .contain()
        .shadow(drop_shadow())
        .make_widget()
}

fn drop_shadow() -> ContainerShadow<Lp> {
    ContainerShadow::new(Point::mm(0, 1)).spread(Lp::mm(1))
}
