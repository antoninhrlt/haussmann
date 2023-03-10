trait Widget {}

struct Label {}

impl Widget for Label {}

impl From<Label> for Box<dyn Widget> {
    fn from(value: Label) -> Self {
        Box::new(value)
    }
}

struct Button {}

impl Widget for Button {}

impl From<Button> for Box<dyn Widget> {
    fn from(value: Button) -> Self {
        Box::new(value)
    }
}

macro_rules! widgets {
    ($first:expr $(, $widget:expr) *) => {
        {
            let widgets: Vec<Box<dyn Widget>> = vec![$first.into(), $($widget.into()),*];
            widgets
        }
    };
}

fn foo<T>(_widgets: Vec<T>)
where
    T: From<Box<dyn Widget>>,
{
}

#[test]
fn dyn_widgets() {
    let label = Label {};
    let button = Button {};

    foo(widgets![button, label]);
}
