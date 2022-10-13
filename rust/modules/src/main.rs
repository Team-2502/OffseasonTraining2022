mod components;

use components::example::do_thing;
use components::hi::do_thing_hi;

fn main() {
    do_thing();
    do_thing_hi();
}
