
ALWAYS_EXCLUDE=serde
DIRECT_EXTERNAL=testdir,regex,goldenfile,serde_json,cairo-rs,pango,pangocairo

depgraph-directs:
	cargo depgraph --dev-deps --exclude=$(DIRECT_EXTERNAL),$(ALWAYS_EXCLUDE) --manifest-path zvx-simples/Cargo.toml | dot -Tsvg > ~/Desktop/tmp-graph.svg

depgraph-one:
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-simples/Cargo.toml | dot -Tsvg > ~/Desktop/tmp-graph.svg

depgraphs:	/tmp/depgraph-a.dot
	cargo depgraph --dev-deps --exclude=$(DIRECT_EXTERNAL),$(ALWAYS_EXCLUDE) --manifest-path zvx-simples/Cargo.toml > /tmp/depgraph-a.dot && \
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-simples/Cargo.toml > /tmp/depgraph-b.dot && \
	cat /tmp/depgraph-a.dot /tmp/depgraph-b.dot | gvpack -u - | dot -Tsvg > ~/Desktop/tmp-graph.svg

# Tests included, a little sloppier. (Note one can use `cargo clippy --tests` for separate.)
run-clippy-sloppy:
	ls -1 | grep -v Makefile | xargs -I XXX cargo clippy --manifest-path XXX/Cargo.toml --all-targets -- -Wclippy::all -Wclippy::pedantic -Wclippy::nursery -Aclippy::too_many_lines -Aclippy::suboptimal_flops -Aclippy::cast_lossless -Aclippy::tuple_array_conversions -Aclippy::needless_update

run-clippy-no-tests:
	ls -1 | grep -v Makefile | xargs -I XXX cargo clippy --manifest-path XXX/Cargo.toml -- -Wclippy::all -Wclippy::pedantic -Wclippy::nursery -Aclippy::tuple_array_conversions
