use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LanguageProps {
    pub name: String,
    pub level: String,
    pub width: String, // Tailwind 
}

#[function_component(LanguageItem)]
pub fn language_item(props: &LanguageProps) -> Html {
    html! {
        <div class="mb-4">
            <div class="flex justify-between mb-1">
                <span class="text-sm font-medium text-white">{ &props.name }</span>
                <span class="text-xs font-mono text-slate-500">{ &props.level }</span>
            </div>
            <div class="w-full bg-slate-800 rounded-full h-1.5">
                <div class={classes!(
                    "bg-orange-500", "h-1.5", "rounded-full", "transition-all", "duration-1000",
                    props.width.clone()
                )}></div>
            </div>
        </div>
    }
}

#[function_component(Languages)]
pub fn languages() -> Html {
    html! {
        <section id="languages" class="mt-16">
            <h2 class="text-2xl font-semibold mb-6">{"Languages"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12">
                /* Data directly from your CV  */
                <LanguageItem name="Kazakh" level="Native" width="w-full" />
                <LanguageItem name="Russian" level="Fluent" width="w-[95%]" />
                <LanguageItem name="English" level="Upper-Intermediate" width="w-[75%]" />
                <LanguageItem name="Polish" level="Basic" width="w-[25%]" />
            </div>
        </section>
    }
}