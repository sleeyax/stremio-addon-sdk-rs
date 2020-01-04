use stremio_core::types::addons::Manifest;


const STYLESHEET: &str = r#"
* {
    box-sizing: border-box;
 }
 
 body,
 html {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%
 }
 
 html {
    background-size: auto 100%;
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat
 }
 
 body {
    display: flex;
    background: rgba(0, 0, 0, 0.60);
    font-family: 'Open Sans', Arial, sans-serif;
    color: white;
 }
 
 h1 {
    font-size: 4.5vh;
    font-weight: 700;
 }
 
 h2 {
    font-size: 2.2vh;
    font-weight: normal;
    font-style: italic;
    opacity: 0.8;
 }
 
 h3 {
    font-size: 2.2vh;
 }
 
 h1,
 h2,
 h3,
 p {
    margin: 0;
    text-shadow: 0 0 1vh rgba(0, 0, 0, 0.15);
 }
 
 p {
    font-size: 1.75vh;
 }
 
 ul {
    font-size: 1.75vh;
    margin: 0;
    margin-top: 1vh;
    padding-left: 3vh;
 }
 
 a {
    color: white
 }
 
 a.install-link {
    text-decoration: none
 }
 
 button {
    border: 0;
    outline: 0;
    color: white;
    background: #8A5AAB;
    padding: 1.2vh 3.5vh;
    margin: auto;
    text-align: center;
    font-family: 'Open Sans', Arial, sans-serif;
    font-size: 2.2vh;
    font-weight: 600;
    cursor: pointer;
    display: block;
    box-shadow: 0 0.5vh 1vh rgba(0, 0, 0, 0.2);
    transition: box-shadow 0.1s ease-in-out;
 }
 
 button:hover {
    box-shadow: none;
 }
 
 button:active {
    box-shadow: 0 0 0 0.5vh white inset;
 }
 
 #addon {
    width: 40vh;
    margin: auto;
 }
 
 .logo {
    height: 14vh;
    width: 14vh;
    margin: auto;
    margin-bottom: 3vh;
 }
 
 .logo img {
    width: 100%;
 }
 
 .name, .version {
    display: inline-block;
    vertical-align: top;
 }
 
 .name {
    line-height: 5vh;
 }
 
 .version {
    position: absolute;
    line-height: 5vh;
    margin-left: 1vh;
    opacity: 0.8;
 }
 
 .contact {
    position: absolute;
    left: 0;
    bottom: 4vh;
    width: 100%;
    text-align: center;
 }
 
 .contact a {
    font-size: 1.4vh;
    font-style: italic;
 }
 
 .separator {
    margin-bottom: 4vh;
 }
"#;

fn get_contact_html(addon_name: &str, email: &str) -> String {
    format!(r#"
        <div class="contact">
            <p>Contact {} creator:</p>
            <a href="mailto:{email}">{email}</a>
        </div>
    "#, addon_name, email = email)
}

pub fn landing_template(manifest: &Manifest) -> String {    
    let background = match &manifest.background {
        Some(background) => background,
        None => "https://dl.strem.io/addon-background.jpg"
    };

    let logo = match &manifest.logo {
        Some(logo) => logo,
        None => "https://dl.strem.io/addon-logo.png"
    };

    let contact_html = match &manifest.contact_email {
        Some(email) => get_contact_html(&manifest.name, &email),
        None => "".into()
    };

    let description = match &manifest.description {
        Some(descr) => descr,
        None => ""
    };

    let version = manifest.version.to_string();
    
    let types = manifest.types.iter()
        .map(|t| {
            // convert first char to upper case
            let mut chars: Vec<char> = t.chars().collect();
            chars[0] = chars[0].to_uppercase().nth(0).unwrap();
            let result: String = chars.into_iter().collect();

            format!("<li>{}{}</li>", result, if t != "series" { "s" } else {""})
        })
        .collect::<Vec<String>>()
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