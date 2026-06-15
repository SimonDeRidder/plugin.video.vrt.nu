export PYTHONPATH := $(CURDIR)/resources/lib:$(CURDIR)/tests
PYTHON := python

name = $(shell xmllint --xpath 'string(/addon/@id)' addon.xml)
version = $(shell xmllint --xpath 'string(/addon/@version)' addon.xml)
git_branch = $(shell git rev-parse --abbrev-ref HEAD)
git_hash = $(shell git rev-parse --short HEAD)

ifdef release
	zip_name = $(name)-$(version).zip
else
	zip_name = $(name)-$(version)-$(git_branch)-$(git_hash).zip
endif
zip_dir = $(name)/

languages = $(filter-out en_gb, $(patsubst resources/language/resource.language.%, %, $(wildcard resources/language/*)))

path := /

blue = \e[1;34m
white = \e[1;37m
reset = \e[0;39m

all: check test build
zip: build
test: check test-unit test-service test-run cargo-test

check: check-tox check-pylint check-translations check-clippy

check-tox:
	@printf "$(white)=$(blue) Starting sanity tox test$(reset)\n"
	$(PYTHON) -m tox -q

check-pylint:
	@printf "$(white)=$(blue) Starting sanity pylint test$(reset)\n"
	$(PYTHON) -m pylint resources/lib/ tests/

check-translations:
	@printf "$(white)=$(blue) Starting language test$(reset)\n"
	@-$(foreach lang,$(languages), \
		msgcmp resources/language/resource.language.$(lang)/strings.po resources/language/resource.language.en_gb/strings.po; \
	)

check-clippy:
	@printf "$(white)=$(blue) Starting Clippy test$(reset)\n"
	cargo clippy --all-targets -- -D warnings

# Dev loop: build the .so (debug profile, fast compile) and place it in resources/lib/
dev:
	@printf "$(white)=$(blue) Building _vrtmax (debug) into resources/lib/$(reset)\n"
	@rm -f target/wheels/vrtmax-*.whl
	$(PYTHON) -m maturin build --out target/wheels
	@unzip -j -o target/wheels/vrtmax-*.whl '_vrtmax*.abi3.so' -d resources/lib/
	@touch resources/lib/_vrtmax*.abi3.so  # maturin strips timestamp

native:
	@printf "$(white)=$(blue) Building _vrtmax (release) into resources/lib/$(reset)\n"
	@rm -f target/wheels/vrtmax-*.whl
	$(PYTHON) -m maturin build --release --out target/wheels
	@unzip -j -o target/wheels/vrtmax-*.whl '_vrtmax*.abi3.so' -d resources/lib/
	@touch resources/lib/_vrtmax*.abi3.so  # maturin strips timestamp

kill-proxy:
	-pkill -ef '$(PYTHON) -m proxy'

unit: test-unit
run: test-run

test-unit: clean-py kill-proxy
	@printf "$(white)=$(blue) Starting unit tests$(reset)\n"
	-$(PYTHON) -m proxy --hostname 127.0.0.1 --log-level DEBUG &
	$(PYTHON) -m unittest discover -v
	-pkill -ef '$(PYTHON) -m proxy'

test-service:
	@printf "$(white)=$(blue) Run service$(reset)\n"
	$(PYTHON) resources/lib/service_entry.py

test-run:
	@printf "$(white)=$(blue) Run CLI$(reset)\n"
	$(PYTHON) tests/run.py $(path)

cargo-test:
	@printf "$(white)=$(blue) Cargo test$(reset)\n"
	cargo test

profile:
	@printf "$(white)=$(blue) Profiling $(white)$(path)$(reset)\n"
	$(PYTHON) -m cProfile -o profiling_stats-$(git_branch)-$(git_hash).bin tests/run.py $(path)

build: clean-build native
	@printf "$(white)=$(blue) Building new package$(reset)\n"
	@rm -f ../$(zip_name)
	@rm -rf target/stage && mkdir -p target/stage/$(zip_dir)
	@git archive --format tar --worktree-attributes $(or $(shell git stash create), HEAD) | tar -x -C target/stage/$(zip_dir)
	@cp resources/lib/_vrtmax*.abi3.so target/stage/$(zip_dir)resources/lib/
	@cd target/stage && zip -r $(CURDIR)/../$(zip_name) $(zip_dir) >/dev/null
	@printf "$(white)=$(blue) Successfully wrote package as: $(white)../$(zip_name)$(reset)\n"

clean-py:
	@printf "$(white)=$(blue) Cleaning Python artifacts$(reset)\n"
	find . -name '*.py[cod]' -type f -delete
	find . -name '__pycache__' -type d -delete
	rm -rf .pytest_cache/ .tox/
	rm -f *.log tests/userdata/tokens/*.tkn

clean-build:
	@printf "$(white)=$(blue) Cleaning native artifacts$(reset)\n"
	@rm -rf target/stage
	@rm -f resources/lib/_vrtmax*.abi3.so resources/lib/_vrtmax*.abi3.pyd resources/lib/_vrtmax*.abi3.dylib

clean: clean-py clean-build
