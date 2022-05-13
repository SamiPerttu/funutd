# FunUTD

## Fun Universal Texture Definitions

FunUTD is a 3-D [procedural texture](https://en.wikipedia.org/wiki/Procedural_texture) library running on the CPU.
This is an alpha version undergoing rapid development and may contain rough edges.

### Features

* Different tiling modes, including tiling of all 3 dimensions
* An endless supply of procedurally generated, self-describing volumetric textures
* Isotropic value noise, isotropic gradient noise and Voronoi bases
* Palette generation with Okhsv and Okhsl color spaces

## Basics

The type returned by texture generators is `Box<dyn Texture>`.
`Texture` is the trait implemented by procedural textures.

The canonical range of texture values is -1...1 in each component.
This applies to the palette component as well.

Some components may slightly exceed the range, while others may come under.
Many unary nodes such as `reflect`, `vreflect` and `saturate` remap
any range back to -1...1.

Data for procedural generation is contained in `Dna` objects.
Generator functions draw whatever data they need from the supplied `Dna` object.
`Dna` objects can be constructed full of random data from a seed value.

Textures can describe themself, that is, print the code that generates them.
This is done using the `get_code` method. Obtained codes can be copied and
pasted around and subjected to further scrutiny.

### Tiling Modes

Tiling modes - whether the texture loops seamlessly for each dimension -
are implemented via a hasher parameter.

Currently implemented tiling modes are:

- `tile_none()` - none of the axes tile.
- `tile_all()` - space is filled with copies of the unit cube and texture
frequencies are rounded to the nearest whole number.
- `tile_xy()` - for each fixed `z`, the `xy` plane is filled with copies
of the unit square, while moving in the `z` dimension produces infinite variation.
Texture frequencies are rounded to the nearest whole number.

To tile a different shape than the unit cube or square:

- `tile_all_in(x, y, z)` - space is filled with copies of `(x, y, z)` sized boxes.
Texture frequencies are rounded to the nearest whole number.
- `tile_xy_in(x, y)` - for each fixed `z`, the `xy` plane is filled with copies
of `(x, y)` sized rectangles, while moving in the `z` dimension produces infinite
variation. Texture frequencies are rounded to the nearest whole number.

## Future

`Dna` objects can be mutated or crossed over to create variations of genotypes
or to optimize a texture for a purpose.

## Examples

```rust
palette(
    Space::HSL,
    0.50937665,
    0.7222409,
    0.0,
    1.0,
    posterize(
        3.8965485,
        0.60872394,
        softmix3(
            5.2831173,
            vnoise(1974317952, 10.774254, tile_all()),
            voronoi(1974803501, 24.273146, tile_all(), 5, 9, 7),
        ),
    ),
)
```

![](example1.png "texture example")

---

```rust
palette(
    Space::HSV,
    0.7194102,
    0.21881655,
    0.0,
    1.0,
    fractal(
        5.3895693,
        7,
        0.5545446,
        2.5686815,
        0.0022501,
        posterize(4.580785, 0.2511709, vnoise_basis(2690581512, tile_all())),
    ),
)
```

![](example2.png "texture example")
