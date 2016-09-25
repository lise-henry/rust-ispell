// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate ispell;
use ispell::SpellLauncher;


fn main() {
    let mut checker = SpellLauncher::new()
        .command("aspell")
        .language("en")
        .launch()
        .unwrap();
    checker.check("test of a msitake");
    checker.check("test without mistake (?)");
    checker.check("Another test wiht a mistake");
}
