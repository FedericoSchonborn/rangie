# Rangie

An alternative to `std::ops::Range` that implements `IntoIterator` instead of being an `Iterator`
itself, allowing it to also implement `Copy`.

Turns out this requires a Nightly toolchain so I'm not even bothering. Feel free to fork this repo
and make something useful out of it.
