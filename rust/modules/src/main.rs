mod components;

use components::example::do_thing;
use components::test::do_thing_test;
use components::hi::do_thing_hi;
use components::ok::ok;

fn main() {
    do_thing();
    do_thing_test();
    do_thing_hi();
    ok();
}
