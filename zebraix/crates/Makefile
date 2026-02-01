# Copyright 2026 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Some (serde) are just used almost as a system library.  The direct
# externals actually include some indirects that are noisy if not excluded.
ALWAYS_EXCLUDE=serde
DIRECT_EXTERNAL=testdir,regex,goldenfile,serde_json,cairo-rs,pango,pangocairo,approx,libm,serde_core

depgraph-directs:
	cargo depgraph --dev-deps --exclude=$(DIRECT_EXTERNAL),$(ALWAYS_EXCLUDE) --manifest-path zvx-simples/Cargo.toml | dot -Tsvg > ~/Desktop/tmp-graph.svg

depgraph-experimental:
	cargo depgraph --dev-deps --exclude=$(DIRECT_EXTERNAL),$(ALWAYS_EXCLUDE) --manifest-path zvx-simples/Cargo.toml | dot -Tsvg > ~/Desktop/tmp-graph.svg

# Includes the explicit excludes.
depgraph-one:
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-simples/Cargo.toml | dot -Tsvg > ~/Desktop/tmp-graph.svg

depgraph-twosie:
	cargo depgraph --dev-deps --exclude=$(DIRECT_EXTERNAL),$(ALWAYS_EXCLUDE) --manifest-path zvx-simples/Cargo.toml > /tmp/depgraph-a.dot && \
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-simples/Cargo.toml > /tmp/depgraph-b.dot && \
	cat /tmp/depgraph-a.dot /tmp/depgraph-b.dot | gvpack - | dot -Tsvg > ~/Desktop/tmp-graph.svg

depgraph-twosie-experimental:
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-simples/Cargo.toml > /tmp/depgraph-a.dot && \
	cargo depgraph --dev-deps --dedup-transitive-deps --include=zvx-base,zvx-cairo,zvx-curves,zvx-docagram,zvx-drawable,zvx-golden,zvx-simples,$(DIRECT_EXTERNAL) --manifest-path zvx-cairo/Cargo.toml > /tmp/depgraph-b.dot && \
	dot -Tdot /tmp/depgraph-a.dot > /tmp/depgraph-a-better.dot &&\
	dot -Tdot /tmp/depgraph-b.dot > /tmp/depgraph-b-better.dot &&\
	gvpack -n /tmp/depgraph-a-better.dot /tmp/depgraph-b-better.dot | dot -Tsvg > ~/Desktop/tmp-graph.svg

# Tests included, a little sloppier. (Note one can use `cargo clippy --tests` for separate.)
run-clippy-sloppy:
	ls -1 | grep -v Makefile | xargs -I XXX cargo clippy --manifest-path XXX/Cargo.toml --all-targets -- -Wclippy::all -Wclippy::pedantic -Wclippy::nursery -Aclippy::too_many_lines -Aclippy::suboptimal_flops -Aclippy::cast_lossless -Aclippy::tuple_array_conversions -Aclippy::needless_update

run-clippy-no-tests:
	ls -1 | grep -v Makefile | xargs -I XXX cargo clippy --manifest-path XXX/Cargo.toml -- -Wclippy::all -Wclippy::pedantic -Wclippy::nursery -Aclippy::tuple_array_conversions
