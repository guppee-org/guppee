# Fishy Game about fishes, using WebGl, Unity, etc.

## Proof of Concept
- [ ]: Fishes have hunger.
- [ ]: User feeds the fish.
- [ ]: Fish gets less hungry when fed.
- [ ]: When fish runs out of hunger, dies
- [ ]: Fish can be sacrificed for.. Lootboxes?
- [ ]: Data is persisted. When the user returns after a long period, fish lost hunger / died.

Covers most of CRUD just to prove we know how to do it.

### Goals & Challenges

#### Web Client
- [ ]: Web page with `<form>`-ish data collection.
- [ ]: Parse response from the server (or just use the url?)

#### Database
- [ ]: Which database?
- [ ]: Until that is decided, sqlite or a HashMap works fine.

#### Game Event Loop
Need to understand how to database calls / http requests happen in webgl / unity.

#### Game Render
- [ ]: Display fish to user (static image of fish, or just their name).
- [ ]: Show how hungry the fish is.
- [ ]: Communicate state changes to the database.
