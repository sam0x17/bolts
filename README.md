# Rust on Bolts
Bolts is a fast, safe web framework for the Rust language inspired loosely by
Ruby on Rails. Bolts is under active development, with the following planned
features (subject to change):
* fast, powerful, and safe, with sane defaults for everything
* built-in super-fast and safe routing system, including subdomain/domain-based
  routing, automatic parsing of URL parameters, etc
* simple MVC based application layout for basic projects
* templating system for server-side-rendered layouts
* well defined environments (i.e. `development`, `test`, `staging`, `production`)
* pre-configured secure cookies setup
* built-in CSRF protection
* ActiveRecord-esque ORM for SQL-based databases with a migrations system
* ability to deploy entire apps to AWS Lambda + CloudFront
* a CLI allowing for things like `bolts s` to run local dev server
* integrated command/task system
* some sort of frontend framework written in rust / web assembly so we can say
  goodbye to JavaScript
* many other things

To start out, the main objectives are:
1. routing system
2. controller scheme
3. templating / views
4. middleware, session management, etc
