use super::{
    AddScanner,
    DragRect,
    Draggable,
    RemoveScanner,
    Scannable,
    ScannerSpec,
};
use crate::{
    state::UIState,
    util::{detect_center_element, nodelist_to_elements},
    views::cards::{CardFocusArea, Cards},
};
use hes_engine::Game;
use leptos::*;
use leptos_use::{
    use_document,
    use_event_listener,
    use_interval_fn,
};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
enum Mode {
    /// Can start scanning or scrolling.
    Any,

    /// Currently scanning, so can't scroll.
    Scan,

    /// Currently scrolling, so can't scan.
    Scroll,
}
impl Mode {
    fn can_scroll(&self) -> bool {
        match self {
            Mode::Any | Mode::Scroll => true,
            _ => false,
        }
    }

    fn can_scan(&self) -> bool {
        match self {
            Mode::Any | Mode::Scan => true,
            _ => false,
        }
    }
}

#[component]
pub fn ScannerCards<S: ScannerSpec>(
    #[prop(into)] items: Signal<Vec<S::Item>>,
    spec: S,
) -> impl IntoView {
    let focused = create_rw_signal(None);
    let (mode, set_mode) = create_signal(Mode::Any);
    let (drag_rect, set_drag_rect) = create_signal(None);
    let (card_height, set_card_height) = create_signal(0.);

    let can_scroll = move || mode.get().can_scroll();
    let can_scan = move || mode.get().can_scan();

    let on_drag = move |rect: DragRect| {
        // This triggers the scanner functionalities
        tracing::debug!("Scanner > Dragging");
        set_drag_rect.set(Some(rect));
        set_mode.set(Mode::Scan);
    };

    let update_focused = move || {
        // Figure out what the focused card is
        if let Some(scroller) =
            document().query_selector(".cards").unwrap()
        {
            let els = document()
                .query_selector_all(".draggable")
                .unwrap();
            if els.length() > 0 {
                let els = nodelist_to_elements(els);
                if let Some(idx) = detect_center_element(
                    scroller
                        .dyn_into::<web_sys::HtmlElement>()
                        .expect("Is an html element"),
                    &els,
                ) {
                    set_card_height.set(
                        els[idx]
                            .get_bounding_client_rect()
                            .height(),
                    );

                    let item = items
                        .with(|items| items.get(idx).cloned());
                    focused.set(item);
                }
            }
        }
    };

    // let scroll_to_next = move || {
    //     if let Some(scroller) =
    //         document().query_selector(".cards").unwrap()
    //     {
    //         let els = document()
    //             .query_selector_all(".draggable")
    //             .unwrap();
    //         if els.length() > 0 {
    //             let els = nodelist_to_elements(els);
    //             if let Some(idx) = detect_center_element(
    //                 scroller
    //                     .dyn_into::<web_sys::HtmlElement>()
    //                     .expect("Is an html element"),
    //                 &els,
    //             ) {
    //                 if idx < els.len() - 1 {
    //                     els[idx + 1].scroll_to();
    //                 }
    //             }
    //         }
    //     }
    // };
    let _ = use_event_listener(
        use_document(),
        ev::wheel,
        move |ev: ev::WheelEvent| {
            if let Some(scroller) =
                document().query_selector(".cards").unwrap()
            {
                let scroller = scroller
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("Is an html element");
                let s = scroller.scroll_left();
                scroller.set_scroll_left(
                    s + ev.delta_y().round() as i32,
                );
            }
        },
    );

    // use_interval_fn(
    //     move || {
    //         // TODO perhaps more efficient way to do this?
    //         tracing::debug!(
    //             "Scanner > Updating Focused in Interval"
    //         );
    //         update_focused();
    //     },
    //     60,
    // );

    let on_scroll_start = move |_| {
        tracing::debug!("Scanner > Scroll Started");
        set_mode.set(Mode::Scroll);
    };
    let on_scroll_end = move |_| {
        tracing::debug!("Scanner > Scroll Ended");
        set_mode.set(Mode::Any);
        update_focused();
    };

    let on_drag_stop = move |_| {
        tracing::debug!("Scanner > Drag Stopped");
        // This stops/cancels the scanner functionalities
        set_mode.set(Mode::Any);
        set_drag_rect.set(None);
    };

    let (top_y_bound, set_top_y_bound) = create_signal(0.);
    let (bot_y_bound, set_bot_y_bound) = create_signal(0.);
    let y_bounds =
        move || [top_y_bound.get(), bot_y_bound.get()];

    let ui = expect_context::<RwSignal<UIState>>();
    let on_focus = move |idx: Option<usize>| {
        tracing::debug!("Scanner > Cards > On Focus");

        let item = idx
            .map(|idx| {
                items.with(|items| items.get(idx).cloned())
            })
            .flatten();

        if let Some(item) = &item {
            let id = item.id();
            ui.update(|ui| {
                if !ui.viewed.contains(id) {
                    ui.viewed.push(*id);
                }
            });
        }
        focused.set(item);
    };

    let add_props = spec.add_props(focused);
    let rem_props = spec.rem_props(focused);
    let focused_id = move || {
        focused
            .with(|item| item.as_ref().map(|item| *item.id()))
    };

    let game = expect_context::<RwSignal<Game>>();

    view! {
        <Show when=move || focused.with(|item| item.is_some())>
            <AddScanner
                scan_time=add_props.scan_time
                drag_rect
                should_show=add_props.should_show
                scan_allowed=add_props.scan_allowed
                on_finish_scan=add_props.on_finish_scan
                set_y_bound=move |(_top, bot)| {
                    set_top_y_bound.set(bot - 10.);
                }
            />

            <RemoveScanner
                scan_time=rem_props.scan_time
                drag_rect
                label=rem_props.label.unwrap()
                should_show=rem_props.should_show
                scan_allowed=rem_props.scan_allowed
                on_finish_scan=rem_props.on_finish_scan
                set_y_bound=move |(top, _bot)| {
                    set_bot_y_bound.set(top + 10. - card_height.get() as f32);
                }
            />

        </Show>
        <Cards
            enabled=can_scroll
            on_focus
            on_scroll_start
            on_scroll_end
        >
            <For
                each=move || items.get().into_iter()
                key=|item| *item.id()
                children=move |item| {
                    let id = *item.id();
                    let draggable = move || {
                        can_scan() && focused_id() == Some(id)
                    };

                    let item = create_memo(move |_| {
                        game.with(move |game| S::Item::get_from_state(&id, &game))
                    });
                    let card = S::Item::as_card(item.into());
                    view! {
                        <Draggable
                            on_drag
                            on_drag_stop
                            y_bounds
                            draggable=draggable.into_signal()
                        >
                            {card}
                        </Draggable>
                    }
                }
            />

        </Cards>

        <CardFocusArea/>
    }
}
