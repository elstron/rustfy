
subdir('ui')

resources = gnome.compile_resources(
    'rustfy',
    'rustfy.gresource.xml',
    dependencies: [blueprints],
    gresource_bundle: true,
    source_dir: meson.current_build_dir(),
    install: true,
    install_dir: pkgdatadir,
)
