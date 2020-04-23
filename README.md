<h1 align="center">
  <img width="150" src="https://i.imgur.com/QaYvRVJ.png" />
  <p>Stremio Addon SDK</p>
</h1>

<h4 align="center">Rust version of the <a href="https://github.com/Stremio/stremio-addon-sdk" target="_blak">stremio-addon-sdk</a> using <a href="https://github.com/Stremio/stremio-core" target="_blank">stremio-core</a></h4>

This is an **unofficial** SDK for building [stremio](https://www.stremio.com/) addons. If you're totally new to addon building, it's recommended to start reading the [offical stremio addon SDK docs](https://github.com/Stremio/stremio-addon-sdk/tree/master/docs) to get a basic understanding of how addons work. This SDK is meant to step up your game if you want to make use of Rust's powerfull type system and performance.

## Getting started
```rust
use stremio_addon_sdk::builder::Builder;
use stremio_addon_sdk::server::{serve_http, ServerOptions};

#[tokio::main]
async fn main() {
    // create manifest file using stremio-core's Manifest struct
    let manifest = Manifest {
        // ...
    };

    // build addon interface
    let interface = Builder::new(manifest)
        // function as parameter
        .define_catalog_handler(handle_catalog)
        .define_stream_handler(handle_stream)
        // closure as parameter
        .define_meta_handler(|resource: &ResourceRef| -> EnvFuture<ResourceResponse> {
            let response = ResourceResponse::Metas { metas: vec![] };
            return Box::new(future::ok(response));
        })
        .build();

    // run HTTP server with default settings
    serve_http(interface, ServerOptions::default());
}
```

See the [example-addon](example-addon) for more details or take a look at the [documentation](https://github.com/sleeyax/stremio-addon-sdk/wiki/Addon).

## FAQ
* How do I host my addon?
    * [now.sh](https://zeit.co/home) deployments are [supported](https://github.com/sleeyax/stremio-addon-sdk/wiki/Now).
    * [heroku](https://dashboard.heroku.com) deployments are [supported](https://github.com/sleeyax/stremio-addon-sdk/wiki/Heroku).
* Why would I use this over the official SDK?
    * You tell me. Types, speed & stremio-core are my reasons ;)
* How do I publish my addon?
    * Use [this website](https://stremio.github.io/stremio-publish-addon/) to publish your addon to the public Stremio addon collection.

## Documentation
Documentation can be found [here](https://github.com/sleeyax/stremio-addon-sdk/wiki).