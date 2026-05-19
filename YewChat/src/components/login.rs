use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="flex min-h-screen w-screen items-center justify-center bg-slate-950 px-6 text-white">
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_20%_20%,_rgba(34,211,238,0.22),_transparent_30%),radial-gradient(circle_at_80%_10%,_rgba(16,185,129,0.16),_transparent_28%),linear-gradient(135deg,_#020617,_#0f172a)]"></div>
            <div class="relative w-full max-w-xl">
                <div class="mb-8">
                    <div class="text-xs font-semibold uppercase tracking-[0.3em] text-cyan-300">{"YewChat"}</div>
                    <h1 class="mt-4 text-5xl font-bold">{"Enter Orbit Room"}</h1>
                    <p class="mt-4 text-base text-slate-300">{"Join a live Rust-powered WebSocket chat with a cleaner interface, online presence, and instant broadcast messages."}</p>
                </div>
                <form class="flex rounded-2xl bg-white/10 p-2 shadow-2xl ring-1 ring-white/10 backdrop-blur">
                    <input {oninput} class="min-w-0 flex-1 rounded-xl border border-white/10 bg-white px-5 py-4 text-slate-900 outline-none placeholder:text-slate-400" placeholder="Choose a username" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="ml-2 rounded-xl bg-cyan-400 px-7 py-4 font-bold text-slate-950 transition hover:bg-cyan-300 disabled:cursor-not-allowed disabled:bg-slate-500 disabled:text-slate-300" >
                            {"Launch Chat"}
                        </button>
                    </Link<Route>>
                </form>
                <div class="mt-5 text-sm text-slate-400">{"Tip: open a second browser tab with another username to test the broadcast."}</div>
            </div>
        </div>
    }
}
