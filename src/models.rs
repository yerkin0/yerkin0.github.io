#[derive(Clone, PartialEq, Debug)]  
pub struct Experience {        
    pub title: String,        
    pub company: String,       
    pub location: String,      
    pub period: String,        
    pub bullets: Vec<String>,  
}

pub fn get_experience_data() -> Vec<Experience> {
    vec![
        Experience { 
            title: "Cyber Security Analyst (SOC/EPS)".into(), 
            company: "HCLTech".into(), 
            location: "Remote, Poland".into(), 
            period: "Sept 2024 - Present".into(),
            bullets: vec![
                "Perform real-time threat monitoring and incident triage within a 24/7 SOC environment, utilizing Microsoft Sentinel and Defender XDR.".into(),
                "Collaborated with the Sentinel team to fine-tune detection rules, successfully reducing false positives by 70% and increasing Tier 1 analyst capacity.".into(),
                "Manage Endpoint Security (EPS) policies and health across global estates, ensuring 99%+ agent compliance.".into(),
                "Curate and maintain Indicator of Compromise (IOC) lists in Microsoft Defender to proactively block emerging threats and reduce attack surface.".into(),
                "Coordinate with the Antivirus (AV) team to tune alert logic, significantly reducing false positives in the endpoint environment.".into(),
            ]
        },
        Experience { 
            title: "Senior IT Analyst".into(), 
            company: "HCLTech".into(), 
            location: "Krakow, Poland".into(), 
            period: "Jan 2023 - Sept 2024".into(),
            bullets: vec![
                "Provided Tier 2 technical support for enterprise-level infrastructure".into(),
                "Created and maintained knowledge base articles".into(),
                "Mentored junior analysts on IT troubleshooting and security hygiene best practices.".into(),
                "Assisted management with reporting on team performance, incidents, and other metrics".into(),
            ]
        },
        Experience { 
            title: "IT Analyst".into(), 
            company: "HCLTech".into(), 
            location: "Krakow, Poland".into(), 
            period: "Sept 2021 - Dec 2022".into(),
            bullets: vec![
                "Provided global technical support for hardware, software, and networking issues in a multi-OS environment.".into(),
                "Managed user identities and access controls via Active Directory and Azure AD.".into(),
                "Documented incidents, resolutions, and knowledge base articles to improve support efficiency.".into(),
                "Managed tickets in ITSM tools, ensuring timely resolution and escalation when needed.".into(),
            ]
        },
    ]
}