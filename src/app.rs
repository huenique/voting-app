use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container mx-auto">
            <div class="flex flex-col items-center justify-center min-h-screen w-full">
                <Card>
                    <Poll />
                </Card>
            </div>
        </div>
    }
}

#[component]
fn Card(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="max-w-lg w-full bg-white rounded-lg shadow-md">
        {children(cx)}
        </div>
    }
}

#[component]
fn Poll(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="p-6">
            <h2 class="text-2xl font-bold mb-4">"Poll List"</h2>
            <ul class="space-y-4">
                <li class="border border-gray-300 rounded-md p-4 flex items-center justify-between">
                    <label class="flex items-center space-x-2">
                        <input type="radio" name="poll" class="form-radio text-blue-500 h-5 w-5" />
                        <span class="text-lg">"Option 1"</span>
                    </label>
                </li>
                <li class="border border-gray-300 rounded-md p-4 flex items-center justify-between">
                    <label class="flex items-center space-x-2">
                        <input type="radio" name="poll" class="form-radio text-blue-500 h-5 w-5" />
                        <span class="text-lg">"Option 2"</span>
                    </label>
                </li>
            </ul>
            <div class="mt-6 flex justify-between">
                <button class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:bg-green-600">
                    "Submit Vote"
                </button>
            </div>
        </div>
    }
}
