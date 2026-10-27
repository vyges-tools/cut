# vyges-cut

The part of a design a re-synthesis works on, and its way out to and back from ABC:

- **extract** the cut from the timing graph: the logic behind a set of end points, bounded by
  registers, ports and constants, with its primary inputs and outputs;
- **write** it to ABC, either as BLIF (with arrival and required times) or as a network mapped
  onto an ABC library built from the design's Liberty cells;
- **put back** what ABC returns: the old cells removed, the new ones and their nets created under
  unique names, every connection restored at the cut's boundary.

> **Part of Vyges Loom.** A library, not a command: [`vyges-rmp`](https://github.com/vyges-tools/rmp)
> links it for `restructure` and the `resynth` family, and it ships inside the `vyges-physical`
> binary through it.

## Use it

```toml
[dependencies]
vyges-cut = { git = "https://github.com/vyges-tools/cut", branch = "main" }
```

The design and the timer are reached through two seams, so nothing here needs a database to be
tested:

- `design::CutDesign` (and the BLIF reader's `blif::BlifDesign`) for the instances, nets and
  terminals it reads and edits;
- `facts::Facts` for the timer's answers, one entry per graph vertex, gathered before the cut is
  built.

## Same input, same answer

Instances, nets and vertices are visited in a fixed order (ids, then names), never in hash-map
order, and new names come from a counter the caller owns, so the same design and the same ABC
result give the same edited design on every run.

## Build

ABC is linked from the prebuilt archive of [`vyges-abc-lib`](https://github.com/vyges-tools/abc-lib):

```sh
export $(bash ../vyges-tools-abc-lib/scripts/fetch-bundle.sh "$PWD/.abc")   # VYGES_ABC_PREBUILT_DIR
cargo test
```

## License

Apache-2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
