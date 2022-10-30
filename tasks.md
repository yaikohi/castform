
# Tasks
- [x] think of a new name that's not generic
- [x] learn how to split rust code into smaller modules
- [ ] fetch weather api data with lat/lon
- [ ] learn how to show pixelated pokemon inside of a cli
- [ ] show castform in the style of the weather inside the CLI

# Context
## New name: [Castform](https://bulbapedia.bulbagarden.net/wiki/Castform_(Pok%C3%A9mon)#Pok.C3.A9mon_GO) (from Pokemon)
This pokemon changes shape based on the weather.

# Splitting rust code into modules
Sources used so far:
1. https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
2. https://www.youtube.com/watch?v=969j0qnJGi8


# Weather API
I will be using [openweathermap](https://openweathermap.org/api/) since it allows for plenty of api calls on a single day on a free-to-use basis.
Alternatives seemed expensive and not fitting my goals which is just a simple api call for the current weather at some location.

I don't think people should use this CLI for making more than 10 calls a day and that should already be pushing it.

I used [tomorrow.io's blog](https://www.tomorrow.io/blog/top-8-weather-apis-for-2022/) for making this decision.

# Displaying Pokemon inside of the terminal
...WIP

Possible sources:
[pokemon-icat](https://github.com/ph04/pokemon-icat)
[pokemon-terminal-art](https://github.com/shinya/pokemon-terminal-art)
