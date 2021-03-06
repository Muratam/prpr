use crate::const_params::*;
use actix_web::*;
// use prpr::*;

#[derive(Debug)]
pub struct WebPageConfig {
  pub description: String,
  pub title: String,
  pub allow_publish: bool,
}

impl Default for WebPageConfig {
  fn default() -> Self {
    Self {
      description: String::from(""),
      title: String::from(""),
      allow_publish: true,
    }
  }
}

pub fn respond_html(config: &WebPageConfig) -> impl Responder {
  let robots = if config.allow_publish {
    "index,follow"
  } else {
    "noindex,nofollow"
  };
  HttpResponse::Ok()
    .content_type("text/html")
    .body(format!(
r###"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="description" content="{description}">
    <meta name="format-detection" content="email=no,telephone=no,address=no">
    <meta name="robots" content="{robots}">
    <link rel="icon" href="{root}/favicon.ico">
    <title>{title}</title>
  </head>
  <body>
    <noscript> We're sorry but doesn't work properly without JavaScript enabled. Please enable it to continue. </noscript>
    <script type="module"> import init from "./{root}/js/prpr.js"; init(); </script>
  </body>
</html>
"###,
  root=format!("./{}/", RESOURCE_ROOT_DIR_NAME),
  description=config.description,
  title=config.title,
  robots=robots,
))
  // TODO: OGP
}
