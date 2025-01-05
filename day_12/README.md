# Day 12

Parse stdin, so just redirect the input file with shell:

```sh
cargo run < input.txt
```

Accepts -d for colorized map output debugging (`cargo run -- -d < input.txt`)

![](color_output2.png)

Accepts -v for the verbose list of all regions detected

![](color_output1.png)

Colors will use all the ANSI colors except black, to display on my black background terminal.

## Algo

Pretty simple (it's only day 12). From the initial input matrix of letters, create a second matrix
of individual regions by using a flood-filling algo on each region for matrix cells not yet processes.

The floodfill function fills a contiguous row, then recursively calls itself for pixels above and under.

No trick except I was stuck on a off-by-one bug on the floodill algo that overshoot the min and max X of
the row by 1, causing a particular case of connecting two different regions by their corners.

* Part 1 then just re-parses the full region matrix, and for each cell add 1 to its corresponding region
area counter (All regions are maintained in a preallocated Vec, no need for hashmap). Perimeter is increased
each time an adjacent cell is not on the region.

* Part 2 modifies the part 1 function (no need to create a second function, both can be done on the same
loop). 
    * Side counting is done by ignoring adjacent "same side" perimeter; the initial idea was to count 1
    for the first time a perimeter orientation is encountered (top, left, right, bottom) then ignore when
    the next cell would give the same perimeter limit. 
    * Without "following the sides" like a maze, but by
    only iterating on all X and Y linearly, the equivalent is to simply count the corners of the regions.
    * In practice, for each perimeter test, we test if a previous cell on this side direction (perpendicular
    to the delta (x,y) use to compare the adjacent cell) is also in the region and is also different from
    its own adjacent cell. If yes, it's a continuation of the side, don't count anything. If no, something
    changed and we are at the corner or start of a new side: count 1.





