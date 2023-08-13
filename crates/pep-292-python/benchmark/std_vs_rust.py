ITERATIONS = 2_000_000

from pathlib import Path

lorem = Path(__file__).parent / "lorem_ipsum.txt"
with open(lorem) as f:
    template = f.read()

subs = {
    "imperdiet": "first replacement",
    "fringilla": "second replacement",
    "libero": "third replacement",
    "felis": "forth replacement",
    "venenatis": "another one",
}

from timeit import Timer

import string
py = Timer('template.substitute(**subs)', globals={
    "template": string.Template(template),
    "subs": subs,
})
py_time = py.timeit(ITERATIONS)
print(f"Python took {py_time}s")

import pep_292_python
rust = Timer('template.substitute(**subs)', globals={
    "template": pep_292_python.Template(template),
    "subs": subs,
})
rust_time = rust.timeit(ITERATIONS)
print(f"Rust took {rust_time}s")

print(f'Python vs Rust: ±{rust_time/py_time * 100:.2f}%')
