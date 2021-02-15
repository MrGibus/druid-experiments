//! This Module handles the custom titlebar and associated widgets

use std::marker::PhantomData;
use druid::{Color, WidgetExt, WindowState, UnitPoint, Data, Application};
use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, Svg, SvgData, ControllerHost, Click};

// Window buttons: ~46 x 28

pub struct TitleBar {
    height: f64,
    size: Size,
}

impl TitleBar {
    fn raw() -> Self {
        TitleBar {
            height: 32.,
            size: Size::default(),
        }
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Align<T>
        where T: druid::Data {
        // resources
        let min_svg = match include_str!("resources/min.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => {
                SvgData::empty()
            }
        };

        let max_svg = match include_str!("resources/max.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => {
                SvgData::empty()
            }
        };

        let restore_svg = match include_str!("resources/restore.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => {
                SvgData::empty()
            }
        };

        let exit_svg = match include_str!("resources/exit.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => {
                SvgData::empty()
            }
        };

        Flex::row()
            .with_default_spacer()
            .with_child(Label::new("Titlebar TEST"))
            .with_flex_child(TitleBar::raw().expand_width(), 1.)
            .with_child(WindowButton::new(min_svg)
                .on_click(|ctx, _, _| ctx.window().to_owned().set_window_state(WindowState::MINIMIZED)))
            .with_child(WindowButton::with_alt_icon(
                restore_svg, max_svg)
                .on_click(|ctx, _, _| {
                    match  ctx.window().get_window_state() {
                        WindowState::RESTORED => ctx.window().to_owned().set_window_state(WindowState::MAXIMIZED),
                        _ => ctx.window().to_owned().set_window_state(WindowState::RESTORED),
                    }
                }))
            .with_child(WindowButton::with_hightlight(
                exit_svg,
                Color::rgba(0.9, 0.1, 0.1, 0.6))
                .on_click(|_, _, _| Application::global().quit()))
            // .background(Color::grey(0.1))
            .align_vertical(UnitPoint::TOP)
    }
}

impl<T: Data> Widget<T> for TitleBar {
    fn event(&mut self, ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
        ctx.window().handle_titlebar(true);
    }

    // no functionality
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _ev: &LifeCycle,
        _data: &T,
        _env: &Env,
    ) {}

    // no functionality
    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old: &T,
        _new: &T,
        _env: &Env,
    ) {}

    // sets boundaries
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        let size = Size::new(Size::default().expand().width, self.height);
        self.size = bc.constrain(size);
        self.size
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {}
}


struct WindowButton<T> {
    icon: SvgData,
    alt_icon: Option<SvgData>,
    highlight: Color,
    phantom: PhantomData<T>
}

impl<T: Data> WindowButton<T> {

    fn default() -> Self {
        WindowButton {
            icon: SvgData::empty(),
            alt_icon: None,
            highlight: Color::rgba(0.5, 0.5, 0.5, 0.2),
            phantom: PhantomData,
        }
    }

    fn new(icon: SvgData) -> Self {
        WindowButton {
            icon,
            ..Self::default()
        }
    }

    fn with_hightlight(icon: SvgData, highlight: Color) -> Self {
        WindowButton {
            icon,
            highlight,
            ..Self::default()
        }
    }

    fn with_alt_icon(icon: SvgData, alt_icon: SvgData) -> Self {
        WindowButton {
            icon,
            alt_icon: Some(alt_icon),
            ..Self::default()
        }
    }

    fn on_click(self, f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,)
        -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for WindowButton<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
    }

    // no functionality
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _ev: &LifeCycle,
        _data: &T,
        _env: &Env,
    ) {}

    // no functionality
    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old: &T,
        _new: &T,
        _env: &Env,
    ) {}

    // sets boundaries
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        let size = Size::new(46., 32.);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let mut icon: Svg;

        if let Some(alt) = self.alt_icon.clone() {
            match ctx.window().get_window_state() {
                WindowState::RESTORED => icon = Svg::new(alt),
                _ => icon = Svg::new(self.icon.clone()),
            }
        } else {
            icon = Svg::new(self.icon.clone())
        }

        let rect = ctx.size().to_rect();
        ctx.clip(rect);

        // this will highlight the button when hovered
        if ctx.is_hot() {
            ctx.fill(rect, &self.highlight)
        }

        // draw the svg data last
        icon.paint(ctx, data, env);
    }
}

