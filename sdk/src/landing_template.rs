use stremio_core::types::addons::Manifest;


static STYLESHEET: &str = include_str!("../landing_style.css");

fn get_contact_html(addon_name: &str, email: &str) -> String {
    format!(r#"
        <div class="contact">
            <p>Contact {} creator:</p>
            <a href="mailto:{email}">{email}</a>
        </div>
    "#, addon_name, email = email)
}

// "taylor" => "Taylor"
// "édouard" => "édouard"
fn make_ascii_sentence_case(s: &mut str) {
    if let Some(first_char) = s.get_mut(..1) {
        first_char.make_ascii_uppercase();
    }
}

pub fn landing_template(manifest: &Manifest) -> String {
    let background = manifest.background.as_deref()
        .unwrap_or("https://dl.strem.io/addon-background.jpg");

    let logo = manifest.logo.as_deref()
        .unwrap_or("https://dl.strem.io/addon-logo.png");

    let contact_html = manifest.contact_email.as_ref().map(|email| {
        get_contact_html(&manifest.name, email)
    }).unwrap_or_default();

    let description = manifest.description.as_deref().unwrap_or_default();

    let version = manifest.version.to_string();
    
    let types = manifest
        .types
        .clone()
        .into_iter()
        .map(|mut type_| {
            make_ascii_sentence_case(&mut type_);
            format!("<li>{}{}</li>", type_, if type_ != "Series" { "s" } else {""})
        })
        .collect::<Vec<_>>()
        .join("");

    format!(r#"

        <!DOCTYPE html>
        <html style="background-image: url({background});">
    
        <head>
        <meta charset="utf-8">
        <title>{name} - Stremio Addon</title>
        <style>{stylesheet}</style>
        <link rel="shortcut icon" href="{logo}" type="image/x-icon">
        <link href="https://fonts.googleapis.com/css?family=Open+Sans:400,600,700&display=swap" rel="stylesheet">
        </head>
    
        <body>
        <div id="addon">
            <div class="logo">
                <img src="{logo}">
            </div>
            <h1 class="name">{name}</h1>
            <h2 class="version">{version}</h2>
            <h2 class="description">{description}</h2>
    
            <div class="separator"></div>
    
            <h3 class="gives">This addon has more :</h3>
            <ul>
                {types}
            </ul>
    
            <div class="separator"></div>
    
            <a id="installLink" class="install-link" href="\#">
                <button name="Install">INSTALL</button>
            </a>
            {contact}
        </div>
        <script>
            installLink.href = 'stremio://' + window.location.host + '/manifest.json'
        </script>
        </body>
        </html>
        "#,
        background = background,
        name = manifest.name,
        stylesheet = STYLESHEET, 
        logo = logo,
        version = version,
        description = description,
        types = types,
        contact = contact_html,
    )
}
