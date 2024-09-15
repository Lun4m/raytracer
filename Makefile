.PHONY: run-gpu run-cpu

run-gpu:
	@cargo run --release -p raytracer-gpu
	@# -- ${scene}
	@# open out.ppm

run-cpu:
	cargo run --release -p raytracer-cpu -- ${scene}
	@open out.ppm

help-cpu:
	cargo run -p raytracer-cpu -- --help
