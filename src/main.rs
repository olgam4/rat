use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
    id: usize,
    count: RwSignal<i32>,
}

#[component]
fn Counters(cx: Scope) -> Element {
    let (counters, set_counters) = create_signal::<Vec<Counter>>(
        cx,
        vec![Counter {
            id: 0,
            count: create_rw_signal(cx, 0),
        }],
    );

    let add_counter = move |_| {
        set_counters.update(|counters| {
            let id = counters.len();
            counters.push(Counter {
                id,
                count: create_rw_signal(cx, 0),
            });
        })
    };

    view! {
      cx,
      <div class="flex flex-col">
        <For
          each=counters
          key=|counter| counter.id
        >
          {|cx: Scope, counter: &Counter| {
            let count = counter.count;
            view! {
              cx,
              <div class="w-full flex justify-center space-x-2">
                  <p>"Value: " {move || count.get()}</p>
                  <button
                    on:click=move |_| count.update(|count| *count += 1)
                    >
                        "Increment"
                    </button>
              </div>
            }
          }
        }
        </For>
        <button on:click=add_counter>
            "+"
        </button>
      </div>
    }
}
pub fn main() {
    mount_to_body(|cx| view! { cx, <Counters /> })
}
