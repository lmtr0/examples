project('GtkApp', 'vala', 'c')

dependencies = [
    dependency('glib-2.0'),
    dependency('gobject-2.0'),
    dependency('gtk4'),
]

sources = files('main.vala')

executable('gtkapp', sources, dependencies: dependencies)