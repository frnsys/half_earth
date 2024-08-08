use crate::{
    memo,
    views::{DisplayEvent, Events, Updates},
};
use hes_engine::{
    ProjectType,
    ResolvedEvent,
    State,
    Status,
    Update,
};
use leptos::*;

#[component]
pub fn DebugEvents() -> impl IntoView {
    let events = create_rw_signal(vec![]);
    let updates = create_rw_signal(vec![]);
    let game = expect_context::<RwSignal<State>>();

    let region = game.with_untracked(|game| {
        let region = game.world.regions.first();
        (region.id, region.name.clone())
    });
    let game_events = memo!(game.world.events);
    let event_views = move || {
        game_events.get().iter().map(|event| {
            let name = event.name.clone();
            let event = ResolvedEvent {
                event: event.clone(),
                region: if event.is_regional() {
                    Some(region.clone())
                } else {
                    None
                }
            };

            view! {
                <div class="debug-event" on:click=move |_| {
                    let event = game.with_untracked(|game| DisplayEvent::new(event.clone(), game));
                    update!(|events| events.push(event));
                }>
                    {name}
                </div>
            }
        }).collect::<Vec<_>>()
    };

    let regions = memo!(game.world.regions);
    let region_updates =
        move || {
            regions.get().iter().map(|region| {
            let id = region.id;
            view! {
                <div class="debug-event" on:click=move |_| {
                    let up = Update::Region {
                        id,
                        up: true,
                    };
                    let down = Update::Region {
                        id,
                        up: false,
                    };
                    update!(|updates| {
                        updates.push(up);
                        updates.push(down);
                    });
                }>
                    {&region.name}
                </div>
            }
        }).collect::<Vec<_>>()
        };

    let projects = memo!(game.world.projects);
    let project_updates = move || {
        projects.get().iter().map(|project| {
            let id = project.id;
            let kind = project.kind;
            let n_outcomes = project.outcomes.len();
            if n_outcomes == 1 {
                view! {
                    <div class="debug-event event-trigger" on:click=move |_| {
                        let update = match kind {
                            ProjectType::Policy => Update::Policy {
                                id,
                            },
                            _ => Update::Project {
                                id
                            }
                        };
                        update!(|game| {
                            game.world.projects[&id].status = Status::Active;
                            game.world.projects[&id].active_outcome = Some(0);
                        });
                        update!(|updates| {
                            updates.push(update);
                        });
                    }>
                        {&project.name}
                    </div>
                }.into_view()
            } else {
                let inner = (0..n_outcomes).map(|i| {
                    view! {
                        <div class="event-trigger" on:click=move |_| {
                            let update = match kind {
                                ProjectType::Policy => Update::Policy {
                                    id,
                                },
                                _ => Update::Project {
                                    id
                                }
                            };
                            update!(|game| {
                                game.world.projects[&id].status = Status::Active;
                                game.world.projects[&id].active_outcome = Some(i);
                            });
                            update!(|updates| {
                                updates.push(update);
                            });
                        }>{format!("[Outcome {i}]")}</div>
                    }
                }).collect::<Vec<_>>();
                view! {
                    <div class="debug-event">
                        {&project.name}
                        <div class="debug-outcomes">
                            {inner}
                        </div>
                    </div>
                }.into_view()
            }
        }).collect::<Vec<_>>()
    };

    view! {
        <div class="debug-events">
            {event_views}
            <hr />
            {region_updates}
            <hr />
            {project_updates}
        </div>
        <Events events />
        <Updates updates on_done=move |_| {
            update!(|updates| updates.clear());
        }/>
    }
}
