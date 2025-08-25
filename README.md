# PickEat
Simple web app for finding and sharing recipes.

## Why ?

Multiple reasons:

- I like to learn by working on projects, and I needed something to start learning rust
- I like to cook, but finding recipes and listing ingredients to buy has always been a chore
- I like simple things, and I don't find the well known recipes websites pleasant to use (ads, useless articles before the recipe, general clutter, etc)

So I decided to make PickEat: a simple web app where you can find recipes, add your own, and list ingredients needed for what you want to cook.

## How ?

### master branch

This repo was originally splitted in 3 distinct parts:

- the backend, a `rust` http server serving a REST API, using a `postgresql` database
- the frontend, a `vuejs` app consumming the API
- the ansible stuff, with playbooks for deploying both backend and frontend from scrach

This is what's currently online currently pick-eat.fr, it should be relatively usable.

## v2 branch

You're now on the v2 branch which is very very WIP. The goal of this branch is to try to make things differenly and see if it's better. On my wishlist:

- try htmx (hence probably get rid of vuejs and make backend spit html)
- way less rigidity when entering recipes, especially ingredients. This is currently very cumberstone and I don't like it.
- try axum instead of actix
- dockerize apps
