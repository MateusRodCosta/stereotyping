desktop_file = configure_file(
          input: '@0@.desktop.in'.format(base_id),
         output: '@0@.desktop'.format(app_id),
  configuration: { 'icon' : app_id },
        install: true,
    install_dir: get_option('datadir') / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_file = configure_file(
          input: '@0@.metainfo.xml.in'.format(base_id),
         output: '@0@.metainfo.xml'.format(app_id),
  configuration: { 'app-id' : app_id },
        install: true,
    install_dir: get_option('datadir') / 'metainfo'
)

appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test('Validate appstream file', appstreamcli,
     args: ['validate', '--no-net', '--explain', appstream_file])

subdir('icon')