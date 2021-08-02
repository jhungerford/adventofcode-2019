use crate::computer::Computer;

mod computer;

fn main() {
    // Commands:
    // Movement: north, south, east, or west.
    // Take Item: take <name of item>
    // Drop Item: drop <name of item>
    // List Inventory: inv


    let mut computer = Computer::load("input.txt");

    computer.run_interactive();

    /*
    Room Inventory: (o means safe to take, x means not safe to take)

    x Kitchen - molten lava
    x Engineering - photons
    x Navigation - giant electromagnet
    x Science lab - infinite loop
    x Stables - escape pod

    o Observatory - dark matter
    o Warp Drive Maintenance - manifold
    o Passages - jam
    o Sick bay - candy cane
    o Hallway - antenna
    o Storage - hypercube
    o Hot Chocolate Fountain - bowl of rice
    o Corridor - dehydrated water


    Hypercube + manifold is too heavy
    Hypercube + dark matter + bowl of rice is too heavy

    Hypercube + bowl of rice is too heavy

    Need hypercube
    - jam x
    - antenna
    - dehydrated water
    - candy cane
    - dark matter


    hypercube + jam? - no - still too light
    - antenna
    - candy cane
    - dark matter


    hypercube + antenna?
    - dehydrated water
    - candy cane
    - dark matter


    Answer: hypercube + antenna + dehydrated water + candy cane
     */
}
