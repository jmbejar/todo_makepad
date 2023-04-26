use makepad_widgets::*;
use makepad_widgets::text_input::*;


// The live_design macro generates a function that registers a DSL code block with the global
// context object (`Cx`).
//
// DSL code blocks are used in Makepad to facilitate live design. A DSL code block defines
// structured data that describes the styling of the UI. The Makepad runtime automatically
// initializes widgets from their corresponding DSL objects. Moreover, external programs (such
// as a code editor) can notify the Makepad runtime that a DSL code block has been changed, allowing
// the runtime to automatically update the affected widgets.
live_design!{
    import makepad_draw::shader::std::*;

    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::frame::Frame;
    import makepad_widgets::label::Label;
    import makepad_widgets::check_box::CheckBox;
    import makepad_widgets::text_input::TextInput;

    TITLE_TEXT = {
        font_size: (40),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (20),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    TodoPrompt = <Frame> {
        layout: {
            flow: Down,
            spacing: 10,
        },

        prompt = <Label> {
            draw_label: {
                color: #0,
                text_style: <REGULAR_TEXT>{},
            },
            text: "What is the next to add?"
        }

        input = <TextInput> {
            instance border_width: 1.0,
            walk: {width: 800, height: 40},
            draw_bg: {
                color: #223322
            }
            draw_label: {
                text_style:<REGULAR_TEXT>{font_size: (16)},
                color: #aaaaaa
            }
            text: "Write here your next task...",
        }
    }

    TodoList = {{TodoList}} {
        layout: {
            flow: Down,
            spacing: 10,
            padding: {left: 100}
        },
        walk: {width: Fill, height: Fit},
        checkbox: <CheckBox> {
            draw_check: {
                instance border_width: 1.0
                instance border_color: #223322
                instance border_color2: #229999
                size: 20.0,
            }
    
            draw_label: {
                text_style: <REGULAR_TEXT>{},
                fn get_color(self) -> vec4 {
                    return mix(
                        (#333333),
                        (#111111),
                        self.selected
                    )
                }
            }
    
            walk: {margin: {left: 50.0}, width: 800},
            label_walk: {margin: {left: 50.0}, width: 800} 
        }
    }

    // The `{{App}}` syntax is used to inherit a DSL object from a Rust struct. This tells the
    // Makepad runtime that our DSL object corresponds to a Rust struct named `App`. Whenever an
    // instance of `App` is initialized, the Makepad runtime will obtain its initial values from
    // this DSL object.
    App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        ui: <DesktopWindow>{frame: {body = {        
            show_bg: true
            layout: {
                flow: Down,
                spacing: 100,
                align: {
                    x: 0.5,
                    y: 0.2
                }
            },
            // The `walk` property determines how the frame widget itself is laid out. In this
            // case, the frame widget takes up the entire window.
            walk: {
                width: Fill,
                height: Fill
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    // Gradient color effect starting from a yellow tone
                    // The final color would be black, however the x value is divided to 3
                    // so the color gets darker slower.
                    return mix(#b0d1b1, #0, self.geom_pos.x / 3);
                }
            }
            // A label to display the counter.
            title = <Label> {
                draw_label: {
                    color: #0,
                    text_style: <TITLE_TEXT>{},
                },
                text: "My TODO list"
            }

            todo_prompt = <TodoPrompt> {
                walk: {width: Fit, height: Fit}
            }

            todo_list = <TodoList> {}
        }}}
    }
}

// This app_main macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop. On web, this means
// creating an event handler function that the browser event loop can call into.
app_main!(App);

// #[derive(Live, LiveHook)]
// #[live_design_with{
//     widget_factory!(cx, CheckBox)
// }]
// pub struct TodoItem {
// }

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct CheckBoxId(pub LiveId);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live, LiveHook)]
// This function is used to register any DSL code that you depend on.
// called automatically by the code we generated with the call to the macro `main_app` above.
#[live_design_with {
    crate::makepad_widgets::live_design(cx);
}]
pub struct App {
    // A chromeless window for our application. Used to contain our frame widget.
    // A frame widget. Used to contain our button and label.
    ui: WidgetRef,

    todos: Vec<String>
}

impl AppMain for App{
    
    // This function is used to handle any incoming events from the host system. It is called
    // automatically by the code we generated with the call to the macro `main_app` above.
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            // This is a draw event, so create a draw context and use that to draw our application.
            let mut draw_cx = Cx2d::new(cx, event);
            return self.ui.draw_widget(&mut draw_cx);
        }

        let mut new_todo:Option<String> = None;
        for widget_action in self.ui.handle_widget_event(cx, event) {
            if let TextInputAction::Return(value) = widget_action.action::<TextInputAction>() {
                new_todo = Some(value);
                break
            }
        }

        if let Some(new_todo_value) = new_todo {
            self.todos.push(new_todo_value);

            let text_input = self.ui.get_text_input(id!(input));
            text_input.set_text("");

            // This redraw is needed to have this element and the todo list updated upon pressing Enter
            text_input.redraw(cx); 
        }

        if let Some(mut todo_list) = self.ui.get_widget(id!(todo_list)).borrow_mut::<TodoList>() {
            todo_list.set_todos(self.todos.clone());
        }
    }
}

#[derive(Live, LiveHook)]
#[live_design_with{
    widget_factory!(cx, TodoList)
}]
pub struct TodoList {
    // It is mandatory to list here all the fields that are part of the live design block.
    // You may use `#[live]` but this is the default value, so no need to specify it.
    walk: Walk,
    layout: Layout,

    // This is also refered in the live design block, but it is not meant to be rendered automatically.
    // This is like a template element, that is used to create concrete instances that are
    // rendered by the draw_walk function, depending on the list of TODOs.
    checkbox: Option<LivePtr>,

    // The "rust" attribute is used to indicate there is no field with those names in the
    // "live design" definitions. Those fields are used in our own Rust code.
    #[rust] todos: Vec<String>,
    #[rust] items: ComponentMap<CheckBoxId, CheckBoxRef>
}

impl Widget for TodoList {  
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)
    ) {
        self.handle_event_with(cx, event, &mut | cx, action | {
            dispatch_action(cx, action);
        });
    }

    fn get_walk(&self)->Walk{ self.walk }
    
    fn redraw(&mut self, _cx:&mut Cx){
        // Not sure how I can implement this method if I don't have an Area
        // (which is what I see in many examples).
        // I don't know what is an Area used for.

        //self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl TodoList {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        for (_id, item) in self.items.iter_mut() {
            let item_uid = item.widget_uid();
            if let Some(mut inner) = item.borrow_mut(){
                inner.handle_event_with(cx, event, &mut | cx_imm, action | {
                    dispatch_action(cx_imm, WidgetActionItem::new(action.into(), item_uid));
                });
            }
        }
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // This was needed to apply the layout defined for TodoList in the live design block.
        // Otherwise, it is ignored.
        cx.begin_turtle(walk, self.layout);

        for (i, value) in self.todos.iter().enumerate() {
            let item_id = LiveId(i as u64).into();
            let current_checkbox = self.items.get_or_insert(cx, item_id, | cx | {
                CheckBoxRef::new_from_ptr(cx, self.checkbox)
            });
            
            current_checkbox.set_label_text(value);
            let _ = current_checkbox.draw_walk_widget(cx, walk);
        }

        cx.end_turtle();
        self.items.retain_visible();
    }

    pub fn set_todos(&mut self, todos: Vec<String>) {
        self.todos = todos
    }
}