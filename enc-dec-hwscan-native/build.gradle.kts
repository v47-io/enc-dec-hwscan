plugins {
    `rust-base`

    id("fr.stardustenterprises.rust.wrapper")
}

rust {
    release.set(true)

    targets += defaultTarget()
}
