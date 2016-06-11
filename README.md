Entity Rust
==========

This project is just starting to form into something functional. Look in the tests directory for examples on how to use it.

# General Idea
![General Idea](https://cdn.meme.am/instances/500x/68816513.jpg)

The project is a DSL and framework for defining components and systems in the traditional ECS style. The systems are event-driven and treat components as resources that are subscribed to. Because the framework is aware of the components that are used by each event handler it can (in theory) schedule events to be handled efficiently in parallel. The DSL and Rust together enforce concurrency- and typesafety of the systems. It should be highly performant (i.e. I wrote this with the goal of implementing massive simulation games like Simcity or Dwarf Fortress). But tests should prove whether I actually achieved this.

