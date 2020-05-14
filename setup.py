from setuptools import setup

def build_native(spec):
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='task._native',
        dylib=lambda: build.find_dylib('task', in_path='target/release'),
        header_filename=lambda: build.find_header('task.h', in_path='target'),
        rtld_flags=['NOW', 'NODELETE']
    )

setup(
    name='task',
    version='0.0.1',
    packages=['task'],
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    milksnake_tasks=[
        build_native
    ]
)
