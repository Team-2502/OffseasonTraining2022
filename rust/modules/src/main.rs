mod components;

use components::example::do_thing;
use components::test::do_thing_test;

fn main() {
    do_thing();
    do_thing_test();
}
