# dbus-send \
#   --session \
#   --type=method_call \
#   --dest=org.freedesktop.Notifications \
#   /org/freedesktop/Notifications \
#   org.freedesktop.Notifications.Notify \
#   string:"ExampleApp" \
#   uint32:0 \
#   string:"dialog-information" \
#   string:"Hello World" \
#   string:"This is a test notification." \
#   array:string:"" \
#   dict:string:variant:"" \
#   int32:3000
#
#
gdbus call -e -d org.freedesktop.Notifications \
              -o /org/freedesktop/Notifications \
              -m org.freedesktop.Notifications.Notify \
              "app name" \
              0 \
              "icon" \
              "title" \
              "body" \
              "[]" \
              "{'urgency': <byte 1>}" \
              3000
