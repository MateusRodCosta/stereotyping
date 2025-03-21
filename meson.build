project('stereotyping', 'rust',
          version: '0.5.1',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

gnome = import('gnome')

base_id = 'dev.mateusrodcosta.Stereotyping'

app_id = base_id
if get_option('buildtype') != 'release'
  app_id += '.Devel'
endif

subdir('data')
subdir('src')

gnome.post_install(
     # glib_compile_schemas: true, # no schemas to compile currently
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
