FROM ghcr.io/pyo3/maturin

RUN python3 -m venv ./.venv
RUN source ./.venv/bin/activate