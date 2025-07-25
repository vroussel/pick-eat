# Important note

## v2 branch

You're on the v2 branch which is very very WIP. The goal of this branch is to try to make things differenly and see if it's better. On my wishlist:
 - try htmx (hence probably get rid of vuejs and make backend spit html)
 - way less rigidity when entering recipes, especially ingredients. This is currently very cumberstone and I don't like it.
 - try axum instead of actix
 - dockerize apps

Simple web app for finding and sharing recipes.

Available at `pick-eat.fr` (only in french at the moment).

# PickEat

## Why ?

Multiple reasons:
 - I like to learn by working on projects, and I needed something to start learning rust
 - I like to cook, but finding recipes and listing ingredients to buy has always been a chore
 - I like simple things, and I don't find the well known recipes websites pleasant to use (ads, useless articles before the recipe, general clutter, etc)

So I decided to make PickEat: a simple web app where you can find recipes, add your own, and list ingredients needed for what you want to cook.

## How ?

### Summary

This repo is splitted in 3 distinct parts:
 - the backend, a `rust` http server serving a REST API, using a `postgresql` database
 - the frontend, a `vuejs` app consumming the API
 - the ansible stuff, with playbooks for deploying both backend and frontend from scrach

## Current state

The app should be usable in its current state, and is available at `pick-eat.fr`.
However, it's far from perfect. Don't hesitate to open an issue or to make a pull request if you want to.
