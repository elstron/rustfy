blueprint_files = [
    'revealers/apps_revealer.blp',
    'revealers/calculator_revealer.blp',
    'revealers/file_revealer.blp',
    'revealers/web_search_revealer.blp',
    'main_window.blp',
]

blueprint_targets = []
foreach blp : blueprint_files
    blueprint_targets += blp.split('/')[-1].replace('.blp', '.ui')
endforeach

blueprints = custom_target(
    'blueprints',
    input: files(blueprint_files),
    output: blueprint_targets,
    command: [
        find_program('blueprint-compiler'),
        'batch-compile',
        '@OUTDIR@',
        '@CURRENT_SOURCE_DIR@',
        '@INPUT@',
    ],
)
