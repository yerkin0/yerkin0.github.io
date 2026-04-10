use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SkillProps {
    pub name: String,
}

#[function_component(SkillTag)]
pub fn skill_tag(props: &SkillProps) -> Html {
    html! {
        <div class="bg-slate-800/50 p-3 rounded border border-slate-700 hover:border-orange-500/50 transition-colors group">
            <span class="text-slate-400 group-hover:text-orange-400 font-mono text-sm">
                { &props.name }
            </span>
        </div>
    }
}

#[function_component(TechnicalArsenal)]
pub fn technical_arsenal() -> Html {
    let skills = vec![
        "Microsoft Sentinel", "Microsoft Defender XDR", "IOC Management","KQL", 
        "Excel", "Power Automate", "Wireshark", "TCPdump", "Incident Response",
        "Endpoint Security", "Log Analysis", "Rust / Yew / WASM"
    ];

    html! {
        <section id="skills" class="mt-16">
            <h2 class="text-2xl font-semibold mb-6 flex items-center">
                {"Technical Arsenal"}
            </h2>
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                { for skills.into_iter().map(|skill| html! {
                    <SkillTag name={skill.to_string()} />
                }) }
            </div>
        </section>
    }
}