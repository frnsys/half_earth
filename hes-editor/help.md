# The HES Editor

When we designed _[Half-Earth Socialism: The Game](https://play.half.earth)_ we tried to accurately represent the all the different projects, production processes, and so on as best we could. However we knew that given the inherent ambiguity and uncertainty involved people would disagree with some of the decisions we made. Part of the game's vision of being a taste of a democratic planning tool involves not only giving people a means to explore and express *how* we can achieve a better world, but also providing a way to articulate different understandings about *what our options are* in the first place. Hopefully this editor opens up this missing side of the _Half-Earth Socialism_ experience.

We've provided as many options for customization as we could: you can modify model parameters, edit existing cards or create completely new ones, and edit and create events. This should cover the bulk of the game's dynamics, but there are some parts of the game that are more complicated to open up. Dialogue, for example, and other parts that are better off left hardcoded, such as the parliament factions and regions.

With that in mind, a small **caveat emptor**: Unfortunately we can't guarantee complete stability with custom cards as there are probably combinations of parameters and effects that will inevitably break things.

If you *do* want to edit *everything*, you can open up a `.world` file in a text editor and you'll see that it's just JSON. Of course, if you edit worlds by hand then we really can't guarantee any stability. But it could be fun.

## Some further details

The `Planet` groups together parameters relating to:

- Initial state of the planet
- Resource and feedstock availability
- Population growth dynamics
- Relationship b/w region income level and demand
- Initial development level and population of regions

_Annual population growth_ is represented by an equation in the form:

```
β₀ + β₁y + β₂y^2 + β₃y^3
```

where `y` is the current year.

There is one such equation for each level of regional wealth (low income, lower-middle, upper-middle, and high income).

_Processes_ (representing particular production process for energy and calories) and _Industries_ (meant to represent all other major productive activities) have _Byproducts_ and _Resources_.

For processes these are all expressed per-unit of output. For calorie production processes this is per _kilocalorie_ and for energy production processes this is per _kilowatt-hour_.

For industries the unit is a bit more complicated. We use _low-income-per-capita_ (shortened to "LIC"), which is perhaps best explained with an example:

> For simplicity let's say we have only two income levels: LOW and HIGH. We say on average that a HIGH income person consumes 4x as much as a LOW income person. Thus one HIGH income person is equal to four LIC (i.e. they represent the equivalent of four low income people).
>
> So if we have a world with two HIGH income people and three LOW income people, in total we have an equivalent low-income population of `2 * 4 + 3`, i.e. `11 LIC`. We have one industry in our world which consumes 2kWh of fuel per LIC, so its total fuel consumption is `11 * 2 = 22kWh`.


## Saving

`Ctrl+S` will save your changes, but this tool only saves the last world you were working on (which reloads when you return to the site).

**Note**: Your editing session is saved in your browser, and does not carry across browsers (though you can export from one browser and then import into another). This also means that you'll lose your session data if you clear your browser's data!

If you want to work on multiple worlds you can _export_ and then re-_import_ worlds to your local filesystem. This is the best way to ensure that you don't lose any data.


## Sharing

You can share your worlds at the [Steam Workshop](#).

## Reporting bugs

If you encounter any bugs with the editor, the game, or the integration of the two, please [file a report here](https://github.com/frnsys/half_earth/issues).

If you have other questions feel free to [open up a discussion](https://github.com/frnsys/half_earth/discussions).
