use yew::prelude::*;
use crate::models::get_experience_data; 
use crate::components::job_card::JobCard;
use crate::components::skills::TechnicalArsenal;
use crate::components::languages::Languages;

#[function_component(App)]
pub fn app() -> Html {
    let experiences = get_experience_data();

    html! {
        <main class="max-w-4xl mx-auto p-8 bg-slate-900 text-white min-h-screen">
            <header class="mb-12 border-b border-slate-800 pb-8">
                <h1 class="text-5xl font-extrabold tracking-tight">{"Yerkin Amanzhol"}</h1>
                <p class="text-orange-500 text-xl mt-2 font-mono">{"SOC Analyst | Endpoint Security (EPS) | Rust Enthusiast"}</p>
            </header>

            //Experience 
            <section id="experience">
                <h2 class="text-2xl font-semibold mb-6 flex items-center">
                    {"Professional Experience"}
                </h2>
                
                <div class="space-y-4">
                    { for experiences.into_iter().map(|job| html! {
                        <JobCard job={job} />
                    }) }
                </div>
            </section>
            
            //Skills
            <TechnicalArsenal />
            <section id="skills" class="mt-16">
                <h2 class="text-2xl font-semibold mb-6 flex items-center">
                    {"Certifications"}
                </h2>
                <ul class="space-y-2 font-mono text-sm text-slate-300">
                    <li class="flex items-center"><span class="text-orange-500 mr-2">{"▹"}</span>{"Microsoft SC-900"}</li> // [cite: 16]
                    <li class="flex items-center"><span class="text-orange-500 mr-2">{"▹"}</span>{"QualysGuard Certified Specialist"}</li> // [cite: 13]
                    <li class="flex items-center"><span class="text-orange-500 mr-2">{"▹"}</span>{"Siemplify Certified SOAR Analyst"}</li> // [cite: 14, 15]
                </ul>
            <Languages />
            </section>
        </main>
    }
}

