/*
Next, we’ll extract the hosting module to its own file.

The process is a bit different because hosting is a child module of front_of_house, not of the root module.
We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house.
*/
pub mod hosting;