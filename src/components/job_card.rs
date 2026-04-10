use yew::prelude::*;
use crate::models::Experience;

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub job: Experience,
}

#[function_component(JobCard)]
pub fn job_card(props: &JobProps) -> Html { // Experience animation
    let job = &props.job;
    let is_expanded = use_state(|| false);
    let onmouseenter = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(true))
    };
    
    let onmouseleave = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(false))
    };

    let container_style = if *is_expanded {
        "border-orange-500 bg-slate-800/50 shadow-lg shadow-orange-500/10"
    } else {
        "border-slate-800 bg-transparent"
    };

    let list_style = if *is_expanded {
        "opacity-100 max-h-96 translate-y-0 scale-100"
    } else {
        "opacity-0 max-h-0 -translate-y-2 scale-95"
    };

    html! {
        <div 
            class={classes!("p-6", "rounded-xl", "border", "transition-all", "duration-500", "cursor-default", container_style)}
            {onmouseenter}
            {onmouseleave}
        >
            <div class="flex justify-between items-start mb-2">
                <div>
                    <h3 class={classes!("text-xl", "font-bold", "transition-colors", if *is_expanded { "text-orange-400" } else { "text-white" })}>
                        {&job.title}
                    </h3>
                    <p class="text-slate-400 font-medium">{&job.company}</p>
                </div>
                <span class="text-xs font-mono text-slate-500 bg-slate-800 px-2 py-1 rounded">
                    {&job.period}
                </span>
            </div>

            <ul class={classes!("space-y-3", "overflow-hidden", "transition-all", "duration-500", "ease-out", list_style)}>
                <div class="pt-4 border-t border-slate-700/50 mt-2">
                    { for job.bullets.iter().map(|bullet| html! {
                        <li class="flex items-start text-sm text-slate-300 leading-relaxed">
                            <span class="text-orange-500 mr-3 mt-1 text-xs">{"▶"}</span>
                            {bullet}
                        </li>
                    }) }
                </div>
            </ul>
        </div>
    }
}