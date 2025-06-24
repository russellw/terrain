The purpose of this project is to generate random terrain for fictional worlds
and thereby test the performance of Claude Code on an algorithmically hard problem
the program is to be written in Rust
it is to be a commandline program with parameters for things like scale, and percentage of water versus land
there are two output files
a PNG image
and a data dump of intermediate representation, maybe json
the generated landscape should be as realistic as possible
it should have continents and mountains that look like they could've been produced by plate tectonics
and take into account:
convection cells, prevailing wings and rain shadows for rainfall
water and temperature for biome
rivers
but it can assume a flat world, does not need to worry about spherical geometry
