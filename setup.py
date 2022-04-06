from setuptools import dist 
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import  setup
from setuptools_rust import Binding, RustExtension
# We are going to use setup tp define parameters, and use find_packages to exclude tests.

# Load the README.md contents for long_description
with open("README.md", "r") as fh:
    long_description = fh.read()

setup(
    name="flitton-fib-rs",
    version="0.1",
    rust_extensions = [
        RustExtension(
            ".flitton_fib_rs.flitton_fib_rs",
            path="Cargo.toml",
            binding=Binding.PyO3
        )
    ],
    packages=["flitton_fib_rs"],
    author="Weiren",
    author_email="miblue119@gmail.com",
    description="Calculates a Fibonacci number",
    long_description=long_description,
    url="https://github.com/MIBlue119/flitton-fib-rs",
    install_requires=[],
    classifiers=[
        "Development Status :: 4 - Beta",
        "Programming Language :: Python ::3",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    entry_points={
        'console_scripts':[
            'fib-number=flitton_fib_rs.fib_number_command:fib_number_command'
        ]
    },
    python_requires=">=3",
    test_require=['pytest'],
    extra_require={
        'server': ["Flask>=1.0.0"]
    }
)