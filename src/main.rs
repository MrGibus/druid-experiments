mod titlebar;

use std::time::Duration;

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, WindowState, Event, EventCtx, Env, Data, Lens, Size, WidgetExt, Point, AppDelegate, DelegateCtx, Target, Command, Selector, Handled, TimerToken};
use druid::widget::{Controller, CrossAxisAlignment, MainAxisAlignment, Spinner, Button, Label, Flex,
                    Align};

const SPLASH_TIME: Duration = Duration::from_secs(2);


pub fn main() -> Result<(), PlatformError> {
    // let disp = Screen::get_display_rect();
    let data = AppData::new();

    let splash = WindowDesc::new(build_spash)
        .window_size(Size::new(500., 300.))
        .set_position(Point::new(1000., 500.))
        .show_titlebar(false)
        .set_window_state(WindowState::RESTORED)
        .resizable(false);

    AppLauncher::with_window(splash)
        .delegate(Delegate)
        .launch(data)?;
    Ok(())
}


#[derive(Clone, Data, Lens)]
struct AppData {
    count: u64,
}

impl AppData {
    fn new() -> AppData {
        AppData {
            count: 0
        }
    }
}

fn main_window() -> impl Widget<AppData> {

    let layout = Align::centered(Flex::row()
        .with_child(Label::new(|data: &AppData, _: &_| {
            format!("{}", data.count)
        }))
        .with_spacer(25.)
        .with_child(Button::new("Count")
                             .on_click(|_, data: &mut AppData, _: &_| data.count += 1)
        )
    );

    // title bar aspect
    let titlebar = titlebar::TitleBar::new();

    Flex::column()
        .with_child(titlebar)
        .with_flex_child(layout, 1.)
        .controller(MainControl)
}


// Window Controls and Commands
// ============================
const CMD_START: Selector = Selector::new("cmd_start_main_window");
const CMD_MAIN_WINDOW_CONNECTED: Selector = Selector::new("cmd_main_window_connected");

struct Delegate;

impl <T: Data> AppDelegate<T> for Delegate{
    // this now returns a handled enum instead of a bool for readability purposes
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut T,
        _env: &Env
    ) -> Handled {
        match cmd {
            _ if cmd.is(CMD_START) => {
                // thread::sleep(SPLASH_TIME);
                let main_window = WindowDesc::new(main_window)
                    .set_window_state(WindowState::MAXIMIZED)
                    .show_titlebar(false);

                ctx.new_window(main_window);
                Handled::Yes
            },
            _ => Handled::No
        }
    }
}

fn build_spash<T>() -> impl Widget<T> where T: Data {
    let label = Label::new("SPLASH SCREEN TEST").with_text_size(38.);
    let spinner = Spinner::new().fix_size(38., 38.);

    Flex::column()
        .with_flex_child(label, 1.)
        .with_spacer(38.)
        .with_child(spinner)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .controller(SplashControl::new())
}

struct SplashControl {
    timer_id_start: TimerToken,
    timer_id_close: TimerToken,
}

impl SplashControl {
    fn new() -> Self {
        SplashControl {
            timer_id_start: TimerToken::INVALID,
            timer_id_close: TimerToken::INVALID,
        }
    }
}

impl <W: Widget<T>, T: Data> Controller<T, W> for SplashControl {
        fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);
        match event {
            Event::WindowConnected => {
                println!("Connected event running");
                self.timer_id_start = ctx.request_timer(SPLASH_TIME);
                // ctx.submit_command(CMD_START);
            },
            // Event::MouseDown(_) => ctx.submit_command(CMD_START),
            Event::Command(cmd) => {
                if cmd.is(CMD_MAIN_WINDOW_CONNECTED) {
                    // wait a second before closing the splash
                    self.timer_id_close = ctx.request_timer(SPLASH_TIME)
                }
            },
            Event::Timer(id) => {
                if id == &self.timer_id_close {
                    ctx.window().close();
                }
                if id == &self.timer_id_start {
                    println!("STARTING");
                    ctx.submit_command(CMD_START);
                }
            },
            _ => ()
        }
    }
}

struct MainControl;
impl <W: Widget<T>, T: Data> Controller<T, W> for MainControl {
        fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);
        if let Event::WindowConnected = event {
            //not sure how I get a specific windowID
            let splash_cmd = Command::new(
                CMD_MAIN_WINDOW_CONNECTED,
                (),
                Target::Global
            );
            ctx.submit_command(splash_cmd)
        };
    }
}