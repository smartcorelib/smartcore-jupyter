# Smartcore with Jupyter

Try latest [Smartcore](https://smartcorelib.org) (development branch) in your browser using Jupyter Notebooks.

## Install and Usage

### Miniconda
Miniconda3 is needed, download latest version from [here](https://docs.conda.io/en/latest/miniconda.html). This setup is based on [evcxr](https://github.com/google/evcxr/).

Once Miniconda3 is installed: 
```
$ conda create --name smartenv
$ conda activate smartenv
(smartenv) $ conda install -y -c conda-forge nb_conda_kernels
(smartenv) $ cargo install --version 0.18.0 evcxr_jupyter --force
(smartenv) $ evcxr_jupyter --install
```

You can find tutorials at:
* [link](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/)
* [link](https://datacrayon.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)

Run `jupyter notebook` while the Conda environment is active:
```
(smartenv) $ jupyter notebook
```

Open notebooks examples in your browser.

Feel free to contribute your notebook.

### uv
As an alternative to Miniconda3, you can use `uv`, a modern Python package and environment manager built in Rust. Learn more and install `uv` [here](https://docs.astral.sh/uv/getting-started/installation/).

Once `uv` is installed:
```bash
cargo install evcxr_jupyter
evcxr_jupyter --install
uv init
uv add notebook
```
You can launch a notebook using:
```bash
uv run jupyter notebook
```
